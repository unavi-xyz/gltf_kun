use glam::Quat;
use gltf::json::{accessor::ComponentType, validation::Checked};
use thiserror::Error;
use tracing::{debug, error, warn};

use crate::{
    graph::{
        gltf::{
            accessor::Accessor,
            buffer::Buffer,
            buffer_view::{BufferView, Target},
            document::GltfDocument,
            mesh::Mesh,
            node::Node,
            primitive::Primitive,
            scene::Scene,
        },
        Graph,
    },
    io::resolver::Resolver,
};

use super::GltfFormat;

#[derive(Debug, Error)]
pub enum GltfImportError {}

pub async fn import(
    format: &mut GltfFormat,
    resolver: &mut Option<impl Resolver>,
) -> Result<GltfDocument, GltfImportError> {
    let mut graph = Graph::default();
    let doc = GltfDocument::new(&mut graph);

    // Create buffers
    let mut buffers = Vec::new();

    for b in format.json.buffers.iter_mut() {
        let mut buffer = Buffer::new(&mut graph);
        let weight = buffer.get_mut(&mut graph);

        weight.name = b.name.take();
        weight.extras = b.extras.take();

        weight.byte_length = b.byte_length.0 as usize;
        weight.uri = b.uri.take();

        if resolver.is_none() && format.resources.len() == 1 {
            let key = format
                .resources
                .iter_mut()
                .find(|_| true)
                .map(|(k, _)| k.clone())
                .unwrap();
            weight.blob = format.resources.remove(&key);
        } else if let Some(uri) = weight.uri.as_ref() {
            if let Some(resolver) = resolver {
                if let Ok(blob) = resolver.resolve(uri).await {
                    debug!("Resolved buffer: {} ({} bytes)", uri, blob.len());
                    weight.blob = Some(blob);
                } else {
                    warn!("Failed to resolve URI: {}", uri);
                }
            } else {
                warn!("No resolver provided");
            }
        }

        buffers.push(buffer);
    }

    // Create buffer views
    let buffer_views = format
        .json
        .buffer_views
        .iter_mut()
        .map(|v| {
            let mut view = BufferView::new(&mut graph);
            let weight = view.get_mut(&mut graph);

            weight.name = v.name.take();
            weight.extras = v.extras.take();

            weight.byte_length = v.byte_length.0 as usize;
            weight.byte_offset = v.byte_offset.map(|o| o.0).unwrap_or_default() as usize;
            weight.byte_stride = v.byte_stride.map(|s| s.0);

            weight.target = v.target.and_then(|t| match t {
                Checked::Valid(target) => Some(match target {
                    gltf::json::buffer::Target::ArrayBuffer => Target::ArrayBuffer,
                    gltf::json::buffer::Target::ElementArrayBuffer => Target::ElementArrayBuffer,
                }),
                Checked::Invalid => None,
            });

            if let Some(buffer) = buffers.get(v.buffer.value()) {
                view.set_buffer(&mut graph, Some(buffer));
            }

            view
        })
        .collect::<Vec<_>>();

    // Create accessors
    let accessors = format
        .json
        .accessors
        .iter_mut()
        .map(|a| {
            let mut accessor = Accessor::new(&mut graph);
            let weight = accessor.get_mut(&mut graph);

            weight.name = a.name.take();
            weight.extras = a.extras.take();

            weight.byte_offset = a.byte_offset.map(|o| o.0).unwrap_or_default() as usize;
            weight.count = a.count.0 as usize;
            weight.normalized = a.normalized;
            weight.component_type = match a.component_type {
                Checked::Valid(component_type) => component_type.0,
                Checked::Invalid => {
                    error!("Invalid accessor component type: {:?}", a.component_type);
                    ComponentType::U8
                }
            };
            weight.element_type = match a.type_ {
                Checked::Valid(ty) => ty,
                Checked::Invalid => {
                    error!("Invalid accessor type: {:?}", a.type_);
                    gltf::json::accessor::Type::Scalar
                }
            };

            if let Some(index) = a.buffer_view {
                if let Some(buffer_view) = buffer_views.get(index.value()) {
                    accessor.set_buffer_view(&mut graph, Some(buffer_view));
                }
            }

            accessor
        })
        .collect::<Vec<_>>();

    // TODO: Create materials

    // Create meshes
    let meshes = format
        .json
        .meshes
        .iter_mut()
        .map(|m| {
            let mut mesh = Mesh::new(&mut graph);
            let weight = mesh.get_mut(&mut graph);

            weight.name = m.name.take();
            weight.extras = m.extras.take();

            m.primitives.iter_mut().for_each(|p| {
                let mut primitive = Primitive::new(&mut graph);
                let p_weight = primitive.get_mut(&mut graph);

                p_weight.extras = p.extras.take();
                p_weight.mode = match p.mode {
                    Checked::Valid(mode) => mode,
                    Checked::Invalid => gltf::mesh::Mode::Triangles,
                };

                if let Some(index) = p.indices {
                    if let Some(accessor) = accessors.get(index.value()) {
                        primitive.set_indices(&mut graph, Some(accessor));
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

                        primitive.set_attribute(&mut graph, semantic, Some(accessor));
                    }
                });

                mesh.add_primitive(&mut graph, &primitive);
            });

            mesh
        })
        .collect::<Vec<_>>();

    // Create nodes
    let nodes = format
        .json
        .nodes
        .iter_mut()
        .map(|n| {
            let mut node = Node::new(&mut graph);
            let weight = node.get_mut(&mut graph);

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
                    node.set_mesh(&mut graph, Some(mesh));
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
                node.add_child(&mut graph, child);
            });
        });

    // TODO: Create skins

    // Create scenes
    let scenes = format
        .json
        .scenes
        .iter_mut()
        .map(|s| {
            let mut scene = Scene::new(&mut graph);
            let weight = scene.get_mut(&mut graph);

            weight.name = s.name.take();
            weight.extras = s.extras.take();

            s.nodes.iter().for_each(|idx| {
                if let Some(node) = nodes.get(idx.value()) {
                    scene.add_node(&mut graph, node);
                }
            });

            scene
        })
        .collect::<Vec<_>>();

    // Default scene
    if let Some(index) = format.json.scene {
        if let Some(scene) = scenes.get(index.value()) {
            doc.set_default_scene(&mut graph, Some(scene));
        }
    }

    // TODO: Create animations

    Ok(doc)
}
