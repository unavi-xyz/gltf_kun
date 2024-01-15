use bevy::prelude::*;
use gltf_kun::graph::gltf;

use super::{
    document::{BevyImportError, ImportContext},
    primitive::import_primitive,
};

#[derive(Asset, Debug, TypePath)]
pub struct GltfMesh {}

pub fn import_mesh(
    context: &mut ImportContext,
    m: &gltf::mesh::Mesh,
) -> Result<(), BevyImportError> {
    for primitive in m.primitives(&context.doc.0) {
        import_primitive(context, &primitive)?;
    }

    Ok(())
}
