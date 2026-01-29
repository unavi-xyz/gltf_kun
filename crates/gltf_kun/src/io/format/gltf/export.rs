use std::collections::{BTreeMap, HashMap};

use bevy_math::{Quat, Vec3};
use gltf::json::{
    Index,
    accessor::GenericComponentType,
    animation::Target,
    image::MimeType,
    material::{
        EmissiveFactor, NormalTexture, OcclusionTexture, PbrBaseColorFactor, PbrMetallicRoughness,
        StrengthFactor,
    },
    scene::UnitQuaternion,
    texture::Info,
    validation::{Checked, USize64},
};
use petgraph::graph::NodeIndex;
use serde_json::{Number, Value};
use thiserror::Error;
use tracing::warn;

use crate::graph::{
    Graph, GraphNodeWeight,
    gltf::{Buffer, GltfDocument, accessor::iter::AccessorElement},
};

use super::GltfFormat;

#[derive(Debug, Error)]
pub enum GltfExportError {}

#[allow(clippy::too_many_lines)]
pub fn export(graph: &mut Graph, doc: &GltfDocument) -> Result<GltfFormat, GltfExportError> {
    let mut json = gltf::json::root::Root::default();
    let mut resources = HashMap::new();

    let mut accessor_idxs = BTreeMap::<NodeIndex, usize>::new();
    let mut buffer_idxs = BTreeMap::<NodeIndex, usize>::new();
    let mut image_idxs = BTreeMap::<NodeIndex, usize>::new();
    let mut material_idxs = BTreeMap::<NodeIndex, usize>::new();
    let mut mesh_idxs = BTreeMap::<NodeIndex, usize>::new();
    let mut node_idxs = BTreeMap::<NodeIndex, usize>::new();
    let mut scene_idxs = BTreeMap::<NodeIndex, usize>::new();
    let mut skin_idxs = BTreeMap::<NodeIndex, usize>::new();
    let mut uris = BTreeMap::<NodeIndex, String>::new();

    if doc.buffers(graph).is_empty() && !doc.accessors(graph).is_empty() {
        warn!("No buffers found. Creating new buffer.");
        doc.create_buffer(graph);
    }

    // Create buffers
    json.buffers = doc
        .buffers(graph)
        .iter_mut()
        .enumerate()
        .map(|(i, buffer)| {
            buffer_idxs.insert(buffer.0, i);

            let weight = buffer.get(graph);
            let name = weight.name.clone();
            let extras = weight.extras.clone();

            let uri = weight.uri.clone().unwrap_or_else(|| {
                let mut idx = 0;
                loop {
                    let uri = format!("buffer_{idx}.bin");

                    if !uris.values().any(|v| v == &uri) {
                        break uri;
                    }

                    idx += 1;
                }
            });

            resources.insert(uri.clone(), Vec::new());

            uris.insert(buffer.0, uri.clone());

            gltf::json::buffer::Buffer {
                extensions: None,
                extras,
                name,

                byte_length: USize64(0),
                uri: Some(uri),
            }
        })
        .collect::<Vec<_>>();

    // Create accessors
    json.accessors = doc
        .accessors(graph)
        .iter_mut()
        .enumerate()
        .map(|(i, a)| {
            accessor_idxs.insert(a.0, i);

            let count = a.count(graph);
            let max = a.calc_max(graph);
            let min = a.calc_min(graph);
            let [max_value, min_value] = [max, min].map(|m| {
                let m_v: Value = m.into();
                if m_v.is_array() {
                    m_v
                } else {
                    Value::Array(vec![m_v])
                }
            });

            let buffer = a.buffer(graph).unwrap_or_else(|| {
                warn!("Accessor {} has no buffer. Using first buffer.", i);
                let buffers = doc.buffers(graph);
                let buffer = buffers.first().expect("should have at least one element");
                a.set_buffer(graph, Some(*buffer));
                *buffer
            });

            let weight = a.get(graph);

            let buffer_view = create_buffer_view(
                buffer,
                &buffer_idxs,
                &mut json.buffers,
                &uris,
                &mut resources,
                &weight.data,
            );

            let buffer_view_idx = json.buffer_views.len();
            json.buffer_views.push(buffer_view);

            gltf::json::accessor::Accessor {
                extensions: None,
                extras: weight.extras.clone(),
                name: weight.name.clone(),

                buffer_view: Some(Index::new(buffer_view_idx as u32)),
                byte_offset: None,
                component_type: Checked::Valid(GenericComponentType(weight.component_type)),
                count: count.into(),
                max: Some(max_value),
                min: Some(min_value),
                normalized: weight.normalized,
                sparse: None,
                type_: Checked::Valid(weight.element_type),
            }
        })
        .collect::<Vec<_>>();

    // Create images
    json.images = doc
        .images(graph)
        .iter_mut()
        .enumerate()
        .map(|(i, image)| {
            image_idxs.insert(image.0, i);

            let weight = image.take(graph);
            let mime_type = weight.mime_type.map(MimeType);

            let mut json_img = gltf::json::image::Image {
                extensions: None,
                extras: weight.extras,
                name: weight.name,

                uri: None,
                buffer_view: None,
                mime_type: mime_type.clone(),
            };

            if let Some(buffer) = image.buffer(graph) {
                let buffer_view = create_buffer_view(
                    buffer,
                    &buffer_idxs,
                    &mut json.buffers,
                    &uris,
                    &mut resources,
                    &weight.data,
                );

                let buffer_view_idx = json.buffer_views.len();
                json.buffer_views.push(buffer_view);

                json_img.buffer_view = Some(Index::new(buffer_view_idx as u32));
            } else {
                let uri = weight.uri.unwrap_or_else(|| {
                    let file_ext = if let Some(MimeType(mime_type)) = &mime_type {
                        match mime_type.as_str() {
                            "image/jpeg" => ".jpg",
                            "image/png" => ".png",
                            "image/webp" => ".webp",
                            "image/gif" => ".gif",
                            _ => {
                                warn!("No known file extension for mime type: {}", mime_type);
                                ""
                            }
                        }
                    } else {
                        warn!(
                            "No mime type for image {}. Exporting image without a file extension.",
                            i
                        );
                        ""
                    };

                    let mut idx = 0;

                    loop {
                        let without_ext = format!("image_{idx}");
                        let uri = format!("{without_ext}{file_ext}");

                        if !uris.values().any(|v| v.starts_with(&without_ext)) {
                            break uri;
                        }

                        idx += 1;
                    }
                });

                resources.insert(uri.clone(), weight.data);
                json_img.uri = Some(uri);
            }

            json_img
        })
        .collect::<Vec<_>>();

    // Create textures
    json.textures = doc
        .textures(graph)
        .iter_mut()
        .map(|texture| {
            let weight = texture.get(graph);

            let image = texture
                .image(graph)
                .and_then(|image| image_idxs.get(&image.0))
                .map(|idx| Index::new(*idx as u32))
                .expect("value should exist");

            let sampler_idx = json.samplers.len();
            json.samplers.push(gltf::json::texture::Sampler {
                extensions: None,
                extras: weight.extras.clone(),
                name: weight.name.clone(),

                mag_filter: weight.mag_filter.map(Checked::Valid),
                min_filter: weight.min_filter.map(Checked::Valid),
                wrap_s: Checked::Valid(weight.wrap_s),
                wrap_t: Checked::Valid(weight.wrap_t),
            });

            gltf::json::texture::Texture {
                extensions: None,
                extras: weight.extras.clone(),
                name: weight.name.clone(),

                sampler: Some(Index::new(sampler_idx as u32)),
                source: image,
            }
        })
        .collect::<Vec<_>>();

    // Create materials
    json.materials = doc
        .materials(graph)
        .iter_mut()
        .enumerate()
        .map(|(i, material)| {
            material_idxs.insert(material.0, i);

            let weight = material.get(graph);

            let base_color_texture = material.base_color_texture(graph).map(|t| Info {
                extensions: None,
                extras: None,
                index: Index::new(
                    doc.textures(graph)
                        .iter()
                        .position(|tex| tex.0 == t.0)
                        .expect("value should exist in collection") as u32,
                ),
                tex_coord: weight.base_color_tex_coord as u32,
            });

            let metallic_roughness_texture =
                material.metallic_roughness_texture(graph).map(|t| Info {
                    extensions: None,
                    extras: None,
                    index: Index::new(
                        doc.textures(graph)
                            .iter()
                            .position(|tex| tex.0 == t.0)
                            .expect("value should exist in collection")
                            as u32,
                    ),
                    tex_coord: weight.metallic_roughness_tex_coord as u32,
                });

            let normal_texture = material.normal_texture(graph).map(|t| NormalTexture {
                extensions: None,
                extras: None,
                index: Index::new(
                    doc.textures(graph)
                        .iter()
                        .position(|tex| tex.0 == t.0)
                        .expect("value should exist in collection") as u32,
                ),
                tex_coord: weight.normal_tex_coord as u32,
                scale: weight.normal_scale,
            });

            let occlusion_texture = material.occlusion_texture(graph).map(|t| OcclusionTexture {
                extensions: None,
                extras: None,
                index: Index::new(
                    doc.textures(graph)
                        .iter()
                        .position(|tex| tex.0 == t.0)
                        .expect("value should exist in collection") as u32,
                ),
                tex_coord: weight.occlusion_tex_coord as u32,
                strength: StrengthFactor(weight.occlusion_strength),
            });

            let emissive_texture = material.emissive_texture(graph).map(|t| Info {
                extensions: None,
                extras: None,
                index: Index::new(
                    doc.textures(graph)
                        .iter()
                        .position(|tex| tex.0 == t.0)
                        .expect("value should exist in collection") as u32,
                ),
                tex_coord: weight.emissive_tex_coord as u32,
            });

            gltf::json::material::Material {
                name: weight.name.clone(),
                extras: weight.extras.clone(),
                extensions: None,

                alpha_cutoff: Some(weight.alpha_cutoff),
                alpha_mode: Checked::Valid(weight.alpha_mode),
                double_sided: weight.double_sided,
                emissive_factor: EmissiveFactor(weight.emissive_factor),
                emissive_texture,
                normal_texture,
                occlusion_texture,
                pbr_metallic_roughness: PbrMetallicRoughness {
                    base_color_factor: PbrBaseColorFactor(weight.base_color_factor),
                    base_color_texture,
                    extensions: None,
                    extras: None,
                    metallic_factor: StrengthFactor(weight.metallic_factor),
                    metallic_roughness_texture,
                    roughness_factor: StrengthFactor(weight.roughness_factor),
                },
            }
        })
        .collect::<Vec<_>>();

    // Create meshes
    json.meshes = doc
        .meshes(graph)
        .iter_mut()
        .enumerate()
        .map(|(i, mesh)| {
            mesh_idxs.insert(mesh.0, i);

            let primitives = mesh
                .primitives(graph)
                .iter()
                .map(|p| {
                    let weight = p.get(graph);

                    let indices = p
                        .indices(graph)
                        .and_then(|indices| accessor_idxs.get(&indices.0))
                        .map(|idx| Index::new(*idx as u32));

                    let attributes = p
                        .attributes(graph)
                        .iter()
                        .filter_map(|(k, v)| {
                            accessor_idxs
                                .get(&v.0)
                                .map(|idx| (Checked::Valid(k.clone()), Index::new(*idx as u32)))
                        })
                        .collect::<BTreeMap<_, _>>();

                    let material = p
                        .material(graph)
                        .and_then(|material| material_idxs.get(&material.0))
                        .map(|idx| Index::new(*idx as u32));

                    gltf::json::mesh::Primitive {
                        attributes,
                        indices,
                        material,
                        mode: Checked::Valid(weight.mode),
                        targets: None,
                        extensions: None,
                        extras: None,
                    }
                })
                .collect::<Vec<_>>();

            let weight = mesh.get(graph);

            gltf::json::mesh::Mesh {
                name: weight.name.clone(),
                extras: weight.extras.clone(),
                extensions: None,

                weights: if weight.weights.is_empty() {
                    None
                } else {
                    Some(weight.weights.clone())
                },
                primitives,
            }
        })
        .collect::<Vec<_>>();

    // Create nodes
    json.nodes = doc
        .nodes(graph)
        .iter_mut()
        .enumerate()
        .map(|(i, node)| {
            node_idxs.insert(node.0, i);

            let mesh = node
                .mesh(graph)
                .and_then(|mesh| mesh_idxs.get(&mesh.0))
                .map(|idx| Index::new(*idx as u32));

            let weight = node.get(graph);

            gltf::json::scene::Node {
                name: weight.name.clone(),
                extras: weight.extras.clone(),
                extensions: None,

                camera: None,
                children: None,
                skin: None,
                matrix: None,
                mesh,
                rotation: if weight.rotation == Quat::IDENTITY {
                    None
                } else {
                    Some(UnitQuaternion(weight.rotation.into()))
                },
                scale: if weight.scale == Vec3::ONE {
                    None
                } else {
                    Some(weight.scale.into())
                },
                translation: if weight.translation == Vec3::ZERO {
                    None
                } else {
                    Some(weight.translation.into())
                },
                weights: if weight.weights.is_empty() {
                    None
                } else {
                    Some(weight.weights.clone())
                },
            }
        })
        .collect::<Vec<_>>();

    // Parent nodes
    doc.nodes(graph).iter().for_each(|node| {
        let children_idxs = node
            .children(graph)
            .iter()
            .filter_map(|child| node_idxs.get(&child.0))
            .map(|idx| Index::new(*idx as u32))
            .collect::<Vec<_>>();

        let idx = node_idxs.get(&node.0).expect("key should exist in map");
        let node = json.nodes.get_mut(*idx).expect("key should exist in map");

        if !children_idxs.is_empty() {
            node.children = Some(children_idxs);
        }
    });

    // Create scenes
    json.scenes = doc
        .scenes(graph)
        .iter_mut()
        .enumerate()
        .map(|(i, scene)| {
            scene_idxs.insert(scene.0, i);

            let nodes = scene
                .nodes(graph)
                .iter()
                .filter_map(|node| node_idxs.get(&node.0))
                .map(|idx| Index::new(*idx as u32))
                .collect::<Vec<_>>();

            let weight = scene.get(graph);

            gltf::json::scene::Scene {
                name: weight.name.clone(),
                extras: weight.extras.clone(),
                extensions: None,

                nodes,
            }
        })
        .collect::<Vec<_>>();

    // Default scene
    if let Some(scene) = doc.default_scene(graph) {
        json.scene = scene_idxs.get(&scene.0).map(|idx| Index::new(*idx as u32));
    }

    // Create skins
    json.skins = doc
        .skins(graph)
        .iter_mut()
        .enumerate()
        .map(|(i, skin)| {
            skin_idxs.insert(skin.0, i);

            let weight = skin.get(graph);

            let inverse_bind_matrices = skin
                .inverse_bind_matrices(graph)
                .and_then(|accessor| accessor_idxs.get(&accessor.0))
                .map(|idx| Index::new(*idx as u32));

            let skeleton = skin
                .skeleton(graph)
                .and_then(|node| node_idxs.get(&node.0))
                .map(|idx| Index::new(*idx as u32));

            let joints = skin
                .joints(graph)
                .iter()
                .filter_map(|joint| node_idxs.get(&joint.0))
                .map(|idx| Index::new(*idx as u32))
                .collect::<Vec<_>>();

            gltf::json::skin::Skin {
                name: weight.name.clone(),
                extras: weight.extras.clone(),
                extensions: None,

                inverse_bind_matrices,
                skeleton,
                joints,
            }
        })
        .collect::<Vec<_>>();

    doc.nodes(graph).iter().for_each(|node| {
        if let Some(skin) = node.skin(graph) {
            let skin_idx = skin_idxs.get(&skin.0).expect("key should exist in map");
            let node_idx = node_idxs.get(&node.0).expect("key should exist in map");

            let node = json
                .nodes
                .get_mut(*node_idx)
                .expect("key should exist in map");
            node.skin = Some(Index::new(*skin_idx as u32));
        }
    });

    // Create animations
    json.animations =
        doc.animations(graph)
            .iter_mut()
            .map(|animation| {
                let mut samplers = Vec::new();

                let channels = animation
                    .channels(graph)
                    .iter()
                    .filter_map(|c| {
                        let Some(sampler) = c.sampler(graph) else {
                            warn!("No sampler found for animation channel.");
                            return None;
                        };

                        let sampler_index = samplers
                            .iter()
                            .position(|s| *s == sampler)
                            .unwrap_or_else(|| {
                                let idx = samplers.len();
                                samplers.push(sampler);
                                idx
                            });

                        let Some(node) = c.target(graph) else {
                            warn!("No target found for animation channel.");
                            return None;
                        };

                        let node_idx = node_idxs.get(&node.0).expect("key should exist in map");

                        let weight = c.get(graph);

                        Some(gltf::json::animation::Channel {
                            extensions: None,
                            extras: weight.extras.clone(),

                            sampler: Index::new(sampler_index as u32),
                            target: Target {
                                extensions: None,
                                extras: None,

                                node: Index::new(*node_idx as u32),
                                path: Checked::Valid(weight.path),
                            },
                        })
                    })
                    .collect::<Vec<_>>();

                let samplers = samplers
                    .iter()
                    .filter_map(|s| {
                        let Some(input) = s.input(graph) else {
                            warn!("No input found for animation sampler.");
                            return None;
                        };

                        let Some(output) = s.output(graph) else {
                            warn!("No output found for animation sampler.");
                            return None;
                        };

                        let input_idx = accessor_idxs
                            .get(&input.0)
                            .expect("key should exist in map");
                        let output_idx = accessor_idxs
                            .get(&output.0)
                            .expect("key should exist in map");

                        let weight = s.get(graph);

                        Some(gltf::json::animation::Sampler {
                            extensions: None,
                            extras: weight.extras.clone(),

                            input: Index::new(*input_idx as u32),
                            interpolation: Checked::Valid(weight.interpolation),
                            output: Index::new(*output_idx as u32),
                        })
                    })
                    .collect::<Vec<_>>();

                let weight = animation.get(graph);

                gltf::json::animation::Animation {
                    name: weight.name.clone(),
                    extras: weight.extras.clone(),
                    extensions: None,

                    channels,
                    samplers,
                }
            })
            .collect::<Vec<_>>();

    Ok(GltfFormat { json, resources })
}

fn create_buffer_view(
    buffer: Buffer,
    buffer_idxs: &BTreeMap<NodeIndex, usize>,
    buffers: &mut [gltf::json::buffer::Buffer],
    uris: &BTreeMap<NodeIndex, String>,
    resources: &mut HashMap<String, Vec<u8>>,
    data: &[u8],
) -> gltf::json::buffer::View {
    let buffer_idx = buffer_idxs.get(&buffer.0).expect("key should exist in map");
    let buffer_json = buffers
        .get_mut(*buffer_idx)
        .expect("key should exist in map");
    let buffer_uri = uris.get(&buffer.0).expect("key should exist in map");
    let buffer_resource = resources
        .get_mut(buffer_uri)
        .expect("key should exist in map");

    let byte_length = data.len();

    let buffer_view = gltf::json::buffer::View {
        extensions: None,
        extras: None,
        name: None,

        buffer: Index::new(*buffer_idx as u32),
        byte_length: byte_length.into(),
        byte_offset: Some(buffer_json.byte_length),
        byte_stride: None,
        target: None,
    };

    buffer_json.byte_length = USize64(buffer_json.byte_length.0 + byte_length as u64);
    buffer_resource.extend(data.iter());

    buffer_view
}

impl From<AccessorElement> for Value {
    fn from(value: AccessorElement) -> Self {
        match value {
            AccessorElement::F32(value) => Number::from_f64(f64::from(value))
                .expect("f64 should convert to json number")
                .into(),
            AccessorElement::F32x2(value) => Self::Array(
                value
                    .iter()
                    .map(|v| {
                        Number::from_f64(f64::from(*v))
                            .expect("f64 should convert to json number")
                            .into()
                    })
                    .collect(),
            ),
            AccessorElement::F32x3(value) => Self::Array(
                value
                    .iter()
                    .map(|v| {
                        Number::from_f64(f64::from(*v))
                            .expect("f64 should convert to json number")
                            .into()
                    })
                    .collect(),
            ),
            AccessorElement::F32x4(value) => Self::Array(
                value
                    .iter()
                    .map(|v| {
                        Number::from_f64(f64::from(*v))
                            .expect("f64 should convert to json number")
                            .into()
                    })
                    .collect(),
            ),
            AccessorElement::F32x16(value) => Self::Array(
                value
                    .iter()
                    .map(|v| {
                        Number::from_f64(f64::from(*v))
                            .expect("f64 should convert to json number")
                            .into()
                    })
                    .collect(),
            ),
            AccessorElement::U32(value) => Self::Number(value.into()),
            AccessorElement::U32x2(value) => {
                Self::Array(value.iter().map(|v| Self::Number((*v).into())).collect())
            }
            AccessorElement::U32x3(value) => {
                Self::Array(value.iter().map(|v| Self::Number((*v).into())).collect())
            }
            AccessorElement::U32x4(value) => {
                Self::Array(value.iter().map(|v| Self::Number((*v).into())).collect())
            }
            AccessorElement::U16(value) => Self::Number(value.into()),
            AccessorElement::U16x2(value) => {
                Self::Array(value.iter().map(|v| Self::Number((*v).into())).collect())
            }
            AccessorElement::U16x3(value) => {
                Self::Array(value.iter().map(|v| Self::Number((*v).into())).collect())
            }
            AccessorElement::U16x4(value) => {
                Self::Array(value.iter().map(|v| Self::Number((*v).into())).collect())
            }
            AccessorElement::U8(value) => Self::Number(value.into()),
            AccessorElement::U8x2(value) => {
                Self::Array(value.iter().map(|v| Self::Number((*v).into())).collect())
            }
            AccessorElement::U8x3(value) => {
                Self::Array(value.iter().map(|v| Self::Number((*v).into())).collect())
            }
            AccessorElement::U8x4(value) => {
                Self::Array(value.iter().map(|v| Self::Number((*v).into())).collect())
            }
            AccessorElement::I16(value) => Self::Number(value.into()),
            AccessorElement::I16x2(value) => {
                Self::Array(value.iter().map(|v| Self::Number((*v).into())).collect())
            }
            AccessorElement::I16x3(value) => {
                Self::Array(value.iter().map(|v| Self::Number((*v).into())).collect())
            }
            AccessorElement::I16x4(value) => {
                Self::Array(value.iter().map(|v| Self::Number((*v).into())).collect())
            }
            AccessorElement::I8(value) => Self::Number(value.into()),
            AccessorElement::I8x2(value) => {
                Self::Array(value.iter().map(|v| Self::Number((*v).into())).collect())
            }
            AccessorElement::I8x3(value) => {
                Self::Array(value.iter().map(|v| Self::Number((*v).into())).collect())
            }
            AccessorElement::I8x4(value) => {
                Self::Array(value.iter().map(|v| Self::Number((*v).into())).collect())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use tracing_test::traced_test;

    use crate::graph::gltf::{
        Accessor, Buffer, Image, Material, Mesh, Node, Primitive, Scene, Texture,
    };

    use super::*;

    #[traced_test]
    #[test]
    fn test_export() {
        let mut graph = Graph::default();
        let doc = GltfDocument::new(&mut graph);

        let buffer = doc.create_buffer(&mut graph);

        let accessor = doc.create_accessor(&mut graph);
        accessor.set_buffer(&mut graph, Some(buffer));

        let mut image = doc.create_image(&mut graph);
        image.set_buffer(&mut graph, Some(buffer));

        let image_weight = image.get_mut(&mut graph);
        image_weight.data = vec![0, 1, 2, 3];

        let texture = doc.create_texture(&mut graph);
        texture.set_image(&mut graph, Some(image));

        let material = doc.create_material(&mut graph);
        material.set_base_color_texture(&mut graph, Some(texture));

        let mesh = doc.create_mesh(&mut graph);

        let primitive = mesh.create_primitive(&mut graph);
        primitive.set_indices(&mut graph, Some(accessor));
        primitive.set_material(&mut graph, Some(material));

        let node = doc.create_node(&mut graph);
        node.set_mesh(&mut graph, Some(mesh));

        let scene = doc.create_scene(&mut graph);
        scene.add_node(&mut graph, node);

        doc.set_default_scene(&mut graph, Some(scene));

        // Ensure only connected properties are exported
        let _ = Buffer::new(&mut graph);
        let _ = Accessor::new(&mut graph);
        let _ = Image::new(&mut graph);
        let _ = Texture::new(&mut graph);
        let _ = Material::new(&mut graph);
        let _ = Mesh::new(&mut graph);
        let _ = Primitive::new(&mut graph);
        let _ = Node::new(&mut graph);
        let _ = Scene::new(&mut graph);

        let result = export(&mut graph, &doc).expect("export should succeed");

        assert_eq!(result.json.accessors.len(), 1);
        assert_eq!(result.json.buffer_views.len(), 2);
        assert_eq!(result.json.buffers.len(), 1);
        assert_eq!(result.json.images.len(), 1);
        assert_eq!(result.json.materials.len(), 1);
        assert_eq!(result.json.meshes.len(), 1);
        assert_eq!(result.json.nodes.len(), 1);
        assert_eq!(result.json.samplers.len(), 1);
        assert_eq!(result.json.scenes.len(), 1);
        assert_eq!(result.json.scene, Some(Index::new(0)));
        assert_eq!(result.json.textures.len(), 1);
    }
}
