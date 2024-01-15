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
        nodes: Vec::with_capacity(doc.nodes().len()),
        scenes: Vec::with_capacity(doc.scenes().len()),
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
