use bevy::{prelude::*, render::mesh::skinning::SkinnedMeshInverseBindposes};
use gltf_kun::graph::gltf::{accessor::iter::AccessorIter, Skin};
use thiserror::Error;

use super::document::ImportContext;

#[derive(Debug, Error)]
pub enum ImportSkinError {
    #[error("Invalid accessor")]
    InvalidAccessor,
}

pub fn import_skin_matrices(
    context: &mut ImportContext,
    skin: Skin,
) -> Result<Handle<SkinnedMeshInverseBindposes>, ImportSkinError> {
    let iter = match skin.inverse_bind_matrices(context.graph) {
        Some(accessor) => match accessor.iter(context.graph) {
            Ok(AccessorIter::F32x16(iter)) => iter,
            _ => return Err(ImportSkinError::InvalidAccessor),
        },
        None => return Err(ImportSkinError::InvalidAccessor),
    };

    let matrices = iter.map(|m| Mat4::from_cols_array(&m)).collect::<Vec<_>>();

    let index = context.doc.skin_index(context.graph, skin).unwrap();

    Ok(context.load_context.add_labeled_asset(
        skin_label(index),
        SkinnedMeshInverseBindposes::from(matrices),
    ))
}

fn skin_label(index: usize) -> String {
    format!("Skin{}", index)
}
