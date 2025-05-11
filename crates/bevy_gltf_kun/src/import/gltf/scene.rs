use std::collections::HashMap;

use bevy::{platform::collections::HashSet, prelude::*, render::mesh::skinning::SkinnedMesh};
use gltf_kun::graph::{
    GraphNodeWeight,
    gltf::{Node, document::GltfDocument, scene},
};

use crate::import::extensions::BevyExtensionImport;

use super::{
    document::ImportContext,
    node::{GltfNode, import_node, node_name},
};

#[derive(Asset, Debug, TypePath)]
pub struct GltfScene {
    pub extras: Option<Box<serde_json::value::RawValue>>,
    pub nodes: Vec<Handle<GltfNode>>,
    pub scene: Handle<Scene>,
}

const MAX_JOINTS: usize = 256;

pub fn import_scene<E: BevyExtensionImport<GltfDocument>>(
    context: &mut ImportContext,
    animation_roots: &HashSet<Node>,
    s: scene::Scene,
) -> Handle<GltfScene> {
    let mut world = World::default();

    let mut node_entities = HashMap::<Handle<GltfNode>, Entity>::default();
    let mut node_primitive_entities = HashMap::<Handle<GltfNode>, Vec<Entity>>::default();
    let mut root_nodes = Vec::new();

    world
        .spawn((Transform::default(), Visibility::default()))
        .with_children(|parent| {
            for mut node in s.nodes(context.graph) {
                match import_node::<E>(
                    context,
                    &mut node_entities,
                    &mut node_primitive_entities,
                    parent,
                    &Transform::default(),
                    Vec::new(),
                    None,
                    &mut node,
                ) {
                    Ok(handle) => {
                        root_nodes.push(handle);
                    }
                    Err(e) => {
                        error!("Failed to import node: {}", e);
                    }
                }
            }
        });

    for node in context.doc.nodes(context.graph) {
        if animation_roots.contains(&node) {
            let name = node_name(context.doc, context.graph, node);
            let handle = context.gltf.named_nodes.get(&name).unwrap();
            let entity = node_entities.get(handle).unwrap();
            world.entity_mut(*entity).insert(AnimationPlayer::default());
        }

        if let Some(skin) = node.skin(context.graph) {
            let inverse_bindposes = context.skin_matrices.get(&skin).unwrap();

            let joints = skin
                .joints(context.graph)
                .iter()
                .map(|joint| {
                    let handle = context.gltf.node_handles.get(joint).unwrap();
                    *node_entities.get(handle).unwrap()
                })
                .collect::<Vec<_>>();

            if joints.len() > MAX_JOINTS {
                warn!(
                    "Skin has too many joints ({}), maximum is {}",
                    joints.len(),
                    MAX_JOINTS
                );
            }

            let handle = context.gltf.node_handles.get(&node).unwrap();
            let primitive_ents = node_primitive_entities.get(handle).unwrap();

            for entity in primitive_ents {
                world.entity_mut(*entity).insert(SkinnedMesh {
                    inverse_bindposes: inverse_bindposes.clone(),
                    joints: joints.clone(),
                });
            }
        }
    }

    // Load extensions.
    E::import_scene(context, s, &mut world);

    let scene = Scene { world };

    let index = context.doc.scene_index(context.graph, s).unwrap();
    let weight = s.get(context.graph);
    let scene_label = scene_label(index);

    let handle = context
        .load_context
        .add_labeled_asset(scene_label.clone(), scene);

    let gltf_scene = GltfScene {
        extras: weight.extras.clone(),
        nodes: root_nodes,
        scene: handle.clone(),
    };

    let gltf_scene_handle = context
        .load_context
        .add_labeled_asset(gltf_scene_label(index), gltf_scene);

    if weight.name.is_some() {
        context
            .gltf
            .named_scenes
            .insert(scene_label.clone(), gltf_scene_handle.clone());
    }

    gltf_scene_handle
}

fn scene_label(index: usize) -> String {
    format!("Scene{}", index)
}

fn gltf_scene_label(index: usize) -> String {
    format!("GltfScene{}", index)
}
