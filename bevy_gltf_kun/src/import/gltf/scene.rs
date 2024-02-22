use bevy::prelude::*;
use gltf_kun::graph::{
    gltf::{
        document::GltfDocument,
        scene::{self},
    },
    GraphNodeWeight,
};

use crate::import::extensions::BevyImportExtensions;

use super::{
    document::{DocumentImportError, ImportContext},
    node::import_node,
};

pub fn import_scene<E: BevyImportExtensions<GltfDocument>>(
    context: &mut ImportContext,
    s: scene::Scene,
) -> Result<Handle<Scene>, DocumentImportError> {
    let mut world = World::default();

    world
        .spawn(SpatialBundle::INHERITED_IDENTITY)
        .with_children(|parent| {
            for mut node in s.nodes(context.graph) {
                if let Err(e) = import_node::<E>(context, parent, &mut node) {
                    warn!("Failed to import node: {}", e);
                }
            }
        });

    let scene = Scene { world };

    let index = context
        .doc
        .scenes(context.graph)
        .iter()
        .position(|x| *x == s)
        .unwrap();
    let weight = s.get(context.graph);
    let scene_label = scene_label(index);

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

    Ok(handle)
}

fn scene_label(index: usize) -> String {
    format!("Scene{}", index)
}
