use glam::Quat;
use gltf::json::{accessor::ComponentType, validation::Checked};
use thiserror::Error;
use tracing::{debug, error, warn};

use crate::{
    graph::{
        gltf::{buffer_view::Target, document::GltfDocument},
        Graph, GraphNode,
    },
    io::resolver::Resolver,
};

use super::GltfFormat;

#[derive(Debug, Error)]
pub enum GltfImportError {}

pub async fn import(
    graph: &mut Graph,
    format: &mut GltfFormat,
    resolver: &mut Option<impl Resolver>,
) -> Result<GltfDocument, GltfImportError> {
    let doc = GltfDocument::new(graph);

    // Create buffers
    let mut buffers = Vec::new();

    for b in format.json.buffers.iter_mut() {
        let mut buffer = doc.create_buffer(graph);
        let weight = buffer.get_mut(graph);

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
            let mut view = doc.create_buffer_view(graph);
            let weight = view.get_mut(graph);

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
                view.set_buffer(graph, Some(buffer));
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
            let mut accessor = doc.create_accessor(graph);
            let weight = accessor.get_mut(graph);

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
                    accessor.set_buffer_view(graph, Some(buffer_view));
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
            let mut mesh = doc.create_mesh(graph);
            let weight = mesh.get_mut(graph);

            weight.name = m.name.take();
            weight.extras = m.extras.take();

            m.primitives.iter_mut().for_each(|p| {
                let mut primitive = mesh.create_primitive(graph);
                let p_weight = primitive.get_mut(graph);

                p_weight.extras = p.extras.take();
                p_weight.mode = match p.mode {
                    Checked::Valid(mode) => mode,
                    Checked::Invalid => gltf::mesh::Mode::Triangles,
                };

                if let Some(index) = p.indices {
                    if let Some(accessor) = accessors.get(index.value()) {
                        primitive.set_indices(graph, Some(accessor));
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

                        primitive.set_attribute(graph, semantic, Some(accessor));
                    }
                });
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
            let mut node = doc.create_node(graph);
            let weight = node.get_mut(graph);

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
                    node.set_mesh(graph, Some(mesh));
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
                node.add_child(graph, child);
            });
        });

    // TODO: Create skins

    // Create scenes
    let scenes = format
        .json
        .scenes
        .iter_mut()
        .map(|s| {
            let mut scene = doc.create_scene(graph);
            let weight = scene.get_mut(graph);

            weight.name = s.name.take();
            weight.extras = s.extras.take();

            s.nodes.iter().for_each(|idx| {
                if let Some(node) = nodes.get(idx.value()) {
                    scene.add_node(graph, node);
                }
            });

            scene
        })
        .collect::<Vec<_>>();

    // Default scene
    if let Some(index) = format.json.scene {
        if let Some(scene) = scenes.get(index.value()) {
            doc.set_default_scene(graph, Some(scene));
        }
    }

    // TODO: Create animations

    Ok(doc)
}

#[cfg(test)]
mod tests {
    use gltf::json::{self, validation::USize64, Index};

    use crate::io::resolver::file_resolver::FileResolver;

    use super::*;

    #[tokio::test]
    async fn test_import() {
        let mut json = json::Root::default();

        json.buffers.push(json::buffer::Buffer {
            name: Some("MyBuffer".to_string()),
            byte_length: USize64(0),
            uri: None,
            extensions: None,
            extras: None,
        });

        json.buffer_views.push(json::buffer::View {
            name: Some("MyBufferView".to_string()),
            buffer: Index::new(0),
            byte_length: USize64(0),
            byte_offset: None,
            byte_stride: None,
            target: None,
            extensions: None,
            extras: None,
        });

        json.accessors.push(json::accessor::Accessor {
            name: Some("MyAccessor".to_string()),
            buffer_view: Some(Index::new(0)),
            byte_offset: None,
            component_type: Checked::Valid(json::accessor::GenericComponentType(
                json::accessor::ComponentType::U8,
            )),
            count: USize64(0),
            extensions: None,
            extras: None,
            max: None,
            min: None,
            normalized: false,
            sparse: None,
            type_: Checked::Valid(json::accessor::Type::Scalar),
        });

        json.meshes.push(json::mesh::Mesh {
            name: Some("MyMesh".to_string()),
            primitives: vec![json::mesh::Primitive {
                attributes: Default::default(),
                extensions: None,
                extras: None,
                indices: None,
                material: None,
                mode: Checked::Valid(json::mesh::Mode::Triangles),
                targets: None,
            }],
            weights: None,
            extensions: None,
            extras: None,
        });

        json.nodes.push(json::scene::Node {
            name: Some("MyNode".to_string()),
            mesh: Some(Index::new(0)),
            camera: None,
            children: None,
            skin: None,
            matrix: None,
            rotation: None,
            scale: None,
            translation: None,
            weights: None,
            extensions: None,
            extras: None,
        });

        json.scenes.push(json::scene::Scene {
            name: Some("MyScene".to_string()),
            nodes: vec![Index::new(0)],
            extras: None,
            extensions: None,
        });

        json.scene = Some(Index::new(0));

        let mut format = GltfFormat {
            json,
            ..Default::default()
        };

        let mut graph = Graph::new();

        let doc = import(&mut graph, &mut format, &mut None::<FileResolver>)
            .await
            .unwrap();

        assert_eq!(doc.scenes(&graph).len(), 1);
        assert_eq!(doc.default_scene(&graph), Some(doc.scenes(&graph)[0]));
        assert_eq!(doc.nodes(&graph).len(), 1);
        assert_eq!(doc.meshes(&graph).len(), 1);
        assert_eq!(doc.buffers(&graph).len(), 1);
        assert_eq!(doc.buffer_views(&graph).len(), 1);
        assert_eq!(doc.accessors(&graph).len(), 1);
    }
}
