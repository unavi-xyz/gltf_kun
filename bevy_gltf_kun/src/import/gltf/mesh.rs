use bevy::prelude::*;
use gltf_kun::document::GltfDocument;

use super::{document::BevyImportError, Gltf};

#[derive(Asset, Debug, TypePath)]
pub struct GltfMesh {}

pub fn import_meshes(doc: &mut GltfDocument, gltf: &mut Gltf) -> Result<(), BevyImportError> {
    Ok(())
}
