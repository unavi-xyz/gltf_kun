use bevy::{mesh::skinning::SkinnedMeshInverseBindposes, prelude::*};
use gltf_kun::graph::gltf::{
    Skin,
    accessor::{
        ComponentType, Type,
        iter::{AccessorIter, AccessorIterCreateError},
    },
};
use thiserror::Error;

use super::document::ImportContext;

#[derive(Debug, Error)]
pub enum ImportSkinError {
    #[error("missing inverse bind matrices")]
    MissingInverseBindMatrices,
    #[error("invalid accessor type: {0:?} {1:?}")]
    InvalidAccessorType(ComponentType, Type),
    #[error(transparent)]
    AccessorIterCreate(#[from] AccessorIterCreateError),
}

pub fn import_skin_matrices(
    context: &mut ImportContext,
    skin: Skin,
) -> Result<Handle<SkinnedMeshInverseBindposes>, ImportSkinError> {
    let iter = match skin.inverse_bind_matrices(context.graph) {
        Some(accessor) => match accessor.to_iter(context.graph) {
            Ok(AccessorIter::F32x16(iter)) => iter,
            Ok(a) => {
                return Err(ImportSkinError::InvalidAccessorType(
                    a.component_type(),
                    a.element_type(),
                ));
            }
            Err(e) => return Err(ImportSkinError::AccessorIterCreate(e)),
        },
        None => return Err(ImportSkinError::MissingInverseBindMatrices),
    };

    let matrices = iter.map(|m| Mat4::from_cols_array(&m)).collect::<Vec<_>>();

    let index = context
        .doc
        .skin_index(context.graph, skin)
        .expect("index should exist for skin");

    Ok(context.load_context.add_labeled_asset(
        skin_label(index),
        SkinnedMeshInverseBindposes::from(matrices),
    ))
}

fn skin_label(index: usize) -> String {
    format!("Skin{index}")
}
