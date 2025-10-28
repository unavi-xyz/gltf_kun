use bevy_math::{Mat4, Quat, Vec3};
use gltf::{
    Semantic,
    json::{Index, validation::Checked},
};
use thiserror::Error;
use tracing::{debug, error, warn};

use crate::{
    graph::{
        Graph, GraphNodeWeight,
        gltf::{Accessor, animation::AnimationSampler, document::GltfDocument},
    },
    io::resolver::{DataUriResolver, Resolver},
};

use super::GltfFormat;

#[derive(Debug, Error)]
pub enum GltfImportError {
    #[error("invalid URI: {0}")]
    InvalidUri(String),
    #[error("resolver error: {0}")]
    ResolverError(String),
    #[error("invalid accessor: {0}")]
    InvalidAccessor(String),
    #[error(transparent)]
    ReadAccessor(#[from] ReadAccessorError),
    #[error(transparent)]
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

        weight.name.clone_from(&buf.name);
        weight.extras.clone_from(&buf.extras);

        weight.uri.clone_from(&buf.uri);

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
    let accessors =
        format
            .json
            .accessors
            .iter_mut()
            .map(|a| -> Result<Accessor, GltfImportError> {
                let mut accessor = doc.create_accessor(graph);

                let weight = accessor.get_mut(graph);

                weight.name.clone_from(&a.name);
                weight.extras.clone_from(&a.extras);

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

                let buffer_view_idx = a.buffer_view.map(|v| v.value());

                if let Some(sparse) = &a.sparse {
                    let item_size = accessor_item_size(a)
                        .map_err(|e| GltfImportError::InvalidAccessor(e.to_string()))?;

                    let mut base = match buffer_view_idx {
                        Some(idx) => {
                            let view = &format.json.buffer_views[idx];
                            let buffer_idx = view.buffer.value();

                            let data = buffer_data.get(buffer_idx).ok_or(
                                GltfImportError::InvalidAccessor("No buffer data".to_string()),
                            )?;

                            let slice = read_buffer_view(view, data)?;

                            slice.to_vec()
                        }
                        None => {
                            let array_len = item_size * a.count.0 as usize;
                            vec![0; array_len]
                        }
                    };

                    let indices_view_idx = sparse.indices.buffer_view.value();
                    let indices_offset = sparse.indices.byte_offset.0;
                    let indices_component_type = match sparse.indices.component_type {
                        Checked::Valid(component_type) => component_type.0,
                        Checked::Invalid => {
                            return Err(GltfImportError::InvalidAccessor(
                                "Invalid component type".to_string(),
                            ));
                        }
                    };

                    let indices_view = &format.json.buffer_views[indices_view_idx];
                    let indices_buffer_idx = indices_view.buffer.value();

                    let indices_data = buffer_data.get(indices_buffer_idx).ok_or(
                        GltfImportError::InvalidAccessor("No buffer data".to_string()),
                    )?;

                    let indices = read_buffer_view(indices_view, indices_data)?;
                    let indices = &indices[indices_offset as usize..];

                    let values_view_idx = sparse.values.buffer_view.value();
                    let values_offset = sparse.values.byte_offset.0;

                    let values_view = &format.json.buffer_views[values_view_idx];
                    let values_buffer_idx = values_view.buffer.value();

                    let values_data = buffer_data.get(values_buffer_idx).ok_or(
                        GltfImportError::InvalidAccessor("No buffer data".to_string()),
                    )?;

                    let values = read_buffer_view(values_view, values_data)?;
                    let values = &values[values_offset as usize..];

                    for (i, index) in indices
                        .chunks_exact(indices_component_type.size())
                        .enumerate()
                    {
                        let index = match indices_component_type {
                            gltf::json::accessor::ComponentType::U8 => index[0] as usize,
                            gltf::json::accessor::ComponentType::U16 => {
                                u16::from_le_bytes([index[0], index[1]]) as usize
                            }
                            gltf::json::accessor::ComponentType::U32 => {
                                u32::from_le_bytes([index[0], index[1], index[2], index[3]])
                                    as usize
                            }
                            _ => {
                                return Err(GltfImportError::InvalidAccessor(
                                    "Invalid component type".to_string(),
                                ));
                            }
                        };

                        let value = &values[i * item_size..(i + 1) * item_size];

                        base.splice(
                            index * item_size..(index + 1) * item_size,
                            value.iter().cloned(),
                        );
                    }

                    weight.data = base;
                } else {
                    let buffer_view_idx = buffer_view_idx.ok_or(
                        GltfImportError::InvalidAccessor("No buffer view".to_string()),
                    )?;

                    let buffer_view = &format.json.buffer_views[buffer_view_idx];
                    let buffer_idx = buffer_view.buffer.value();

                    let data =
                        buffer_data
                            .get(buffer_idx)
                            .ok_or(GltfImportError::InvalidAccessor(
                                "No buffer data".to_string(),
                            ))?;

                    weight.data = read_accessor(a, buffer_view, data)?;
                }

                Ok(accessor)
            })
            .collect::<Result<Vec<_>, _>>()?;

    // Create images
    let mut images = Vec::new();

    for img in format.json.images.iter_mut() {
        let mut image = doc.create_image(graph);

        let weight = image.get_mut(graph);
        weight.name.clone_from(&img.name);
        weight.extras.clone_from(&img.extras);
        weight.mime_type = img.mime_type.clone().map(|m| m.0);

        if let Some(uri) = img.uri.as_ref() {
            weight.uri.clone_from(&img.uri);

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

    // Create textures
    let textures = format
        .json
        .textures
        .iter_mut()
        .map(|t| {
            let mut texture = doc.create_texture(graph);
            let weight = texture.get_mut(graph);

            weight.name.clone_from(&t.name);
            weight.extras.clone_from(&t.extras);

            let source = t.source.value();
            let image = images.get(source).copied();
            texture.set_image(graph, image);

            if let Some(sampler) = t.sampler {
                let weight = texture.get_mut(graph);
                let sampler = sampler.value();
                let sampler = &format.json.samplers[sampler];

                weight.mag_filter = sampler.mag_filter.map(|f| f.unwrap());
                weight.min_filter = sampler.min_filter.map(|f| f.unwrap());
                weight.wrap_s = sampler.wrap_s.unwrap();
                weight.wrap_t = sampler.wrap_t.unwrap();
            }

            texture
        })
        .collect::<Vec<_>>();

    // Create materials
    let materials = format
        .json
        .materials
        .iter_mut()
        .map(|m| {
            let mut material = doc.create_material(graph);
            let weight = material.get_mut(graph);

            weight.name.clone_from(&m.name);
            weight.extras.clone_from(&m.extras);

            weight.alpha_cutoff = m.alpha_cutoff.unwrap_or_default();
            weight.alpha_mode = m.alpha_mode.unwrap();
            weight.base_color_factor = m.pbr_metallic_roughness.base_color_factor.0;
            weight.double_sided = m.double_sided;
            weight.emissive_factor = m.emissive_factor.0;
            weight.metallic_factor = m.pbr_metallic_roughness.metallic_factor.0;
            weight.roughness_factor = m.pbr_metallic_roughness.roughness_factor.0;

            let base_color_texture = m
                .pbr_metallic_roughness
                .base_color_texture
                .as_ref()
                .and_then(|t| {
                    let weight = material.get_mut(graph);
                    weight.base_color_tex_coord = t.tex_coord as usize;
                    textures.get(t.index.value()).copied()
                });
            let emissive_texture = m.emissive_texture.as_ref().and_then(|t| {
                let weight = material.get_mut(graph);
                weight.emissive_tex_coord = t.tex_coord as usize;
                textures.get(t.index.value()).copied()
            });
            let metallic_roughness_texture = m
                .pbr_metallic_roughness
                .metallic_roughness_texture
                .as_ref()
                .and_then(|t| {
                    let weight = material.get_mut(graph);
                    weight.metallic_roughness_tex_coord = t.tex_coord as usize;
                    textures.get(t.index.value()).copied()
                });
            let normal_texture = m.normal_texture.as_ref().and_then(|t| {
                let weight = material.get_mut(graph);
                weight.normal_scale = t.scale;
                weight.normal_tex_coord = t.tex_coord as usize;
                textures.get(t.index.value()).copied()
            });
            let occlusion_texture = m.occlusion_texture.as_ref().and_then(|t| {
                let weight = material.get_mut(graph);
                weight.occlusion_strength = t.strength.0;
                weight.occlusion_tex_coord = t.tex_coord as usize;
                textures.get(t.index.value()).copied()
            });

            material.set_base_color_texture(graph, base_color_texture);
            material.set_emissive_texture(graph, emissive_texture);
            material.set_metallic_roughness_texture(graph, metallic_roughness_texture);
            material.set_normal_texture(graph, normal_texture);
            material.set_occlusion_texture(graph, occlusion_texture);

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

            weight.extras.clone_from(&m.extras);
            weight.name.clone_from(&m.name);

            if let Some(weights) = &m.weights {
                weight.weights.clone_from(weights);
            }

            for p in m.primitives.iter() {
                let mut primitive = mesh.create_primitive(graph);
                let p_weight = primitive.get_mut(graph);

                p_weight.extras.clone_from(&p.extras);
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

                for (k, v) in p.attributes.iter() {
                    if let Some(accessor) = accessors.get(v.value()) {
                        let semantic = match k {
                            Checked::Valid(semantic) => semantic,
                            Checked::Invalid => {
                                warn!("Invalid attribute semantic: {:?}", k);
                                break;
                            }
                        };

                        primitive.set_attribute(graph, semantic.clone(), Some(*accessor));
                    }
                }

                if let Some(targets) = &p.targets {
                    for (i, target) in targets.iter().enumerate() {
                        let morph_target = primitive.create_morph_target(graph, i);

                        if let Some(positions_idx) = target.positions {
                            let accessor = accessors.get(positions_idx.value()).copied();
                            morph_target.set_attribute(graph, Semantic::Positions, accessor);
                        }

                        if let Some(normals_idx) = target.normals {
                            let accessor = accessors.get(normals_idx.value()).copied();
                            morph_target.set_attribute(graph, Semantic::Normals, accessor);
                        }

                        if let Some(tangents_idx) = target.tangents {
                            let accessor = accessors.get(tangents_idx.value()).copied();
                            morph_target.set_attribute(graph, Semantic::Tangents, accessor);
                        }
                    }
                }
            }

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

            weight.name.clone_from(&n.name);
            weight.extras.clone_from(&n.extras);

            weight.rotation = n
                .rotation
                .map(|r| Quat::from_slice(&r.0))
                .unwrap_or(Quat::IDENTITY);
            weight.translation = n.translation.map(|t| t.into()).unwrap_or_default();
            weight.scale = n.scale.map(|s| s.into()).unwrap_or(Vec3::ONE);

            if let Some(matrix) = n.matrix {
                let matrix = Mat4::from_cols_slice(&matrix);
                let (scale, rotation, translation) = matrix.to_scale_rotation_translation();
                weight.rotation = rotation;
                weight.scale = scale;
                weight.translation = translation;
            }

            if let Some(weights) = &n.weights {
                weight.weights.clone_from(weights);
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

    // Create scenes
    let scenes = format
        .json
        .scenes
        .iter_mut()
        .map(|s| {
            let mut scene = doc.create_scene(graph);
            let weight = scene.get_mut(graph);

            weight.name.clone_from(&s.name);
            weight.extras.clone_from(&s.extras);

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

    // Create skins
    for (i, s) in format.json.skins.iter_mut().enumerate() {
        let mut skin = doc.create_skin(graph);

        let weight = skin.get_mut(graph);
        weight.name.clone_from(&s.name);
        weight.extras.clone_from(&s.extras);

        if let Some(inverse_bind_matrices) = s.inverse_bind_matrices {
            if let Some(accessor) = accessors.get(inverse_bind_matrices.value()) {
                skin.set_inverse_bind_matrices(graph, Some(*accessor));
            }
        }

        if let Some(skeleton) = s.skeleton {
            if let Some(node) = nodes.get(skeleton.value()) {
                skin.set_skeleton(graph, Some(*node));
            }
        }

        for (j, joint_idx) in s.joints.iter().enumerate() {
            let joint_node = nodes.get(joint_idx.value()).unwrap();
            skin.add_joint(graph, joint_node, j);
        }

        for (j, n) in format.json.nodes.iter().enumerate() {
            if n.skin == Some(Index::new(i as u32)) {
                let node = &nodes[j];
                node.set_skin(graph, Some(skin));
            }
        }
    }

    // Create animtions
    for a in format.json.animations.iter_mut() {
        let mut animation = doc.create_animation(graph);

        let weight = animation.get_mut(graph);
        weight.name.clone_from(&a.name);
        weight.extras.clone_from(&a.extras);

        let samplers = a
            .samplers
            .iter()
            .map(|s| {
                let mut sampler = AnimationSampler::new(graph);

                let weight = sampler.get_mut(graph);
                weight.extras.clone_from(&s.extras);
                weight.interpolation = s.interpolation.unwrap();

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

            c_weight.extras.clone_from(&c.extras);
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
    #[error("buffer view index {0} exceeds buffer length {1}")]
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

#[derive(Debug, Error)]
pub enum ItemSizeError {
    #[error("invalid component type")]
    InvalidComponentType,
    #[error("invalid element type")]
    InvalidElementType,
}

fn accessor_item_size(accessor: &gltf::json::Accessor) -> Result<usize, ItemSizeError> {
    let component_size = match accessor.component_type {
        Checked::Valid(component_type) => component_type.0.size(),
        Checked::Invalid => return Err(ItemSizeError::InvalidComponentType),
    };

    let element_size = match accessor.type_ {
        Checked::Valid(ty) => ty.multiplicity(),
        Checked::Invalid => return Err(ItemSizeError::InvalidElementType),
    };

    Ok(component_size * element_size)
}

#[derive(Debug, Error)]
pub enum ReadAccessorError {
    #[error("accessor index {0} exceeds buffer view length {1}")]
    ExceedsBufferViewLength(usize, usize),
    #[error(transparent)]
    ReadBufferViewError(#[from] ReadBufferViewError),
    #[error(transparent)]
    ItemSizeError(#[from] ItemSizeError),
}

fn read_accessor(
    accessor: &gltf::json::Accessor,
    view: &gltf::json::buffer::View,
    buffer_data: &[u8],
) -> Result<Vec<u8>, ReadAccessorError> {
    let view_data = read_buffer_view(view, buffer_data)?;

    let stride = view.byte_stride.map(|s| s.0).unwrap_or_default();

    let start = accessor
        .byte_offset
        .map(|o| o.0 as usize)
        .unwrap_or_default();

    let item_size = accessor_item_size(accessor)?;

    let count = accessor.count.0 as usize;
    let mut end = start + (count * item_size);

    if stride > 0 {
        end += (stride - item_size) * (count - 1);
    }

    if end > view_data.len() {
        return Err(ReadAccessorError::ExceedsBufferViewLength(
            end,
            view_data.len(),
        ));
    }

    if stride == 0 {
        return Ok(view_data[start..end].to_vec());
    }

    let mut data = Vec::with_capacity(accessor.count.0 as usize * item_size);

    for i in (start..end).step_by(stride) {
        data.extend_from_slice(&view_data[i..i + item_size]);
    }

    Ok(data)
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
    use gltf::json::{self, Index, texture::Info, validation::USize64};
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

        let mut graph = Graph::default();

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
