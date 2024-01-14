use bevy::prelude::*;
use gltf_kun::{
    document::GltfDocument,
    graph::gltf::{scene::Scene, GltfGraph},
};

use super::{document::BevyImportError, node::import_node, Gltf};

#[derive(Asset, Debug, TypePath)]
pub struct GltfScene {}

pub fn import_scenes(doc: &mut GltfDocument, gltf: &mut Gltf) -> Result<(), BevyImportError> {
    for scene in doc.scenes().iter() {
        let scene_label = scene_label(scene, &doc.0);

        for node in scene.nodes(&doc.0) {
            import_node(doc, gltf, &node)?;
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
