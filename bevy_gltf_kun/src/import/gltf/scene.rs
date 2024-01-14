use bevy::prelude::*;
use gltf_kun::graph::gltf::{self, scene::SceneWeight};

use super::{
    document::{BevyImportError, ImportContext},
    node::import_node,
};

pub fn import_scene(
    context: &mut ImportContext,
    s: gltf::scene::Scene,
) -> Result<(), BevyImportError> {
    let mut world = World::default();

    world
        .spawn(SpatialBundle::INHERITED_IDENTITY)
        .with_children(|parent| {
            for mut node in s.nodes(&context.doc.0) {
                if let Err(e) = import_node(context, parent, &mut node) {
                    warn!("Failed to import node: {}", e);
                }
            }
        });

    let scene = Scene { world };

    let index = s.0.index();
    let weight = s.get(&context.doc.0);
    let scene_label = scene_label(index, weight);

    let handle = context
        .load_context
        .add_labeled_asset(scene_label.clone(), scene);

    if weight.name.is_some() {
        if context.gltf.named_scenes.contains_key(&scene_label) {
            warn!(
                "Duplicate scene name: {}. May cause issues if using name-based resolution.",
                scene_label
            );
        } else {
            context
                .gltf
                .named_scenes
                .insert(scene_label.clone(), handle.clone());
        }
    }

    context.gltf.scenes.push(handle);

    Ok(())
}

fn scene_label(index: usize, weight: &SceneWeight) -> String {
    match weight.name.as_ref() {
        Some(n) => format!("Scene/{}", n),
        None => format!("Scene{}", index),
    }
}
