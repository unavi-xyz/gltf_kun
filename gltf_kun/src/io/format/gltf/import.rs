use glam::Quat;
use gltf::json::validation::Checked;
use thiserror::Error;
use tracing::{debug, error, warn};

use crate::{
    graph::{
        gltf::{
            animation::AnimationSampler, document::GltfDocument, image::Image,
            texture_info::TextureInfo, Accessor,
        },
        Graph, GraphNodeWeight,
    },
    io::resolver::{DataUriResolver, Resolver},
};

use super::GltfFormat;

#[derive(Debug, Error)]
pub enum GltfImportError {
    #[error("Invalid URI: {0}")]
    InvalidUri(String),
    #[error("Resolver error: {0}")]
    ResolverError(String),
    #[error("Invalid accessor: {0}")]
    InvalidAccessor(String),
    #[error("Failed to read buffer view: {0}")]
    ReadBufferView(#[from] ReadBufferViewError),
}

pub async fn import(
    graph: &mut Graph,
    format: &mut GltfFormat,
    mut resolver: Option<impl Resolver>,
) -> Result<GltfDocument, GltfImportError> {
    let doc = GltfDocument::new(graph);

    // Create buffers
    let mut buffers = Vec::new();
    let mut buffer_data = Vec::new();

    let buffers_len = format.json.buffers.len();
    let resources_len = format.resources.len();

    for buf in format.json.buffers.iter_mut() {
        let mut buffer = doc.create_buffer(graph);
        let weight = buffer.get_mut(graph);

        weight.name = buf.name.clone();
        weight.extras = buf.extras.clone();

        weight.uri = buf.uri.clone();

        let mut data = None;

        if buffers_len == 1 && resources_len == 1 {
            // Assume gltf is a glb, and the buffer is the only resource
            let key = format
                .resources
                .iter_mut()
                .find(|_| true)
                .map(|(k, _)| k.clone())
                .unwrap();

            data = Some(format.resources.remove(&key).unwrap());
        } else if let Some(uri) = weight.uri.as_ref() {
            data = resolve_uri(uri, &mut resolver).await;
        }

        buffer_data.push(data.unwrap_or_default());
        buffers.push(buffer);
    }

    // Create accessors
    let accessors = format
        .json
        .accessors
        .iter_mut()
        .map(|a| -> Result<Accessor, GltfImportError> {
            let mut accessor = doc.create_accessor(graph);

            let weight = accessor.get_mut(graph);

            weight.name = a.name.clone();
            weight.extras = a.extras.clone();

            weight.normalized = a.normalized;
            weight.component_type = match a.component_type {
                Checked::Valid(component_type) => component_type.0,
                Checked::Invalid => {
                    return Err(GltfImportError::InvalidAccessor(
                        "Invalid component type".to_string(),
                    ));
                }
            };
            weight.element_type = match a.type_ {
                Checked::Valid(ty) => ty,
                Checked::Invalid => {
                    return Err(GltfImportError::InvalidAccessor("Invalid type".to_string()));
                }
            };

            let buffer_view_idx =
                a.buffer_view
                    .map(|v| v.value())
                    .ok_or(GltfImportError::InvalidAccessor(
                        "No buffer view".to_string(),
                    ))?;

            let buffer_view = &format.json.buffer_views[buffer_view_idx];
            let buffer_idx = buffer_view.buffer.value();

            let data = buffer_data
                .get(buffer_idx)
                .ok_or(GltfImportError::InvalidAccessor(
                    "No buffer data".to_string(),
                ))?;

            let view = read_buffer_view(buffer_view, data)?;

            if let Some(_sparse) = &a.sparse {
                return Err(GltfImportError::InvalidAccessor(
                    "Sparse accessors are not yet supported".to_string(),
                ));
            }

            let accessor_start = a.byte_offset.map(|o| o.0 as usize).unwrap_or_default();
            let item_size = a.component_type.unwrap().0.size() * a.type_.unwrap().multiplicity();
            let accessor_end = accessor_start + (a.count.0 as usize * item_size);

            weight.data = view[accessor_start..accessor_end].to_vec();

            Ok(accessor)
        })
        .collect::<Result<Vec<_>, _>>()?;

    // Create images
    let mut images = Vec::new();

    for img in format.json.images.iter_mut() {
        let mut image = doc.create_image(graph);

        let weight = image.get_mut(graph);
        weight.name = img.name.clone();
        weight.extras = img.extras.clone();
        weight.mime_type = img.mime_type.clone().map(|m| m.0);

        if let Some(uri) = img.uri.as_ref() {
            weight.uri = img.uri.clone();

            if weight.mime_type.is_none() {
                weight.mime_type = guess_mime_type(uri).map(|s| s.to_string())
            }

            if let Some(data) = resolve_uri(uri, &mut resolver).await {
                weight.data = data;
            }
        } else if let Some(index) = img.buffer_view {
            let view = &format.json.buffer_views[index.value()];

            let buffer_idx = view.buffer.value();
            let buf_data = match buffer_data.get(buffer_idx) {
                Some(data) => data.as_slice(),
                None => {
                    warn!("Buffer has no data");
                    &[]
                }
            };

            weight.data = read_buffer_view(view, buf_data)?.to_vec();

            let buffer = buffers[buffer_idx];
            image.set_buffer(graph, Some(buffer));
        }

        images.push(image);
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

            weight.alpha_cutoff = m.alpha_cutoff.unwrap_or_default();
            weight.alpha_mode = m.alpha_mode.unwrap();
            weight.base_color_factor = m.pbr_metallic_roughness.base_color_factor.0;
            weight.double_sided = m.double_sided;
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

            weight.rotation = n
                .rotation
                .map(|r| Quat::from_slice(&r.0))
                .unwrap_or(Quat::IDENTITY);
            weight.translation = n.translation.map(|t| t.into()).unwrap_or_default();
            weight.scale = n.scale.map(|s| s.into()).unwrap_or(glam::Vec3::ONE);

            if let Some(matrix) = n.matrix {
                let matrix = glam::Mat4::from_cols_slice(&matrix);
                let (scale, rotation, translation) = matrix.to_scale_rotation_translation();
                weight.rotation = rotation;
                weight.scale = scale;
                weight.translation = translation;
            }

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

    // Create animtions
    for a in format.json.animations.iter_mut() {
        let mut animation = doc.create_animation(graph);

        let weight = animation.get_mut(graph);
        weight.name = a.name.clone();
        weight.extras = a.extras.clone();

        let samplers = a
            .samplers
            .iter()
            .map(|s| {
                let mut sampler = AnimationSampler::new(graph);

                let weight = sampler.get_mut(graph);
                weight.extras = s.extras.clone();
                weight.interpolation = Some(s.interpolation.unwrap());

                let input_idx = s.input.value();
                let input = accessors[input_idx];
                sampler.set_input(graph, Some(input));

                let output_idx = s.output.value();
                let output = accessors[output_idx];
                sampler.set_output(graph, Some(output));

                sampler
            })
            .collect::<Vec<_>>();

        a.channels.iter().for_each(|c| {
            let mut channel = animation.create_channel(graph);
            let c_weight = channel.get_mut(graph);

            c_weight.extras = c.extras.clone();
            c_weight.path = c.target.path.unwrap();

            let node = nodes[c.target.node.value()];
            channel.set_target(graph, Some(node));

            let sampler_idx = c.sampler.value();
            let sampler = &samplers[sampler_idx];
            channel.set_sampler(graph, Some(*sampler));
        });
    }

    Ok(doc)
}

async fn resolve_uri(uri: &str, resolver: &mut Option<impl Resolver>) -> Option<Vec<u8>> {
    debug!("Resolving URI: {}", uri);

    if let Ok(data) = DataUriResolver.resolve(uri).await {
        debug!("Resolved data URI: {} ({} bytes)", uri, data.len());
        return Some(data);
    }

    let resolver = match resolver {
        Some(r) => r,
        None => {
            return None;
        }
    };

    match resolver.resolve(uri).await {
        Ok(data) => {
            debug!("Resolved URI: {} ({} bytes)", uri, data.len());
            Some(data)
        }
        Err(e) => {
            debug!("Failed to resolve URI: {}", e);
            None
        }
    }
}

#[derive(Debug, Error)]
pub enum ReadBufferViewError {
    #[error("Buffer view index {0} exceeds buffer length {1}")]
    ExceedsBufferLength(usize, usize),
}

fn read_buffer_view<'a>(
    view: &gltf::json::buffer::View,
    buffer_data: &'a [u8],
) -> Result<&'a [u8], ReadBufferViewError> {
    let start = view.byte_offset.map(|o| o.0 as usize).unwrap_or_default();
    let end = start + view.byte_length.0 as usize;

    if end > buffer_data.len() {
        return Err(ReadBufferViewError::ExceedsBufferLength(
            end,
            buffer_data.len(),
        ));
    }

    Ok(&buffer_data[start..end])
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

        weight.mag_filter = sampler.mag_filter.map(|f| f.unwrap());
        weight.min_filter = sampler.min_filter.map(|f| f.unwrap());
        weight.wrap_s = sampler.wrap_s.unwrap();
        weight.wrap_t = sampler.wrap_t.unwrap();
    }

    texture_info
}

fn guess_mime_type(uri: &str) -> Option<&'static str> {
    if uri.ends_with(".png") {
        Some("image/png")
    } else if uri.ends_with(".jpg") || uri.ends_with(".jpeg") {
        Some("image/jpeg")
    } else if uri.ends_with(".gif") {
        Some("image/gif")
    } else if uri.ends_with(".bmp") {
        Some("image/bmp")
    } else if uri.ends_with(".tiff") {
        Some("image/tiff")
    } else if uri.ends_with(".webp") {
        Some("image/webp")
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use gltf::json::{self, texture::Info, validation::USize64, Index};
    use tracing_test::traced_test;

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

        let doc = import(&mut graph, &mut format, None::<DataUriResolver>)
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
