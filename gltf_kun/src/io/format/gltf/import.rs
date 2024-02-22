use std::collections::HashMap;

use glam::Quat;
use gltf::json::{accessor::ComponentType, validation::Checked};
use thiserror::Error;
use tracing::{debug, error, warn};

use crate::{
    graph::{
        gltf::{
            document::GltfDocument,
            image::Image,
            material::AlphaMode,
            texture_info::{MagFilter, MinFilter, TextureInfo, Wrap},
        },
        Graph, GraphNodeWeight,
    },
    io::resolver::Resolver,
};

use super::GltfFormat;

#[derive(Debug, Error)]
pub enum GltfImportError {}

pub async fn import(
    graph: &mut Graph,
    format: &mut GltfFormat,
    resolver: &mut Option<impl Resolver>,
) -> Result<GltfDocument, GltfImportError> {
    let doc = GltfDocument::new(graph);

    // Create buffers
    let buffers = vec![doc.create_buffer(graph); format.json.buffers.len()];
    let mut buffer_data = HashMap::new();

    for (i, b) in format.json.buffers.iter_mut().enumerate() {
        let mut buffer = buffers[i];
        let weight = buffer.get_mut(graph);

        weight.name = b.name.clone();
        weight.extras = b.extras.clone();

        weight.uri = b.uri.clone();

        if resolver.is_none() && format.resources.len() == 1 {
            let key = format
                .resources
                .iter_mut()
                .find(|_| true)
                .map(|(k, _)| k.clone());

            if let Some(key) = key {
                let data = format.resources.remove(&key).unwrap();
                buffer_data.insert(i, data);
            } else {
                warn!("No resources provided");
            }
        } else if let Some(uri) = weight.uri.as_ref() {
            if let Some(resolver) = resolver {
                if let Ok(data) = resolver.resolve(uri).await {
                    debug!("Resolved buffer: {} ({} bytes)", uri, data.len());
                    buffer_data.insert(i, data);
                } else {
                    warn!("Failed to resolve URI: {}", uri);
                }
            } else {
                warn!("No resolver provided");
            }
        }
    }

    // Create accessors
    let accessors = format
        .json
        .accessors
        .iter_mut()
        .map(|a| {
            let mut accessor = doc.create_accessor(graph);

            let weight = accessor.get_mut(graph);

            weight.name = a.name.clone();
            weight.extras = a.extras.clone();

            weight.normalized = a.normalized;
            weight.component_type = match a.component_type {
                Checked::Valid(component_type) => component_type.0,
                Checked::Invalid => {
                    error!("Invalid accessor component type: {:?}", a.component_type);
                    ComponentType::U8
                }
            };
            weight.element_type = match a.type_ {
                Checked::Valid(ty) => ty,
                Checked::Invalid => {
                    error!("Invalid accessor type: {:?}", a.type_);
                    gltf::json::accessor::Type::Scalar
                }
            };

            let buffer_view_idx = match a.buffer_view.map(|v| v.value()) {
                Some(idx) => idx,
                None => {
                    warn!("Accessor has no buffer view");
                    return accessor;
                }
            };

            let buffer_view = &format.json.buffer_views[buffer_view_idx];
            let buffer_idx = buffer_view.buffer.value();

            let data = match buffer_data.get(&buffer_idx) {
                Some(data) => data,
                None => {
                    warn!("Buffer has no data");
                    return accessor;
                }
            };

            let view = read_view(buffer_view, data);

            if let Some(_sparse) = &a.sparse {
                error!("Sparse accessors are not supported");
            }

            let accessor_start = a.byte_offset.map(|o| o.0 as usize).unwrap_or_default();
            let item_size = a.component_type.unwrap().0.size() * a.type_.unwrap().multiplicity();
            let accessor_end = accessor_start + (a.count.0 as usize * item_size);

            weight.data = view[accessor_start..accessor_end].to_vec();

            accessor
        })
        .collect::<Vec<_>>();

    // Create images
    let images = vec![doc.create_image(graph); format.json.images.len()];

    for (i, img) in format.json.images.iter_mut().enumerate() {
        let mut image = images[i];

        let weight = image.get_mut(graph);
        weight.name = img.name.clone();
        weight.extras = img.extras.clone();
        weight.mime_type = img.mime_type.clone().map(|m| m.0);

        if let Some(uri) = img.uri.as_ref() {
            weight.uri = img.uri.clone();

            if let Some(resolver) = resolver {
                if let Ok(data) = resolver.resolve(uri).await {
                    debug!("Resolved image: {} ({} bytes)", uri, data.len());
                    weight.data = data;
                } else {
                    warn!("Failed to resolve URI: {}", uri);
                }
            } else {
                warn!("No resolver provided");
            }
        } else if let Some(index) = img.buffer_view {
            let view = &format.json.buffer_views[index.value()];

            let buffer_idx = view.buffer.value();
            let buf_data = match buffer_data.get(&buffer_idx) {
                Some(data) => data.as_slice(),
                None => {
                    warn!("Buffer has no data");
                    &[]
                }
            };

            weight.data = read_view(view, buf_data).to_vec();

            let buffer = buffers[buffer_idx];
            image.set_buffer(graph, Some(buffer));
        }
    }

    // Create materials
    let json_textures = &format.json.textures;
    let json_samplers = &format.json.samplers;

    let materials = format
        .json
        .materials
        .iter_mut()
        .map(|m| {
            let mut material = doc.create_material(graph);
            let weight = material.get_mut(graph);

            weight.name = m.name.clone();
            weight.extras = m.extras.clone();

            weight.alpha_mode = match m.alpha_mode.unwrap() {
                gltf::json::material::AlphaMode::Opaque => AlphaMode::Opaque,
                gltf::json::material::AlphaMode::Mask => AlphaMode::Mask,
                gltf::json::material::AlphaMode::Blend => AlphaMode::Blend,
            };

            weight.alpha_cutoff = m.alpha_cutoff.map(|c| c.0).unwrap_or_default();
            weight.double_sided = m.double_sided;
            weight.base_color_factor = m.pbr_metallic_roughness.base_color_factor.0;
            weight.emissive_factor = m.emissive_factor.0;
            weight.metallic_factor = m.pbr_metallic_roughness.metallic_factor.0;
            weight.roughness_factor = m.pbr_metallic_roughness.roughness_factor.0;

            let base_color_texture_info =
                m.pbr_metallic_roughness
                    .base_color_texture
                    .as_ref()
                    .map(|t| {
                        import_texture_info(
                            t.index.value(),
                            t.tex_coord as usize,
                            graph,
                            json_textures,
                            json_samplers,
                            &images,
                        )
                    });
            let emissive_texture_info = m.emissive_texture.as_ref().map(|t| {
                import_texture_info(
                    t.index.value(),
                    t.tex_coord as usize,
                    graph,
                    json_textures,
                    json_samplers,
                    &images,
                )
            });
            let metallic_roughness_texture_info = m
                .pbr_metallic_roughness
                .metallic_roughness_texture
                .as_ref()
                .map(|t| {
                    import_texture_info(
                        t.index.value(),
                        t.tex_coord as usize,
                        graph,
                        json_textures,
                        json_samplers,
                        &images,
                    )
                });
            let normal_texture_info = m.normal_texture.as_ref().map(|t| {
                let weight = material.get_mut(graph);
                weight.normal_scale = t.scale;
                import_texture_info(
                    t.index.value(),
                    t.tex_coord as usize,
                    graph,
                    json_textures,
                    json_samplers,
                    &images,
                )
            });
            let occlusion_texture_info = m.occlusion_texture.as_ref().map(|t| {
                let weight = material.get_mut(graph);
                weight.occlusion_strength = t.strength.0;
                import_texture_info(
                    t.index.value(),
                    t.tex_coord as usize,
                    graph,
                    json_textures,
                    json_samplers,
                    &images,
                )
            });

            material.set_base_color_texture_info(graph, base_color_texture_info);
            material.set_emissive_texture_info(graph, emissive_texture_info);
            material.set_metallic_roughness_texture_info(graph, metallic_roughness_texture_info);
            material.set_normal_texture_info(graph, normal_texture_info);
            material.set_occlusion_texture_info(graph, occlusion_texture_info);

            material
        })
        .collect::<Vec<_>>();

    // Create meshes
    let meshes = format
        .json
        .meshes
        .iter_mut()
        .map(|m| {
            let mut mesh = doc.create_mesh(graph);
            let weight = mesh.get_mut(graph);

            weight.name = m.name.clone();
            weight.extras = m.extras.clone();

            m.primitives.iter_mut().for_each(|p| {
                let mut primitive = mesh.create_primitive(graph);
                let p_weight = primitive.get_mut(graph);

                p_weight.extras = p.extras.clone();
                p_weight.mode = match p.mode {
                    Checked::Valid(mode) => mode,
                    Checked::Invalid => gltf::mesh::Mode::Triangles,
                };

                let material = match p.material {
                    Some(index) => materials.get(index.value()).copied(),
                    None => None,
                };
                primitive.set_material(graph, material);

                if let Some(index) = p.indices {
                    if let Some(accessor) = accessors.get(index.value()) {
                        primitive.set_indices(graph, Some(*accessor));
                    }
                }

                p.attributes.iter().for_each(|(k, v)| {
                    if let Some(accessor) = accessors.get(v.value()) {
                        let semantic = match k {
                            Checked::Valid(semantic) => semantic,
                            Checked::Invalid => {
                                warn!("Invalid attribute semantic: {:?}", k);
                                return;
                            }
                        };

                        primitive.set_attribute(graph, semantic, Some(*accessor));
                    }
                });
            });

            mesh
        })
        .collect::<Vec<_>>();

    // Create nodes
    let nodes = format
        .json
        .nodes
        .iter_mut()
        .map(|n| {
            let mut node = doc.create_node(graph);
            let weight = node.get_mut(graph);

            weight.name = n.name.clone();
            weight.extras = n.extras.clone();

            weight.translation = n.translation.map(|t| t.into()).unwrap_or_default();
            weight.rotation = n
                .rotation
                .map(|r| Quat::from_slice(&r.0))
                .unwrap_or(Quat::IDENTITY);
            weight.scale = n.scale.map(|s| s.into()).unwrap_or(glam::Vec3::ONE);

            if let Some(index) = n.mesh {
                if let Some(mesh) = meshes.get(index.value()) {
                    node.set_mesh(graph, Some(*mesh));
                }
            }

            node
        })
        .collect::<Vec<_>>();

    // Parent nodes
    format
        .json
        .nodes
        .iter()
        .enumerate()
        .filter_map(|(i, n)| n.children.as_ref().map(|c| (i, c)))
        .for_each(|(i, children)| {
            let node = &nodes[i];

            children.iter().for_each(|idx| {
                let child = &nodes[idx.value()];
                node.add_child(graph, child);
            });
        });

    // TODO: Create skins

    // Create scenes
    let scenes = format
        .json
        .scenes
        .iter_mut()
        .map(|s| {
            let mut scene = doc.create_scene(graph);
            let weight = scene.get_mut(graph);

            weight.name = s.name.clone();
            weight.extras = s.extras.clone();

            s.nodes.iter().for_each(|idx| {
                if let Some(node) = nodes.get(idx.value()) {
                    scene.add_node(graph, *node);
                }
            });

            scene
        })
        .collect::<Vec<_>>();

    // Default scene
    if let Some(index) = format.json.scene {
        if let Some(scene) = scenes.get(index.value()) {
            doc.set_default_scene(graph, Some(*scene));
        }
    }

    // TODO: Create animations

    Ok(doc)
}

fn read_view<'a>(view: &gltf::json::buffer::View, buffer_data: &'a [u8]) -> &'a [u8] {
    let start = view.byte_offset.map(|o| o.0 as usize).unwrap_or_default();
    let end = start + view.byte_length.0 as usize;
    &buffer_data[start..end]
}

fn import_texture_info(
    index: usize,
    tex_coord: usize,
    graph: &mut Graph,
    textures: &[gltf::json::Texture],
    samplers: &[gltf::json::texture::Sampler],
    images: &[Image],
) -> TextureInfo {
    let mut texture_info = TextureInfo::new(graph);

    let texture = &textures[index];
    let image_idx = texture.source.value();
    let image = images[image_idx];
    texture_info.set_image(graph, Some(image));

    let weight = texture_info.get_mut(graph);
    weight.tex_coord = tex_coord;

    if let Some(sampler_idx) = texture.sampler {
        let sampler_idx = sampler_idx.value();
        let sampler = &samplers[sampler_idx];

        weight.mag_filter = sampler.mag_filter.map(|f| match f.unwrap() {
            gltf::json::texture::MagFilter::Nearest => MagFilter::Nearest,
            gltf::json::texture::MagFilter::Linear => MagFilter::Linear,
        });

        weight.min_filter = sampler.min_filter.map(|f| match f.unwrap() {
            gltf::json::texture::MinFilter::Nearest => MinFilter::Nearest,
            gltf::json::texture::MinFilter::Linear => MinFilter::Linear,
            gltf::json::texture::MinFilter::NearestMipmapNearest => MinFilter::NearestMipmapNearest,
            gltf::json::texture::MinFilter::LinearMipmapNearest => MinFilter::LinearMipmapNearest,
            gltf::json::texture::MinFilter::NearestMipmapLinear => MinFilter::NearestMipmapLinear,
            gltf::json::texture::MinFilter::LinearMipmapLinear => MinFilter::LinearMipmapLinear,
        });

        weight.wrap_s = match sampler.wrap_s.unwrap() {
            gltf::json::texture::WrappingMode::ClampToEdge => Some(Wrap::ClampToEdge),
            gltf::json::texture::WrappingMode::MirroredRepeat => Some(Wrap::MirroredRepeat),
            gltf::json::texture::WrappingMode::Repeat => Some(Wrap::Repeat),
        };

        weight.wrap_t = match sampler.wrap_t.unwrap() {
            gltf::json::texture::WrappingMode::ClampToEdge => Some(Wrap::ClampToEdge),
            gltf::json::texture::WrappingMode::MirroredRepeat => Some(Wrap::MirroredRepeat),
            gltf::json::texture::WrappingMode::Repeat => Some(Wrap::Repeat),
        };
    }

    texture_info
}

#[cfg(test)]
mod tests {
    use gltf::json::{self, texture::Info, validation::USize64, Index};
    use tracing_test::traced_test;

    use crate::io::resolver::file_resolver::FileResolver;

    use super::*;

    #[tokio::test]
    #[traced_test]
    async fn test_import() {
        let mut json = json::Root::default();

        json.buffers.push(json::buffer::Buffer {
            name: Some("MyBuffer".to_string()),
            byte_length: USize64(0),
            uri: None,
            extensions: None,
            extras: None,
        });

        json.buffer_views.push(json::buffer::View {
            name: Some("MyBufferView".to_string()),
            buffer: Index::new(0),
            byte_length: USize64(0),
            byte_offset: None,
            byte_stride: None,
            target: None,
            extensions: None,
            extras: None,
        });

        json.accessors.push(json::accessor::Accessor {
            name: Some("MyAccessor".to_string()),
            buffer_view: Some(Index::new(0)),
            byte_offset: None,
            component_type: Checked::Valid(json::accessor::GenericComponentType(
                json::accessor::ComponentType::U8,
            )),
            count: USize64(0),
            extensions: None,
            extras: None,
            max: None,
            min: None,
            normalized: false,
            sparse: None,
            type_: Checked::Valid(json::accessor::Type::Scalar),
        });

        json.images.push(json::image::Image {
            name: Some("MyImage".to_string()),
            uri: None,
            buffer_view: Some(Index::new(0)),
            mime_type: None,
            extensions: None,
            extras: None,
        });

        json.textures.push(json::texture::Texture {
            name: Some("MyTexture".to_string()),
            extensions: None,
            extras: None,
            sampler: None,
            source: Index::new(0),
        });

        json.materials.push(json::material::Material {
            name: Some("MyMaterial".to_string()),
            pbr_metallic_roughness: json::material::PbrMetallicRoughness {
                base_color_texture: Some(Info {
                    index: Index::new(0),
                    tex_coord: 0,
                    extensions: None,
                    extras: None,
                }),
                ..Default::default()
            },
            ..Default::default()
        });

        json.meshes.push(json::mesh::Mesh {
            name: Some("MyMesh".to_string()),
            primitives: vec![json::mesh::Primitive {
                attributes: Default::default(),
                extensions: None,
                extras: None,
                indices: None,
                material: Some(Index::new(0)),
                mode: Checked::Valid(json::mesh::Mode::Triangles),
                targets: None,
            }],
            weights: None,
            extensions: None,
            extras: None,
        });

        json.nodes.push(json::scene::Node {
            name: Some("MyNode".to_string()),
            mesh: Some(Index::new(0)),
            camera: None,
            children: None,
            skin: None,
            matrix: None,
            rotation: None,
            scale: None,
            translation: None,
            weights: None,
            extensions: None,
            extras: None,
        });

        json.scenes.push(json::scene::Scene {
            name: Some("MyScene".to_string()),
            nodes: vec![Index::new(0)],
            extras: None,
            extensions: None,
        });

        json.scene = Some(Index::new(0));

        let mut format = GltfFormat {
            json,
            ..Default::default()
        };

        let mut graph = Graph::new();

        let doc = import(&mut graph, &mut format, &mut None::<FileResolver>)
            .await
            .unwrap();

        assert_eq!(doc.scenes(&graph).len(), 1);
        assert_eq!(doc.default_scene(&graph), Some(doc.scenes(&graph)[0]));
        assert_eq!(doc.nodes(&graph).len(), 1);
        assert_eq!(doc.images(&graph).len(), 1);
        assert_eq!(doc.materials(&graph).len(), 1);
        assert_eq!(doc.meshes(&graph).len(), 1);
        assert_eq!(doc.buffers(&graph).len(), 1);
        assert_eq!(doc.accessors(&graph).len(), 1);
    }
}
