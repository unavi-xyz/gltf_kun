use bevy::{
    prelude::*,
    render::{
        mesh::{MeshVertexAttribute, VertexAttributeValues},
        render_resource::VertexFormat,
    },
};
use gltf_kun::graph::gltf::{
    accessor::{
        colors::ReadColors, iter::AccessorIter, joints::ReadJoints, tex_coords::ReadTexCoords,
        weights::ReadWeights, Accessor, ComponentType, GetAccessorIterError, Type,
    },
    primitive::{Primitive, Semantic},
};
use thiserror::Error;

use super::document::ImportContext;

#[derive(Asset, Debug, TypePath)]
pub struct GltfPrimitive {}

enum ConversionMode {
    Any,
    Rgba,
    JointIndex,
    JointWeight,
    TexCoord,
}

#[derive(Debug, Error)]
pub enum ImportPrimitiveError {}

pub fn import_primitive(
    context: &mut ImportContext,
    mesh: &mut Mesh,
    p: &Primitive,
) -> Result<(), ImportPrimitiveError> {
    for (semantic, accessor) in p.attributes(&context.doc.0) {
        let (attribute, values) = match convert_attribute(context, &semantic, &accessor) {
            Ok(a) => a,
            Err(err) => {
                warn!("Failed to convert attribute: {}", err);
                continue;
            }
        };

        mesh.insert_attribute(attribute, values);
    }

    Ok(())
}

#[derive(Debug, Error)]
enum AttributeConversionError {
    #[error("Buffer view not found for attribute {0:?}")]
    BufferViewNotFound(String),
    #[error("Buffer not found for attribute {0:?}")]
    BufferNotFound(String),
    #[error("Failed to get accessor iterator: {0}")]
    GetAccessorIterError(#[from] GetAccessorIterError),
    #[error("Unsupported attribute format: {0:?} {1:?}")]
    UnsupportedAttributeFormat(ComponentType, Type),
    #[error("Unsupported semantic: {0:?}")]
    UnsupportedSemantic(Semantic),
    #[error("Wrong format for attribute {0:?} (expected {1:?}, got {2:?})")]
    WrongFormat(String, VertexFormat, String, VertexFormat),
}

fn convert_attribute(
    context: &ImportContext,
    semantic: &Semantic,
    accessor: &Accessor,
) -> Result<(MeshVertexAttribute, VertexAttributeValues), AttributeConversionError> {
    let (attribute, conversion) = match semantic {
        Semantic::Normals => (Mesh::ATTRIBUTE_NORMAL, ConversionMode::Any),
        Semantic::Positions => (Mesh::ATTRIBUTE_POSITION, ConversionMode::Any),
        Semantic::Tangents => (Mesh::ATTRIBUTE_TANGENT, ConversionMode::Any),
        Semantic::Colors(0) => (Mesh::ATTRIBUTE_COLOR, ConversionMode::Rgba),
        Semantic::TexCoords(0) => (Mesh::ATTRIBUTE_UV_0, ConversionMode::TexCoord),
        Semantic::TexCoords(1) => (Mesh::ATTRIBUTE_UV_1, ConversionMode::TexCoord),
        Semantic::Joints(0) => (Mesh::ATTRIBUTE_JOINT_INDEX, ConversionMode::JointIndex),
        Semantic::Weights(0) => (Mesh::ATTRIBUTE_JOINT_WEIGHT, ConversionMode::JointWeight),
        _ => {
            return Err(AttributeConversionError::UnsupportedSemantic(
                semantic.clone(),
            ));
        }
    };

    let buffer_view = match accessor.buffer_view(&context.doc.0) {
        Some(buffer_view) => buffer_view,
        None => {
            return Err(AttributeConversionError::BufferViewNotFound(
                semantic.to_string(),
            ));
        }
    };

    let buffer = match buffer_view.buffer(&context.doc.0) {
        Some(buffer) => buffer,
        None => {
            return Err(AttributeConversionError::BufferNotFound(
                semantic.to_string(),
            ));
        }
    };

    let iter = accessor.iter(&context.doc.0, &buffer_view, &buffer)?;

    let values = match conversion {
        ConversionMode::Any => convert_any_values(iter)?,
        ConversionMode::Rgba => convert_rgba_values(iter)?,
        ConversionMode::JointIndex => convert_joint_index_values(iter)?,
        ConversionMode::JointWeight => convert_joint_weight_values(iter)?,
        ConversionMode::TexCoord => convert_tex_coord_values(iter)?,
    };

    let format = VertexFormat::from(&values);

    if attribute.format == format {
        Ok((attribute, values))
    } else {
        Err(AttributeConversionError::WrongFormat(
            semantic.to_string(),
            format,
            attribute.name.to_string(),
            attribute.format,
        ))
    }
}

/// Materializes values for any supported format of vertex attribute
fn convert_any_values(
    iter: AccessorIter,
) -> Result<VertexAttributeValues, AttributeConversionError> {
    match iter {
        AccessorIter::F32(iter) => Ok(VertexAttributeValues::Float32(iter.collect())),
        AccessorIter::U32(iter) => Ok(VertexAttributeValues::Uint32(iter.collect())),
        AccessorIter::F32x2(iter) => Ok(VertexAttributeValues::Float32x2(iter.collect())),
        AccessorIter::U32x2(iter) => Ok(VertexAttributeValues::Uint32x2(iter.collect())),
        AccessorIter::F32x3(iter) => Ok(VertexAttributeValues::Float32x3(iter.collect())),
        AccessorIter::U32x3(iter) => Ok(VertexAttributeValues::Uint32x3(iter.collect())),
        AccessorIter::F32x4(iter) => Ok(VertexAttributeValues::Float32x4(iter.collect())),
        AccessorIter::U32x4(iter) => Ok(VertexAttributeValues::Uint32x4(iter.collect())),
        AccessorIter::I16x2(iter) => {
            if iter.normalized {
                Ok(VertexAttributeValues::Snorm16x2(iter.collect()))
            } else {
                Ok(VertexAttributeValues::Sint16x2(iter.collect()))
            }
        }
        AccessorIter::U16x2(iter) => {
            if iter.normalized {
                Ok(VertexAttributeValues::Unorm16x2(iter.collect()))
            } else {
                Ok(VertexAttributeValues::Uint16x2(iter.collect()))
            }
        }
        AccessorIter::I16x4(iter) => {
            if iter.normalized {
                Ok(VertexAttributeValues::Snorm16x4(iter.collect()))
            } else {
                Ok(VertexAttributeValues::Sint16x4(iter.collect()))
            }
        }
        AccessorIter::U16x4(iter) => {
            if iter.normalized {
                Ok(VertexAttributeValues::Unorm16x4(iter.collect()))
            } else {
                Ok(VertexAttributeValues::Uint16x4(iter.collect()))
            }
        }
        AccessorIter::I8x2(iter) => {
            if iter.normalized {
                Ok(VertexAttributeValues::Snorm8x2(iter.collect()))
            } else {
                Ok(VertexAttributeValues::Sint8x2(iter.collect()))
            }
        }
        AccessorIter::U8x2(iter) => {
            if iter.normalized {
                Ok(VertexAttributeValues::Unorm8x2(iter.collect()))
            } else {
                Ok(VertexAttributeValues::Uint8x2(iter.collect()))
            }
        }
        AccessorIter::I8x4(iter) => {
            if iter.normalized {
                Ok(VertexAttributeValues::Snorm8x4(iter.collect()))
            } else {
                Ok(VertexAttributeValues::Sint8x4(iter.collect()))
            }
        }
        AccessorIter::U8x4(iter) => {
            if iter.normalized {
                Ok(VertexAttributeValues::Unorm8x4(iter.collect()))
            } else {
                Ok(VertexAttributeValues::Uint8x4(iter.collect()))
            }
        }
        iter => Err(AttributeConversionError::UnsupportedAttributeFormat(
            iter.component_type(),
            iter.element_type(),
        )),
    }
}

/// Materializes RGBA values, converting compatible formats to Float32x4
fn convert_rgba_values(
    iter: AccessorIter,
) -> Result<VertexAttributeValues, AttributeConversionError> {
    match (iter, iter.normalized()) {
        (AccessorIter::U8x3(iter), true) => Ok(VertexAttributeValues::Float32x4(
            ReadColors::RgbU8(iter).into_rgba_f32().collect(),
        )),
        (AccessorIter::U16x3(iter), true) => Ok(VertexAttributeValues::Float32x4(
            ReadColors::RgbU16(iter).into_rgba_f32().collect(),
        )),
        (AccessorIter::F32x3(iter), _) => Ok(VertexAttributeValues::Float32x4(
            ReadColors::RgbF32(iter).into_rgba_f32().collect(),
        )),
        (AccessorIter::U8x4(iter), true) => Ok(VertexAttributeValues::Float32x4(
            ReadColors::RgbaU8(iter).into_rgba_f32().collect(),
        )),
        (AccessorIter::U16x4(iter), true) => Ok(VertexAttributeValues::Float32x4(
            ReadColors::RgbaU16(iter).into_rgba_f32().collect(),
        )),
        (iter, _) => convert_any_values(iter),
    }
}

/// Materializes joint index values, converting compatible formats to Uint16x4
fn convert_joint_index_values(
    iter: AccessorIter,
) -> Result<VertexAttributeValues, AttributeConversionError> {
    match (iter, iter.normalized()) {
        (AccessorIter::U8x4(iter), false) => Ok(VertexAttributeValues::Uint16x4(
            ReadJoints::U8(iter).into_u16().collect(),
        )),
        (iter, _) => convert_any_values(iter),
    }
}

/// Materializes joint weight values, converting compatible formats to Float32x4
fn convert_joint_weight_values(
    iter: AccessorIter,
) -> Result<VertexAttributeValues, AttributeConversionError> {
    match (iter, iter.normalized()) {
        (AccessorIter::U8x4(iter), true) => Ok(VertexAttributeValues::Float32x4(
            ReadWeights::U8(iter).into_f32().collect(),
        )),
        (AccessorIter::U16x4(iter), true) => Ok(VertexAttributeValues::Float32x4(
            ReadWeights::U16(iter).into_f32().collect(),
        )),
        (iter, _) => convert_any_values(iter),
    }
}

/// Materializes texture coordinate values, converting compatible formats to Float32x2
fn convert_tex_coord_values(
    iter: AccessorIter,
) -> Result<VertexAttributeValues, AttributeConversionError> {
    match (iter, iter.normalized()) {
        (AccessorIter::U8x2(iter), true) => Ok(VertexAttributeValues::Float32x2(
            ReadTexCoords::U8(iter).into_f32().collect(),
        )),
        (AccessorIter::U16x2(iter), true) => Ok(VertexAttributeValues::Float32x2(
            ReadTexCoords::U16(iter).into_f32().collect(),
        )),
        (iter, _) => convert_any_values(iter),
    }
}
