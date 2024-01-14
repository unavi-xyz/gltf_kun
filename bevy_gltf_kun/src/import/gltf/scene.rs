use bevy::prelude::*;
use gltf_kun::graph::gltf::{scene::Scene, GltfGraph};

use super::{
    document::{BevyImportError, ImportContext},
    node::import_node,
};

#[derive(Asset, Debug, TypePath)]
pub struct GltfScene {}

pub fn import_scenes(context: &mut ImportContext) -> Result<(), BevyImportError> {
    for scene in context.doc.scenes().iter() {
        let scene_label = scene_label(scene, &context.doc.0);

        for mut node in scene.nodes(&context.doc.0) {
            import_node(context, &mut node)?;
        }
    }

    Ok(())
}

fn scene_label(scene: &Scene, graph: &GltfGraph) -> Option<String> {
    scene
        .get(graph)
        .name
        .as_ref()
        .map(|n| format!("Scene/{}", n))
}
