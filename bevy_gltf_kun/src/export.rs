use anyhow::Result;
use bevy::prelude::*;
use gltf_kun::{
    document::GltfDocument,
    graph::gltf::{buffer, buffer_view, mesh, node, primitive, scene, GltfGraph},
};

use crate::{Export, ExportResult};

pub fn export_gltf(
    mut events: ResMut<Events<Export<GltfDocument>>>,
    mut writer: EventWriter<ExportResult<GltfDocument>>,
    scenes: Query<Option<&Children>, With<Handle<Scene>>>,
    meshes: Query<(Entity, &Handle<Mesh>)>,
    nodes: Query<(&Transform, Option<&Children>)>,
    names: Query<&Name>,
) {
    for event in events.drain() {
        let mut doc = event.document;

        event.scenes.iter().for_each(|entity| {
            match export_scene(&mut doc.0, &names, &scenes, &nodes, *entity) {
                Ok(scene) => {
                    if let Some(default_scene) = event.default_scene {
                        if default_scene == *entity {
                            doc.set_default_scene(Some(&scene));
                        }
                    }
                }
                Err(e) => {
                    error!("Error exporting scene: {}", e);
                }
            }
        });

        writer.send(ExportResult { result: Ok(doc) });
    }
}

fn export_scene(
    graph: &mut GltfGraph,
    names: &Query<&Name>,
    scenes: &Query<Option<&Children>, With<Handle<Scene>>>,
    nodes: &Query<(&Transform, Option<&Children>)>,
    entity: Entity,
) -> Result<scene::Scene> {
    let mut scene = scene::Scene::new(graph);
    let weight = scene.get_mut(graph);

    if let Ok(name) = names.get(entity) {
        weight.name = Some(name.to_string());
    }

    let children = scenes.get(entity)?;

    if let Some(children) = children {
        children
            .iter()
            .for_each(|child| match export_node(graph, names, nodes, *child) {
                Ok(node) => scene.add_node(graph, &node),
                Err(e) => {
                    error!("Error exporting node: {}", e);
                }
            })
    }

    Ok(scene)
}

fn export_node(
    graph: &mut GltfGraph,
    names: &Query<&Name>,
    nodes: &Query<(&Transform, Option<&Children>)>,
    entity: Entity,
) -> Result<node::Node> {
    let mut node = node::Node::new(graph);
    let weight = node.get_mut(graph);

    let (transform, children) = nodes.get(entity)?;

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
