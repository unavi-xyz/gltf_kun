use bevy::{asset::LoadContext, prelude::*};
use gltf_kun::document::GltfDocument;
use thiserror::Error;

use super::{scene::import_scene, Gltf};

#[derive(Debug, Error)]
pub enum BevyImportError {}

pub struct ImportContext<'a, 'b> {
    pub doc: &'a mut GltfDocument,
    pub gltf: &'a mut Gltf,
    pub load_context: &'a mut LoadContext<'b>,
}

pub fn import_gltf_document(
    mut doc: GltfDocument,
    load_context: &mut LoadContext<'_>,
) -> Result<Gltf, BevyImportError> {
    let mut gltf = Gltf {
        nodes: vec![Handle::default(); doc.nodes().len()],
        scenes: vec![Handle::default(); doc.scenes().len()],
        ..default()
    };

    let mut context = ImportContext {
        doc: &mut doc,
        gltf: &mut gltf,
        load_context,
    };

    for scene in context.doc.scenes() {
        import_scene(&mut context, scene)?;
    }

    Ok(gltf)
}
