use std::collections::{BTreeMap, HashMap};

use crate::{
    graph::{AccessorArray, NodeCover},
    Gltf,
};

use gltf::json::{self, scene::UnitQuaternion, validation::Checked};

pub fn gltf_to_json(gltf: &Gltf) -> json::Root {
    let mut root = gltf::json::Root::default();

    // Maps of node index -> json index
    let mut accessors = HashMap::new();
    let mut meshes = HashMap::new();

    root.accessors = gltf
        .accessors()
        .iter()
        .enumerate()
        .map(|(i, a)| {
            accessors.insert(a.node.index.index(), i as u32);

            let data = a.data();

            let max = match a.max() {
                AccessorArray::I8(max) => Some(max.iter().map(|x| *x).collect()),
                AccessorArray::U8(max) => Some(max.iter().map(|x| *x).collect()),
                AccessorArray::I16(max) => Some(max.iter().map(|x| *x).collect()),
                AccessorArray::U16(max) => Some(max.iter().map(|x| *x).collect()),
                AccessorArray::U32(max) => Some(max.iter().map(|x| *x).collect()),
                AccessorArray::F32(max) => Some(max.iter().map(|x| *x).collect()),
            };

            let min = match a.min() {
                AccessorArray::I8(min) => Some(min.iter().map(|x| *x).collect()),
                AccessorArray::U8(min) => Some(min.iter().map(|x| *x).collect()),
                AccessorArray::I16(min) => Some(min.iter().map(|x| *x).collect()),
                AccessorArray::U16(min) => Some(min.iter().map(|x| *x).collect()),
                AccessorArray::U32(min) => Some(min.iter().map(|x| *x).collect()),
                AccessorArray::F32(min) => Some(min.iter().map(|x| *x).collect()),
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
                buffer_view: None,
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
                            let data = i.data();
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
                rotation: Some(UnitQuaternion { 0: data.rotation }),
                scale: Some(data.scale),
                extras: None,
                extensions: None,
            }
        })
        .collect();

    root
}
