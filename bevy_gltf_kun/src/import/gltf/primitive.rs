use bevy::prelude::*;
use gltf_kun::graph::gltf::primitive::{Primitive, Semantic};

use super::document::{BevyImportError, ImportContext};

#[derive(Asset, Debug, TypePath)]
pub struct GltfPrimitive {}

enum ConversionMode {
    Any,
    Rgba,
    JointIndex,
    JointWeight,
    TexCoord,
}

pub fn import_primitive(context: &mut ImportContext, p: &Primitive) -> Result<(), BevyImportError> {
    for (semantic, _accessor) in p.attributes(&context.doc.0) {
        let (_attribute, _conversion) = match semantic {
            Semantic::Normals => (Mesh::ATTRIBUTE_NORMAL, ConversionMode::Any),
            Semantic::Positions => (Mesh::ATTRIBUTE_POSITION, ConversionMode::Any),
            Semantic::Tangents => (Mesh::ATTRIBUTE_TANGENT, ConversionMode::Any),
            Semantic::Colors(0) => (Mesh::ATTRIBUTE_COLOR, ConversionMode::Rgba),
            Semantic::TexCoords(0) => (Mesh::ATTRIBUTE_UV_0, ConversionMode::TexCoord),
            Semantic::TexCoords(1) => (Mesh::ATTRIBUTE_UV_1, ConversionMode::TexCoord),
            Semantic::Joints(0) => (Mesh::ATTRIBUTE_JOINT_INDEX, ConversionMode::JointIndex),
            Semantic::Weights(0) => (Mesh::ATTRIBUTE_JOINT_WEIGHT, ConversionMode::JointWeight),
            _ => continue,
        };
    }

    Ok(())
}
