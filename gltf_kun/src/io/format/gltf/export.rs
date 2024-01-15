use std::collections::{BTreeMap, HashMap};

use glam::Quat;
use gltf::json::{
    accessor::GenericComponentType, buffer::Stride, scene::UnitQuaternion, validation::Checked,
    Index,
};
use petgraph::stable_graph::NodeIndex;
use serde_json::{Number, Value};
use thiserror::Error;
use tracing::warn;

use crate::{
    document::GltfDocument,
    graph::gltf::{accessor::iter::AccessorElement, buffer_view::Target},
};

use super::GltfFormat;

#[derive(Debug, Error)]
pub enum GltfExportError {}

impl GltfFormat {
    pub fn export(mut doc: GltfDocument) -> Result<GltfFormat, GltfExportError> {
        let mut json = gltf::json::root::Root::default();
        let mut resources = HashMap::new();

        let mut accessor_idxs = BTreeMap::<NodeIndex, u32>::new();
        let mut buffer_idxs = BTreeMap::<NodeIndex, u32>::new();
        let mut buffer_view_idxs = BTreeMap::<NodeIndex, u32>::new();
        let mut mesh_idxs = BTreeMap::<NodeIndex, u32>::new();
        let mut node_idxs = BTreeMap::<NodeIndex, u32>::new();
        let mut scene_idxs = BTreeMap::<NodeIndex, u32>::new();
        let mut uris = BTreeMap::<NodeIndex, String>::new();

        // Calculate min/max before exporting buffer blobs
        let mut min_max = doc
            .accessors()
            .iter()
            .map(|a| {
                let min = a.calc_min(&doc.0);
                let max = a.calc_max(&doc.0);
                (min, max)
            })
            .collect::<Vec<_>>();

        // Create buffers
        json.buffers = doc
            .buffers()
            .iter_mut()
            .enumerate()
            .map(|(i, buffer)| {
                buffer_idxs.insert(buffer.0, i as u32);

                let weight = buffer.get_mut(&mut doc.0);
                let name = weight.name.take();
                let extras = weight.extras.take();
                let byte_length = weight.byte_length.into();

                let uri = match weight.uri.take() {
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

                weight
                    .blob
                    .take()
                    .map(|blob| resources.insert(uri.clone(), blob));

                uris.insert(buffer.0, uri.clone());

                gltf::json::buffer::Buffer {
                    name,
                    extras,
                    extensions: None,

                    byte_length,
                    uri: Some(uri),
                }
            })
            .collect::<Vec<_>>();

        // Create buffer views
        json.buffer_views = doc
            .buffer_views()
            .iter_mut()
            .enumerate()
            .filter_map(|(i, view)| {
                buffer_view_idxs.insert(view.0, i as u32);

                let buffer_idx = match view
                    .buffer(&doc.0)
                    .and_then(|buffer| buffer_idxs.get(&buffer.0))
                {
                    Some(idx) => idx,
                    None => {
                        warn!("Buffer view has no buffer");
                        return None;
                    }
                };

                let weight = view.get_mut(&mut doc.0);

                Some(gltf::json::buffer::View {
                    name: weight.name.take(),
                    extras: weight.extras.take(),
                    extensions: None,

                    buffer: Index::new(*buffer_idx),
                    byte_length: weight.byte_length.into(),
                    byte_offset: Some(weight.byte_offset.into()),
                    byte_stride: weight.byte_stride.map(Stride),
                    target: weight
                        .target
                        .take()
                        .map(|t| match t {
                            Target::ArrayBuffer => gltf::json::buffer::Target::ArrayBuffer,
                            Target::ElementArrayBuffer => {
                                gltf::json::buffer::Target::ElementArrayBuffer
                            }
                            Target::Unknown(value) => {
                                warn!("Unknown buffer view target: {}", value);
                                gltf::json::buffer::Target::ArrayBuffer
                            }
                        })
                        .map(Checked::Valid),
                })
            })
            .collect::<Vec<_>>();

        // Create accessors
        json.accessors = doc
            .accessors()
            .iter_mut()
            .enumerate()
            .filter_map(|(i, a)| {
                accessor_idxs.insert(a.0, i as u32);

                let buffer_view_idx = match a
                    .buffer_view(&doc.0)
                    .and_then(|buffer_view| buffer_view_idxs.get(&buffer_view.0))
                {
                    Some(idx) => idx,
                    None => {
                        warn!("Accessor has no buffer view");
                        return None;
                    }
                };

                let min = min_max[i].0.take();
                let max = min_max[i].1.take();
                let weight = a.get_mut(&mut doc.0);

                Some(gltf::json::accessor::Accessor {
                    name: weight.name.take(),
                    extras: weight.extras.take(),
                    extensions: None,

                    buffer_view: Some(Index::new(*buffer_view_idx)),
                    byte_offset: Some(weight.byte_offset.into()),
                    component_type: Checked::Valid(GenericComponentType(weight.component_type)),
                    count: weight.count.into(),
                    max: Some(max.into()),
                    min: Some(min.into()),
                    normalized: weight.normalized,
                    sparse: None,
                    type_: Checked::Valid(weight.element_type),
                })
            })
            .collect::<Vec<_>>();

        // TODO: Create materials

        // Create meshes
        json.meshes = doc
            .meshes()
            .iter_mut()
            .enumerate()
            .map(|(i, mesh)| {
                mesh_idxs.insert(mesh.0, i as u32);

                let primitives = mesh
                    .primitives(&doc.0)
                    .iter()
                    .map(|p| {
                        let weight = p.get(&doc.0);

                        let indices = p
                            .indices(&doc.0)
                            .and_then(|indices| accessor_idxs.get(&indices.0))
                            .map(|idx| Index::new(*idx));

                        let attributes = p
                            .attributes(&doc.0)
                            .iter()
                            .filter_map(|(k, v)| {
                                accessor_idxs
                                    .get(&v.0)
                                    .map(|idx| (Checked::Valid(k.clone()), Index::new(*idx)))
                            })
                            .collect::<BTreeMap<_, _>>();

                        gltf::json::mesh::Primitive {
                            attributes,
                            indices,
                            material: None,
                            mode: Checked::Valid(weight.mode),
                            targets: None,
                            extensions: None,
                            extras: None,
                        }
                    })
                    .collect::<Vec<_>>();

                let weight = mesh.get_mut(&mut doc.0);

                gltf::json::mesh::Mesh {
                    name: weight.name.take(),
                    extras: weight.extras.take(),
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
            .nodes()
            .iter_mut()
            .enumerate()
            .map(|(i, node)| {
                node_idxs.insert(node.0, i as u32);

                let mesh = node
                    .mesh(&doc.0)
                    .and_then(|mesh| mesh_idxs.get(&mesh.0))
                    .map(|idx| Index::new(*idx));

                let weight = node.get_mut(&mut doc.0);

                gltf::json::scene::Node {
                    name: weight.name.take(),
                    extras: weight.extras.take(),
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
        doc.nodes().iter().for_each(|node| {
            let children_idxs = node
                .children(&doc.0)
                .iter()
                .filter_map(|child| node_idxs.get(&child.0))
                .map(|idx| Index::new(*idx))
                .collect::<Vec<_>>();

            let idx = node_idxs.get(&node.0).unwrap();
            let node = json.nodes.get_mut(*idx as usize).unwrap();

            if !children_idxs.is_empty() {
                node.children = Some(children_idxs);
            };
        });

        // TODO: Create skins

        // Create scenes
        json.scenes = doc
            .scenes()
            .iter_mut()
            .enumerate()
            .map(|(i, scene)| {
                scene_idxs.insert(scene.0, i as u32);

                let nodes = scene
                    .nodes(&doc.0)
                    .iter()
                    .filter_map(|node| node_idxs.get(&node.0))
                    .map(|idx| Index::new(*idx))
                    .collect::<Vec<_>>();

                let weight = scene.get_mut(&mut doc.0);

                gltf::json::scene::Scene {
                    name: weight.name.take(),
                    extras: weight.extras.take(),
                    extensions: None,

                    nodes,
                }
            })
            .collect::<Vec<_>>();

        // Default scene
        if let Some(scene) = doc.default_scene() {
            json.scene = scene_idxs.get(&scene.0).map(|idx| Index::new(*idx));
        }

        // TODO: Create animations

        Ok(GltfFormat { json, resources })
    }
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
