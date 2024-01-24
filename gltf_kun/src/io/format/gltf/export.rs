use std::collections::{BTreeMap, HashMap};

use glam::Quat;
use gltf::json::{
    accessor::GenericComponentType,
    scene::UnitQuaternion,
    validation::{Checked, USize64},
    Index,
};
use petgraph::graph::NodeIndex;
use serde_json::{Number, Value};
use thiserror::Error;
use tracing::warn;

use crate::graph::{
    gltf::{accessor::iter::AccessorElement, document::GltfDocument},
    Graph, GraphNode,
};

use super::GltfFormat;

#[derive(Debug, Error)]
pub enum GltfExportError {}

pub fn export(graph: &mut Graph, doc: &GltfDocument) -> Result<GltfFormat, GltfExportError> {
    let mut json = gltf::json::root::Root::default();
    let mut resources = HashMap::new();

    let mut accessor_idxs = BTreeMap::<NodeIndex, usize>::new();
    let mut buffer_idxs = BTreeMap::<NodeIndex, usize>::new();
    let mut mesh_idxs = BTreeMap::<NodeIndex, usize>::new();
    let mut node_idxs = BTreeMap::<NodeIndex, usize>::new();
    let mut scene_idxs = BTreeMap::<NodeIndex, usize>::new();
    let mut uris = BTreeMap::<NodeIndex, String>::new();

    if doc.buffers(graph).len() == 0 && doc.accessors(graph).len() > 0 {
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

            let weight = buffer.get_mut(graph);
            let name = weight.name.take();
            let extras = weight.extras.take();

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
                a.set_buffer(graph, Some(buffer));
                *buffer
            });

            let buffer_idx = buffer_idxs.get(&buffer.0).unwrap();
            let buffer_json = json.buffers.get_mut(*buffer_idx).unwrap();
            let buffer_uri = uris.get(&buffer.0).unwrap();
            let buffer_resource = resources.get_mut(buffer_uri).unwrap();

            let weight = a.get_mut(graph);
            let byte_length = weight.data.len();

            let buffer_view = gltf::json::buffer::View {
                extensions: None,
                extras: None,
                name: None,

                buffer: Index::new(*buffer_idx as u32),
                byte_length: byte_length.into(),
                byte_offset: Some(buffer_json.byte_length),
                byte_stride: None,
                target: None, // TODO
            };

            buffer_json.byte_length = USize64(buffer_json.byte_length.0 + byte_length as u64);
            buffer_resource.extend(weight.data.iter());

            let buffer_view_idx = json.buffer_views.len();
            json.buffer_views.push(buffer_view);

            gltf::json::accessor::Accessor {
                extensions: None,
                extras: weight.extras.take(),
                name: weight.name.take(),

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

    // TODO: Create materials

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

            let weight = mesh.get_mut(graph);

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
        .nodes(graph)
        .iter_mut()
        .enumerate()
        .map(|(i, node)| {
            node_idxs.insert(node.0, i);

            let mesh = node
                .mesh(graph)
                .and_then(|mesh| mesh_idxs.get(&mesh.0))
                .map(|idx| Index::new(*idx as u32));

            let weight = node.get_mut(graph);

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
    doc.nodes(graph).iter().for_each(|node| {
        let children_idxs = node
            .children(graph)
            .iter()
            .filter_map(|child| node_idxs.get(&child.0))
            .map(|idx| Index::new(*idx as u32))
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

            let weight = scene.get_mut(graph);

            gltf::json::scene::Scene {
                name: weight.name.take(),
                extras: weight.extras.take(),
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

    use crate::graph::gltf::{
        accessor::Accessor, buffer::Buffer, mesh::Mesh, node::Node, primitive::Primitive,
        scene::Scene,
    };

    use super::*;

    #[traced_test]
    #[test]
    fn test_export() {
        let mut graph = Graph::default();
        let doc = GltfDocument::new(&mut graph);

        let buffer = doc.create_buffer(&mut graph);

        let accessor = doc.create_accessor(&mut graph);
        accessor.set_buffer(&mut graph, Some(&buffer));

        let mesh = doc.create_mesh(&mut graph);

        let primitive = mesh.create_primitive(&mut graph);
        primitive.set_indices(&mut graph, Some(&accessor));

        let node = doc.create_node(&mut graph);
        node.set_mesh(&mut graph, Some(&mesh));

        let scene = doc.create_scene(&mut graph);
        scene.add_node(&mut graph, &node);

        doc.set_default_scene(&mut graph, Some(&scene));

        // Ensure only connected properties are exported
        let _ = Buffer::new(&mut graph);
        let _ = Accessor::new(&mut graph);
        let _ = Mesh::new(&mut graph);
        let _ = Primitive::new(&mut graph);
        let _ = Node::new(&mut graph);
        let _ = Scene::new(&mut graph);

        let result = export(&mut graph, &doc).unwrap();

        assert_eq!(result.json.accessors.len(), 1);
        assert_eq!(result.json.buffer_views.len(), 1);
        assert_eq!(result.json.buffers.len(), 1);
        assert_eq!(result.json.meshes.len(), 1);
        assert_eq!(result.json.nodes.len(), 1);
        assert_eq!(result.json.scenes.len(), 1);
        assert_eq!(result.json.scene, Some(Index::new(0)));
    }
}
