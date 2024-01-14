use bevy::prelude::*;
use gltf_kun::document::GltfDocument;

use super::{document::BevyImportError, Gltf};

#[derive(Asset, Debug, TypePath)]
pub struct GltfMesh {}

pub fn import_meshes(_doc: &mut GltfDocument, _gltf: &mut Gltf) -> Result<(), BevyImportError> {
    Ok(())
}
