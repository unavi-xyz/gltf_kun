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
            buffer_bytes.extend_from_slice(a.data().array.bytes().as_ref());

            let data = a.data();

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
                name: data.name,
                count: a.count() as u32,
                max,
                min,
                type_: Checked::Valid(data.element_type.into()),
                normalized: data.normalized,
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

            let data = m.data();

            json::Mesh {
                name: data.name,
                extras: None,
                extensions: None,
                weights: None,
                primitives: m
                    .primitives()
                    .iter()
                    .map(|p| {
                        let data = p.data();
                        let mut attributes = BTreeMap::new();

                        p.attributes().iter().for_each(|attr| {
                            if let Some(accessor) = attr.accessor() {
                                let data = attr.data();
                                let index = accessors[&accessor.node.index.index()];
                                attributes.insert(
                                    Checked::Valid(data.semantic.into()),
                                    json::Index::new(index),
                                );
                            }
                        });

                        let indices = p.indices().map(|i| {
                            let _data = i.data();
                            let index = accessors[&i.node.index.index()];
                            json::Index::new(index)
                        });

                        json::mesh::Primitive {
                            mode: Checked::Valid(data.mode.into()),
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
            let data = n.data();

            let mesh = match n.mesh() {
                Some(mesh) => {
                    let index = meshes[&mesh.node.index.index()];
                    Some(json::Index::new(index))
                }
                None => None,
            };

            json::Node {
                name: data.name,
                camera: None,
                children: None,
                matrix: None,
                mesh,
                skin: None,
                weights: None,
                translation: Some(data.translation),
                rotation: Some(UnitQuaternion(data.rotation)),
                scale: Some(data.scale),
                extras: None,
                extensions: None,
            }
        })
        .collect();

    root.buffers = vec![buffer];

    (root, buffer_bytes)
}
