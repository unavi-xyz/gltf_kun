use anyhow::Result;
use bevy::prelude::*;
use gltf_kun::{
    document::gltf::GltfDocument,
    graph::gltf::{buffer, buffer_view, mesh, node, primitive, scene, GltfGraph},
    io::format::ImportFormat,
};

use crate::{BevyFormat, Export, ExportResult};

impl ImportFormat<GltfDocument> for BevyFormat {
    fn import(self) -> Result<GltfDocument> {
        todo!()
    }
}

pub fn export_gltf(
    mut reader: EventReader<Export>,
    mut writer: EventWriter<ExportResult>,
    scenes: Query<(Entity, Option<&Children>), With<Handle<Scene>>>,
    nodes: Query<(Entity, &Transform, Option<&Children>)>,
    names: Query<&Name>,
) {
    for event in reader.read() {
        let mut doc = GltfDocument::default();

        event
            .scenes
            .iter()
            .filter_map(|entity| match scenes.get(*entity) {
                Ok(scene) => Some(scene),
                Err(_) => {
                    warn!("Scene not found: {:?}", entity);
                    None
                }
            })
            .for_each(|(entity, children)| {
                let mut scene = scene::Scene::new(&mut doc.0);
                let weight = scene.get_mut(&mut doc.0);

                if let Ok(name) = names.get(entity) {
                    weight.name = Some(name.to_string());
                }

                if let Some(children) = children {
                    children.iter().for_each(|child| {
                        match export_node(&mut doc.0, &names, &nodes, *child) {
                            Ok(node) => scene.add_node(&mut doc.0, &node),
                            Err(_) => {
                                warn!("Node not found: {:?}", child);
                            }
                        }
                    })
                }

                if let Some(default_scene) = event.default_scene {
                    if default_scene == entity {
                        doc.set_default_scene(Some(&scene));
                    }
                }
            });

        writer.send(ExportResult {
            result: Ok(Box::new(doc)),
        });
    }
}

fn export_node(
    graph: &mut GltfGraph,
    names: &Query<&Name>,
    nodes: &Query<(Entity, &Transform, Option<&Children>)>,
    entity: Entity,
) -> Result<node::Node> {
    let mut node = node::Node::new(graph);
    let weight = node.get_mut(graph);

    let (entity, transform, children) = nodes.get(entity)?;

    if let Ok(name) = names.get(entity) {
        weight.name = Some(name.to_string());
    }

    weight.translation = transform.translation.to_array().into();
    weight.rotation = glam::Quat::from_array(transform.rotation.to_array());
    weight.scale = transform.scale.to_array().into();

    if let Some(children) = children {
        children
            .iter()
            .for_each(|child| match export_node(graph, names, nodes, *child) {
                Ok(node) => node.add_child(graph, &node),
                Err(_) => {
                    warn!("Node not found: {:?}", child);
                }
            })
    }

    Ok(node)
}
