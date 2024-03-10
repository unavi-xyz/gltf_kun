use bevy::{prelude::*, render::mesh::skinning::SkinnedMesh, utils::HashSet};
use gltf_kun::graph::{
    gltf::{document::GltfDocument, scene, Node},
    GraphNodeWeight,
};

use crate::import::extensions::BevyImportExtensions;

use super::{
    document::ImportContext,
    node::{import_node, node_name},
};

const MAX_JOINTS: usize = 256;

pub fn import_scene<E: BevyImportExtensions<GltfDocument>>(
    context: &mut ImportContext,
    animation_roots: &HashSet<Node>,
    s: scene::Scene,
) -> Handle<Scene> {
    let mut world = World::default();

    world
        .spawn(SpatialBundle::INHERITED_IDENTITY)
        .with_children(|parent| {
            for mut node in s.nodes(context.graph) {
                import_node::<E>(context, parent, &Transform::default(), &mut node);
            }
        });

    for node in context.doc.nodes(context.graph) {
        if animation_roots.contains(&node) {
            let name = node_name(context.doc, context.graph, node);
            let handle = context.gltf.named_nodes.get(&name).unwrap();
            let entity = context.node_entities.get(handle).unwrap();
            let mut entity = world.entity_mut(*entity);
            entity.insert(AnimationPlayer::default());
        }

        if let Some(skin) = node.skin(context.graph) {
            let inverse_bindposes = context.skin_matrices.get(&skin).unwrap();

            let joints = skin
                .joints(context.graph)
                .iter()
                .map(|joint| {
                    let handle = context.nodes_handles.get(joint).unwrap();
                    *context.node_entities.get(handle).unwrap()
                })
                .collect::<Vec<_>>();

            if joints.len() > MAX_JOINTS {
                warn!(
                    "Skin has too many joints ({}), maximum is {}",
                    joints.len(),
                    MAX_JOINTS
                );
            }

            let handle = context.nodes_handles.get(&node).unwrap();
            let primitive_ents = context.node_primitive_entities.get(handle).unwrap();

            for entity in primitive_ents {
                let mut entity = world.entity_mut(*entity);
                entity.insert(SkinnedMesh {
                    inverse_bindposes: inverse_bindposes.clone(),
                    joints: joints.clone(),
                });
            }
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

fn scene_label(index: usize) -> String {
    format!("Scene{}", index)
}
