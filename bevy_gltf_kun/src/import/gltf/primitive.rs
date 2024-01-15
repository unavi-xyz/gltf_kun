use bevy::{prelude::*, render::mesh::VertexAttributeValues};
use gltf_kun::graph::gltf::{
    accessor::{colors::ReadColors, iter::AccessorIter, ComponentType, GetAccessorIterError, Type},
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
pub enum ImportPrimitiveError {
    #[error("Failed to get accessor iterator: {0}")]
    GetAccessorIterError(#[from] GetAccessorIterError),
    #[error("Unsupported attribute format: {0:?} {1:?}")]
    UnsupportedAttributeFormat(ComponentType, Type),
}

pub fn import_primitive(
    context: &mut ImportContext,
    p: &Primitive,
) -> Result<(), ImportPrimitiveError> {
    for (semantic, accessor) in p.attributes(&context.doc.0) {
        let (attribute, conversion) = match semantic {
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

        let buffer_view = match accessor.buffer_view(&context.doc.0) {
            Some(buffer_view) => buffer_view,
            None => {
                warn!("Primitive attribute BufferView not found");
                continue;
            }
        };

        let buffer = match buffer_view.buffer(&context.doc.0) {
            Some(buffer) => buffer,
            None => {
                warn!("Primitive attribute Buffer not found");
                continue;
            }
        };

        let iter = accessor.iter(&context.doc.0, &buffer_view, &buffer)?;

        let converted = match conversion {
            ConversionMode::Any => convert_any_values(iter)?,
            ConversionMode::Rgba => convert_rgba_values(iter)?,
            _ => todo!(),
        };
    }

    Ok(())
}

fn convert_any_values(iter: AccessorIter) -> Result<VertexAttributeValues, ImportPrimitiveError> {
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
        iter => Err(ImportPrimitiveError::UnsupportedAttributeFormat(
            iter.component_type(),
            iter.element_type(),
        )),
    }
}

fn convert_rgba_values(iter: AccessorIter) -> Result<VertexAttributeValues, ImportPrimitiveError> {
    match (iter, iter.normalized()) {
        (AccessorIter::U8x3(iter), true) => Ok(VertexAttributeValues::Float32x4(
            ReadColors::RgbU8(iter).into_rgba_f32().collect(),
        )),
        (AccessorIter::U16x3(iter), true) => Ok(VertexAttributeValues::Float32x4(
            ReadColors::RgbU16(iter).into_rgba_f32().collect(),
        )),
        (AccessorIter::F32x3(iter), false) => Ok(VertexAttributeValues::Float32x4(
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
