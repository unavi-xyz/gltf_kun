use bevy::{prelude::*, utils::HashMap};
use gltf_kun::graph::{
    gltf::{document::GltfDocument, scene, Node},
    GraphNodeWeight,
};

use crate::import::extensions::BevyImportExtensions;

use super::{
    document::ImportContext,
    node::{import_node, node_name},
};

pub fn import_scene<E: BevyImportExtensions<GltfDocument>>(
    context: &mut ImportContext,
    animation_paths: &HashMap<Node, (Node, Vec<Name>)>,
    s: scene::Scene,
) -> Handle<Scene> {
    let mut world = World::default();

    world
        .spawn(SpatialBundle::INHERITED_IDENTITY)
        .with_children(|parent| {
            for mut node in s.nodes(context.graph) {
                import_node::<E>(context, parent, &mut node);
            }
        });

    for node in s.nodes(context.graph) {
        if is_animation_root(animation_paths, node) {
            let name = node_name(context.doc, context.graph, node);
            let handle = context.gltf.named_nodes.get(&name).unwrap();
            let entity = context.gltf.node_entities.get(handle).unwrap();

            world.entity_mut(*entity).insert(AnimationPlayer::default());
        }
    }

    let scene = Scene { world };

    let index = context.doc.scene_index(context.graph, s).unwrap();
    let weight = s.get(context.graph);
    let scene_label = scene_label(index);

    let handle = context
        .load_context
        .add_labeled_asset(scene_label.clone(), scene);

    if weight.name.is_some() {
        context
            .gltf
            .named_scenes
            .insert(scene_label.clone(), handle.clone());
    }

    handle
}

fn is_animation_root(paths: &HashMap<Node, (Node, Vec<Name>)>, node: Node) -> bool {
    paths.iter().any(|(_, (parent, _))| parent == &node)
}

fn scene_label(index: usize) -> String {
    format!("Scene{}", index)
}
