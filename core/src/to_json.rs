use std::collections::{BTreeMap, HashMap};

use crate::{graph::AccessorArray, Gltf};

use gltf::json::{self, scene::UnitQuaternion, validation::Checked};

pub fn gltf_to_json(gltf: &Gltf) -> (json::Root, Vec<u8>) {
    let mut root = json::Root::default();

    root.asset.generator = Some("gltf_kun".to_string());

    // Maps of node index -> json index
    let mut accessors = HashMap::new();
    let mut meshes = HashMap::new();

    let mut buffer = json::Buffer {
        uri: None,
        name: None,
        extras: None,
        extensions: None,
        byte_length: 0,
    };

    let mut buffer_bytes = Vec::new();

    root.accessors = gltf
        .accessors()
        .iter()
        .enumerate()
        .map(|(i, a)| {
            accessors.insert(a.node.index.index(), i as u32);

            let byte_length = a.byte_length() as u32;

            root.buffer_views.push(json::buffer::View {
                buffer: json::Index::new(0),
                byte_offset: Some(buffer.byte_length),
                byte_length,
                byte_stride: None,
                target: None,
                name: None,
                extras: None,
                extensions: None,
            });

            buffer.byte_length += byte_length;
            buffer_bytes.extend_from_slice(a.array().bytes().as_ref());

            let max = match a.max() {
                AccessorArray::I8(max) => Some(max.iter().copied().collect()),
                AccessorArray::U8(max) => Some(max.iter().copied().collect()),
                AccessorArray::I16(max) => Some(max.iter().copied().collect()),
                AccessorArray::U16(max) => Some(max.iter().copied().collect()),
                AccessorArray::U32(max) => Some(max.iter().copied().collect()),
                AccessorArray::F32(max) => Some(max.iter().copied().collect()),
            };

            let min = match a.min() {
                AccessorArray::I8(min) => Some(min.iter().copied().collect()),
                AccessorArray::U8(min) => Some(min.iter().copied().collect()),
                AccessorArray::I16(min) => Some(min.iter().copied().collect()),
                AccessorArray::U16(min) => Some(min.iter().copied().collect()),
                AccessorArray::U32(min) => Some(min.iter().copied().collect()),
                AccessorArray::F32(min) => Some(min.iter().copied().collect()),
            };

            json::Accessor {
                name: a.name(),
                count: a.count() as u32,
                max,
                min,
                type_: Checked::Valid(a.element_type().into()),
                normalized: a.normalized(),
                component_type: Checked::Valid(json::accessor::GenericComponentType(
                    a.component_type(),
                )),
                buffer_view: Some(json::Index::new(root.buffer_views.len() as u32 - 1)),
                byte_offset: None,
                sparse: None,
                extras: None,
                extensions: None,
            }
        })
        .collect();

    root.meshes = gltf
        .meshes()
        .iter()
        .enumerate()
        .map(|(i, m)| {
            meshes.insert(m.node.index.index(), i as u32);

            json::Mesh {
                name: m.name(),
                extras: None,
                extensions: None,
                weights: None,
                primitives: m
                    .primitives()
                    .iter()
                    .map(|p| {
                        let mut attributes = BTreeMap::new();

                        p.attributes().iter().for_each(|attr| {
                            if let Some(accessor) = attr.accessor() {
                                let index = accessors[&accessor.node.index.index()];
                                attributes.insert(
                                    Checked::Valid(attr.semantic().into()),
                                    json::Index::new(index),
                                );
                            }
                        });

                        let indices = p.indices().map(|i| {
                            let index = accessors[&i.node.index.index()];
                            json::Index::new(index)
                        });

                        json::mesh::Primitive {
                            mode: Checked::Valid(p.mode().into()),
                            attributes,
                            indices,
                            targets: None,
                            material: None,
                            extras: None,
                            extensions: None,
                        }
                    })
                    .collect(),
            }
        })
        .collect();

    root.nodes = gltf
        .nodes()
        .iter()
        .map(|n| {
            let mesh = match n.mesh() {
                Some(mesh) => {
                    let index = meshes[&mesh.node.index.index()];
                    Some(json::Index::new(index))
                }
                None => None,
            };

            let translation = if n.translation() == [0.0, 0.0, 0.0] {
                None
            } else {
                Some(n.translation())
            };

            let rotation = if n.rotation() == [0.0, 0.0, 0.0, 1.0] {
                None
            } else {
                Some(UnitQuaternion(n.rotation()))
            };

            let scale = if n.scale() == [1.0, 1.0, 1.0] {
                None
            } else {
                Some(n.scale())
            };

            json::Node {
                name: n.name(),
                camera: None,
                children: None,
                matrix: None,
                mesh,
                skin: None,
                weights: None,
                translation,
                rotation,
                scale,
                extras: None,
                extensions: None,
            }
        })
        .collect();

    root.buffers = vec![buffer];

    (root, buffer_bytes)
}
