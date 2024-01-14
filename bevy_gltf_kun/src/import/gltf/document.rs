use bevy::asset::LoadContext;
use gltf_kun::document::GltfDocument;
use thiserror::Error;

use super::{scene::import_scenes, Gltf};

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
    let mut gltf = Gltf::default();

    let mut context = ImportContext {
        doc: &mut doc,
        gltf: &mut gltf,
        load_context,
    };

    import_scenes(&mut context)?;

    Ok(gltf)
}
