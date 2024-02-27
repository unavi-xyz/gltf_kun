use std::collections::{BTreeMap, HashMap};

use glam::Quat;
use gltf::json::{
    accessor::GenericComponentType,
    image::MimeType,
    material::{
        EmissiveFactor, NormalTexture, PbrBaseColorFactor, PbrMetallicRoughness, StrengthFactor,
    },
    scene::UnitQuaternion,
    validation::{Checked, USize64},
    Index,
};
use petgraph::graph::NodeIndex;
use serde_json::{Number, Value};
use thiserror::Error;
use tracing::warn;

use crate::graph::{
    gltf::{
        accessor::iter::AccessorElement, buffer::Buffer, document::GltfDocument,
        texture_info::TextureInfo,
    },
    Graph, GraphNodeWeight,
};

use super::GltfFormat;

#[derive(Debug, Error)]
pub enum GltfExportError {}

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

            let uri = match weight.uri.clone() {
                Some(uri) => uri,
                None => {
                    let mut idx = 0;
                    loop {
                        let uri = format!("buffer_{}.bin", idx);

                        if !uris.values().any(|v| v == &uri) {
                            break uri;
                        }

                        idx += 1;
                    }
                }
            };

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

            let buffer = a.buffer(graph).unwrap_or_else(|| {
                warn!("Accessor {} has no buffer. Using first buffer.", i);
                let buffers = doc.buffers(graph);
                let buffer = buffers.first().unwrap();
                a.set_buffer(graph, Some(*buffer));
                *buffer
            });

            let weight = a.get(graph);

            let buffer_view = create_buffer_view(
                &buffer,
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
                max: Some(max.into()),
                min: Some(min.into()),
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
                    &buffer,
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
                    let file_ext = match &mime_type {
                        Some(MimeType(mime_type)) => match mime_type.as_str() {
                            "image/jpeg" => ".jpg",
                            "image/png" => ".png",
                            "image/webp" => ".webp",
                            "image/gif" => ".gif",
                            _ => {
                                warn!("No known file extension for mime type: {}", mime_type);
                                ""
                            }
                        },
                        None => {
                            warn!("No mime type for image {}. Exporting image without a file extension.", i);
                            ""
                        }
                    };

                    let mut idx = 0;

                    loop {
                        let without_ext = format!("image_{}", idx);
                        let uri = format!("{}{}", without_ext, file_ext);

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

    // Create materials
    json.materials = doc
        .materials(graph)
        .iter_mut()
        .enumerate()
        .map(|(i, material)| {
            material_idxs.insert(material.0, i);

            let weight = material.get(graph);

            let base_color_texture = material
                .base_color_texture_info(graph)
                .and_then(|info| export_texture_info(&info, graph, &mut json, &image_idxs));
            let metallic_roughness_texture = material
                .metallic_roughness_texture_info(graph)
                .and_then(|info| export_texture_info(&info, graph, &mut json, &image_idxs));
            let normal_texture = material.normal_texture_info(graph).and_then(|info| {
                export_texture_info(&info, graph, &mut json, &image_idxs).map(|info| {
                    NormalTexture {
                        extras: Default::default(),
                        extensions: None,

                        index: info.index,
                        tex_coord: info.tex_coord,
                        scale: weight.normal_scale,
                    }
                })
            });
            let occlusion_texture = material.occlusion_texture_info(graph).and_then(|info| {
                export_texture_info(&info, graph, &mut json, &image_idxs).map(|info| {
                    gltf::json::material::OcclusionTexture {
                        extras: Default::default(),
                        extensions: None,

                        index: info.index,
                        tex_coord: info.tex_coord,
                        strength: StrengthFactor(weight.occlusion_strength),
                    }
                })
            });
            let emissive_texture = material
                .emissive_texture_info(graph)
                .and_then(|info| export_texture_info(&info, graph, &mut json, &image_idxs));

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
                    extras: Default::default(),
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
                scale: if weight.scale == glam::Vec3::ONE {
                    None
                } else {
                    Some(weight.scale.into())
                },
                translation: if weight.translation == glam::Vec3::ZERO {
                    None
                } else {
                    Some(weight.translation.into())
                },
                weights: None,
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

        let idx = node_idxs.get(&node.0).unwrap();
        let node = json.nodes.get_mut(*idx).unwrap();

        if !children_idxs.is_empty() {
            node.children = Some(children_idxs);
        };
    });

    // TODO: Create skins

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

    // TODO: Create animations

    Ok(GltfFormat { json, resources })
}

fn create_buffer_view(
    buffer: &Buffer,
    buffer_idxs: &BTreeMap<NodeIndex, usize>,
    buffers: &mut [gltf::json::buffer::Buffer],
    uris: &BTreeMap<NodeIndex, String>,
    resources: &mut HashMap<String, Vec<u8>>,
    data: &[u8],
) -> gltf::json::buffer::View {
    let buffer_idx = buffer_idxs.get(&buffer.0).unwrap();
    let buffer_json = buffers.get_mut(*buffer_idx).unwrap();
    let buffer_uri = uris.get(&buffer.0).unwrap();
    let buffer_resource = resources.get_mut(buffer_uri).unwrap();

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

fn export_texture_info(
    info: &TextureInfo,
    graph: &Graph,
    json: &mut gltf::json::Root,
    image_idxs: &BTreeMap<NodeIndex, usize>,
) -> Option<gltf::json::texture::Info> {
    let image = match info.image(graph) {
        Some(image) => image,
        None => {
            warn!("No image found for texture");
            return None;
        }
    };
    let image_idx = image_idxs.get(&image.0).unwrap();

    let weight = info.get(graph);
    let sampler_idx = json.samplers.len();

    json.samplers.push(gltf::json::texture::Sampler {
        extensions: None,
        extras: Default::default(),
        name: None,

        mag_filter: weight.mag_filter.map(Checked::Valid),
        min_filter: weight.min_filter.map(Checked::Valid),
        wrap_s: Checked::Valid(weight.wrap_s),
        wrap_t: Checked::Valid(weight.wrap_t),
    });

    let texture_idx = json.textures.len();
    json.textures.push(gltf::json::texture::Texture {
        extensions: None,
        extras: weight.extras.clone(),
        name: None,

        sampler: Some(Index::new(sampler_idx as u32)),
        source: Index::new(*image_idx as u32),
    });

    Some(gltf::json::texture::Info {
        extensions: None,
        extras: Default::default(),

        index: Index::new(texture_idx as u32),
        tex_coord: weight.tex_coord as u32,
    })
}

impl From<AccessorElement> for Value {
    fn from(value: AccessorElement) -> Self {
        match value {
            AccessorElement::F32(value) => Number::from_f64(value as f64).unwrap().into(),
            AccessorElement::F32x2(value) => Value::Array(
                value
                    .iter()
                    .map(|v| Number::from_f64(*v as f64).unwrap().into())
                    .collect(),
            ),
            AccessorElement::F32x3(value) => Value::Array(
                value
                    .iter()
                    .map(|v| Number::from_f64(*v as f64).unwrap().into())
                    .collect(),
            ),
            AccessorElement::F32x4(value) => Value::Array(
                value
                    .iter()
                    .map(|v| Number::from_f64(*v as f64).unwrap().into())
                    .collect(),
            ),
            AccessorElement::U32(value) => Value::Number(value.into()),
            AccessorElement::U32x2(value) => {
                Value::Array(value.iter().map(|v| Value::Number((*v).into())).collect())
            }
            AccessorElement::U32x3(value) => {
                Value::Array(value.iter().map(|v| Value::Number((*v).into())).collect())
            }
            AccessorElement::U32x4(value) => {
                Value::Array(value.iter().map(|v| Value::Number((*v).into())).collect())
            }
            AccessorElement::U16(value) => Value::Number(value.into()),
            AccessorElement::U16x2(value) => {
                Value::Array(value.iter().map(|v| Value::Number((*v).into())).collect())
            }
            AccessorElement::U16x3(value) => {
                Value::Array(value.iter().map(|v| Value::Number((*v).into())).collect())
            }
            AccessorElement::U16x4(value) => {
                Value::Array(value.iter().map(|v| Value::Number((*v).into())).collect())
            }
            AccessorElement::U8(value) => Value::Number(value.into()),
            AccessorElement::U8x2(value) => {
                Value::Array(value.iter().map(|v| Value::Number((*v).into())).collect())
            }
            AccessorElement::U8x3(value) => {
                Value::Array(value.iter().map(|v| Value::Number((*v).into())).collect())
            }
            AccessorElement::U8x4(value) => {
                Value::Array(value.iter().map(|v| Value::Number((*v).into())).collect())
            }
            AccessorElement::I16(value) => Value::Number(value.into()),
            AccessorElement::I16x2(value) => {
                Value::Array(value.iter().map(|v| Value::Number((*v).into())).collect())
            }
            AccessorElement::I16x3(value) => {
                Value::Array(value.iter().map(|v| Value::Number((*v).into())).collect())
            }
            AccessorElement::I16x4(value) => {
                Value::Array(value.iter().map(|v| Value::Number((*v).into())).collect())
            }
            AccessorElement::I8(value) => Value::Number(value.into()),
            AccessorElement::I8x2(value) => {
                Value::Array(value.iter().map(|v| Value::Number((*v).into())).collect())
            }
            AccessorElement::I8x3(value) => {
                Value::Array(value.iter().map(|v| Value::Number((*v).into())).collect())
            }
            AccessorElement::I8x4(value) => {
                Value::Array(value.iter().map(|v| Value::Number((*v).into())).collect())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use tracing_test::traced_test;

    use crate::graph::gltf::{Accessor, Buffer, Image, Material, Mesh, Node, Primitive, Scene};

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

        let texture = TextureInfo::new(&mut graph);
        texture.set_image(&mut graph, Some(image));

        let material = doc.create_material(&mut graph);
        material.set_base_color_texture_info(&mut graph, Some(texture));

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
        let _ = TextureInfo::new(&mut graph);
        let _ = Material::new(&mut graph);
        let _ = Mesh::new(&mut graph);
        let _ = Primitive::new(&mut graph);
        let _ = Node::new(&mut graph);
        let _ = Scene::new(&mut graph);

        let result = export(&mut graph, &doc).unwrap();

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
