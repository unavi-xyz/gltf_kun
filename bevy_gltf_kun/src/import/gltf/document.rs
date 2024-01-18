use bevy::{asset::LoadContext, prelude::*};
use gltf_kun::graph::{gltf::document::GltfDocument, Graph};
use thiserror::Error;

use super::{scene::import_scene, Gltf};

#[derive(Debug, Error)]
pub enum BevyImportError {}

pub struct ImportContext<'a, 'b> {
    pub graph: &'a mut Graph,
    pub doc: &'a mut GltfDocument,
    pub gltf: &'a mut Gltf,
    pub load_context: &'a mut LoadContext<'b>,
}

pub fn import_gltf_document(
    graph: &mut Graph,
    mut doc: GltfDocument,
    load_context: &mut LoadContext<'_>,
) -> Result<Gltf, BevyImportError> {
    let mut gltf = Gltf {
        nodes: vec![Handle::default(); doc.nodes(graph).len()],
        scenes: vec![Handle::default(); doc.scenes(graph).len()],
        meshes: vec![Handle::default(); doc.meshes(graph).len()],
        ..default()
    };

    let mut context = ImportContext {
        graph,
        doc: &mut doc,
        gltf: &mut gltf,
        load_context,
    };

    for scene in context.doc.scenes(context.graph) {
        import_scene(&mut context, scene)?;
    }

    Ok(gltf)
}
