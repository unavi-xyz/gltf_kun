use anyhow::Result;
use glam::Quat;
use gltf::json::{accessor::ComponentType, validation::Checked};
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
    },
    io::resolver::{file_resolver::FileResolver, Resolver},
};

use super::ImportFormat;

pub struct GltfFormat {
    pub json: gltf::json::Root,
    pub blob: Option<Vec<u8>>,
    pub resolver: Option<Box<dyn Resolver>>,
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
        let _views = self
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
                    view.set_buffer(&mut doc.0, Some(*buffer));
                }

                view
            })
            .collect::<Vec<_>>();

        // Create accessors
        let _accessors = self
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

                // let array = match component_type {
                //     ComponentType::I8 => AccessorArray::I8(vec),
                // };

                accessor
            })
            .collect::<Vec<_>>();

        // TODO: Create materials

        // Create meshes
        let _meshes = self
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

                    if let Some(_accessor) = p.indices {}

                    mesh.add_primitive(&mut doc.0, &primitive);
                })
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

        // TODO: Create scenes

        // TODO: Create animations

        Ok(doc)
    }
}
