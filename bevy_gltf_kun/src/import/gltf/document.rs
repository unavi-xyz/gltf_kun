use gltf_kun::document::GltfDocument;
use thiserror::Error;

use super::{scene::import_scenes, Gltf};

#[derive(Debug, Error)]
pub enum BevyImportError {}

pub fn import_gltf_document(mut doc: GltfDocument) -> Result<Gltf, BevyImportError> {
    let mut gltf = Gltf::default();

    import_scenes(&mut doc, &mut gltf)?;

    Ok(gltf)
}
