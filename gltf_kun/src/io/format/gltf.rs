use std::collections::BTreeMap;

use anyhow::Result;
use glam::Quat;
use gltf::json::{
    accessor::{ComponentType, GenericComponentType},
    buffer::Stride,
    scene::UnitQuaternion,
    validation::Checked,
    Index,
};
use petgraph::stable_graph::NodeIndex;
use tracing::{info, warn};

use crate::{
    document::Document,
    graph::{
        accessor::Accessor,
        buffer::Buffer,
        buffer_view::{BufferView, Target},
        mesh::Mesh,
        node::Node,
        primitive::Primitive,
        scene::Scene,
    },
    io::resolver::{file_resolver::FileResolver, Resolver},
};

use super::{ExportFormat, ImportFormat};

pub struct GltfFormat {
    pub json: gltf::json::Root,
    pub blob: Option<Vec<u8>>,
    pub resolver: Option<Box<dyn Resolver + Send + Sync>>,
}

impl GltfFormat {
    pub fn import_file(path: &str) -> Result<Document> {
        let json = serde_json::from_reader(std::fs::File::open(path)?)?;

        let dir = std::path::Path::new(path)
            .parent()
            .expect("Failed to get parent directory");
        let resolver = FileResolver::new(dir);

        GltfFormat {
            json,
            blob: None,
            resolver: Some(Box::new(resolver)),
        }
        .import()
    }
}

impl ImportFormat for GltfFormat {
    fn import(mut self) -> Result<Document> {
        let mut doc = Document::default();

        // Create buffers
        let buffers = self
            .json
            .buffers
            .iter_mut()
            .map(|b| {
                let mut buffer = Buffer::new(&mut doc.0);
                let weight = buffer.get_mut(&mut doc.0);

                weight.name = b.name.take();
                weight.extras = b.extras.take();

                weight.byte_length = b.byte_length.0;
                weight.uri = b.uri.take();

                if let Some(uri) = weight.uri.as_ref() {
                    info!("Resolving URI: {}", uri);

                    if let Some(resolver) = self.resolver.as_ref() {
                        if let Ok(blob) = resolver.resolve(uri) {
                            weight.blob = blob;
                        } else {
                            warn!("Failed to resolve URI: {}", uri);
                        }
                    } else {
                        warn!("No URI resolver provided");
                    }
                }

                buffer
            })
            .collect::<Vec<_>>();

        // Create buffer views
        let buffer_views = self
            .json
            .buffer_views
            .iter_mut()
            .map(|v| {
                let mut view = BufferView::new(&mut doc.0);
                let weight = view.get_mut(&mut doc.0);

                weight.name = v.name.take();
                weight.extras = v.extras.take();

                weight.byte_length = v.byte_length.0;
                weight.byte_offset = v.byte_offset.map(|o| o.0).unwrap_or_default();
                weight.byte_stride = v.byte_stride.map(|s| s.0);

                weight.target = v.target.and_then(|t| match t {
                    Checked::Valid(target) => Some(match target {
                        gltf::json::buffer::Target::ArrayBuffer => Target::ArrayBuffer,
                        gltf::json::buffer::Target::ElementArrayBuffer => {
                            Target::ElementArrayBuffer
                        }
                    }),
                    Checked::Invalid => None,
                });

                if let Some(buffer) = buffers.get(v.buffer.value()) {
                    view.set_buffer(&mut doc.0, Some(buffer));
                }

                view
            })
            .collect::<Vec<_>>();

        // Create accessors
        let accessors = self
            .json
            .accessors
            .iter_mut()
            .map(|a| {
                let mut accessor = Accessor::new(&mut doc.0);
                let weight = accessor.get_mut(&mut doc.0);

                weight.name = a.name.take();
                weight.extras = a.extras.take();
                weight.normalized = a.normalized;

                let _component_type = match a.component_type {
                    Checked::Valid(component_type) => component_type.0,
                    Checked::Invalid => ComponentType::F32,
                };

                if let Some(index) = a.buffer_view {
                    if let Some(buffer_view) = buffer_views.get(index.value()) {
                        accessor.set_buffer_view(&mut doc.0, Some(buffer_view));
                    }
                }

                accessor
            })
            .collect::<Vec<_>>();

        // TODO: Create materials

        // Create meshes
        let meshes = self
            .json
            .meshes
            .iter_mut()
            .map(|m| {
                let mut mesh = Mesh::new(&mut doc.0);
                let weight = mesh.get_mut(&mut doc.0);

                weight.name = m.name.take();
                weight.extras = m.extras.take();

                m.primitives.iter_mut().for_each(|p| {
                    let mut primitive = Primitive::new(&mut doc.0);
                    let p_weight = primitive.get_mut(&mut doc.0);

                    p_weight.extras = p.extras.take();
                    p_weight.mode = match p.mode {
                        Checked::Valid(mode) => mode,
                        Checked::Invalid => gltf::mesh::Mode::Triangles,
                    };

                    if let Some(index) = p.indices {
                        if let Some(accessor) = accessors.get(index.value()) {
                            primitive.set_indices(&mut doc.0, Some(accessor));
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

                            primitive.set_attribute(&mut doc.0, semantic, Some(accessor));
                        }
                    });

                    mesh.add_primitive(&mut doc.0, &primitive);
                });

                mesh
            })
            .collect::<Vec<_>>();

        // Create nodes
        let nodes = self
            .json
            .nodes
            .iter_mut()
            .map(|n| {
                let mut node = Node::new(&mut doc.0);
                let weight = node.get_mut(&mut doc.0);

                weight.name = n.name.take();
                weight.extras = n.extras.take();

                weight.translation = n.translation.map(|t| t.into()).unwrap_or_default();
                weight.rotation = n
                    .rotation
                    .map(|r| Quat::from_slice(&r.0))
                    .unwrap_or(Quat::IDENTITY);
                weight.scale = n.scale.map(|s| s.into()).unwrap_or(glam::Vec3::ONE);

                if let Some(index) = n.mesh {
                    if let Some(mesh) = meshes.get(index.value()) {
                        node.set_mesh(&mut doc.0, Some(mesh));
                    }
                }

                node
            })
            .collect::<Vec<_>>();

        // Parent nodes
        self.json
            .nodes
            .iter()
            .enumerate()
            .filter_map(|(i, n)| n.children.as_ref().map(|c| (i, c)))
            .for_each(|(i, children)| {
                let node = &nodes[i];

                children.iter().for_each(|idx| {
                    let child = &nodes[idx.value()];
                    node.add_child(&mut doc.0, child);
                });
            });

        // TODO: Create skins

        // Create scenes
        let scenes = self
            .json
            .scenes
            .iter_mut()
            .map(|s| {
                let mut scene = Scene::new(&mut doc.0);
                let weight = scene.get_mut(&mut doc.0);

                weight.name = s.name.take();
                weight.extras = s.extras.take();

                s.nodes.iter().for_each(|idx| {
                    if let Some(node) = nodes.get(idx.value()) {
                        scene.add_node(&mut doc.0, node);
                    }
                });

                scene
            })
            .collect::<Vec<_>>();

        // Default scene
        if let Some(index) = self.json.scene {
            if let Some(scene) = scenes.get(index.value()) {
                doc.set_default_scene(Some(scene));
            }
        }

        // TODO: Create animations

        Ok(doc)
    }
}

impl ExportFormat for GltfFormat {
    fn export(mut doc: Document) -> Result<Box<GltfFormat>> {
        let mut json = gltf::json::root::Root::default();

        let mut buffer_idxs = BTreeMap::<NodeIndex, u32>::new();
        let mut buffer_view_idxs = BTreeMap::<NodeIndex, u32>::new();
        let mut accessor_idxs = BTreeMap::<NodeIndex, u32>::new();
        let mut mesh_idxs = BTreeMap::<NodeIndex, u32>::new();
        let mut node_idxs = BTreeMap::<NodeIndex, u32>::new();
        let mut scene_idxs = BTreeMap::<NodeIndex, u32>::new();

        // Create buffers
        json.buffers = doc
            .buffers()
            .iter_mut()
            .enumerate()
            .map(|(i, buffer)| {
                buffer_idxs.insert(buffer.0, i as u32);

                let weight = buffer.get_mut(&mut doc.0);

                gltf::json::buffer::Buffer {
                    name: weight.name.take(),
                    extras: weight.extras.take(),
                    extensions: None,

                    byte_length: weight.byte_length.into(),
                    uri: weight.uri.take(),
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

                let count = a.count(&doc.0)? as u64;
                let max = a.max(&doc.0).map(|v| v.into());
                let min = a.min(&doc.0).map(|v| v.into());

                let weight = a.get_mut(&mut doc.0);

                Some(gltf::json::accessor::Accessor {
                    name: weight.name.take(),
                    extras: weight.extras.take(),
                    extensions: None,

                    buffer_view: Some(Index::new(*buffer_view_idx)),
                    byte_offset: Some(weight.byte_offset.into()),
                    component_type: Checked::Valid(GenericComponentType(weight.component_type)),
                    count: count.into(),
                    max,
                    min,
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

                let children = node
                    .children(&doc.0)
                    .iter()
                    .filter_map(|child| node_idxs.get(&child.0))
                    .map(|idx| Index::new(*idx))
                    .collect::<Vec<_>>();

                let weight = node.get_mut(&mut doc.0);

                gltf::json::scene::Node {
                    name: weight.name.take(),
                    extras: weight.extras.take(),
                    extensions: None,

                    camera: None,
                    children: if children.is_empty() {
                        None
                    } else {
                        Some(children)
                    },
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

        Ok(Box::new(GltfFormat {
            json,
            blob: None,
            resolver: None,
        }))
    }
}
