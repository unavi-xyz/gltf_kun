use bevy::{
    prelude::*,
    render::{
        mesh::{Indices, MeshVertexAttribute, VertexAttributeValues},
        primitives::Aabb,
        render_asset::RenderAssetUsages,
        render_resource::{PrimitiveTopology, VertexFormat},
    },
};
use gltf_kun::graph::{
    gltf::{
        accessor::{
            colors::ReadColors,
            indices::ReadIndices,
            iter::{AccessorElement, AccessorIter, AccessorIterCreateError, ElementIter},
            joints::ReadJoints,
            tex_coords::ReadTexCoords,
            weights::ReadWeights,
            Accessor, ComponentType, GetAccessorSliceError, Type,
        },
        primitive::{Mode, Primitive, Semantic},
        GltfDocument,
    },
    GraphNodeWeight,
};
use thiserror::Error;

use crate::import::extensions::BevyImportExtensions;

use super::{
    document::ImportContext,
    material::{default_material, import_material},
};

#[derive(Debug)]
pub struct GltfPrimitive {
    pub extras: Option<Box<serde_json::value::RawValue>>,
    pub material: Handle<StandardMaterial>,
    pub mesh: Handle<Mesh>,
}

enum ConversionMode {
    Any,
    Rgba,
    JointIndex,
    JointWeight,
    TexCoord,
}

#[derive(Debug, Error)]
pub enum ImportPrimitiveError {
    #[error("Failed to convert attribute: {0}")]
    ConvertAttribute(#[from] AttributeConversionError),
    #[error("Failed to read indices: {0}")]
    ReadIndices(#[from] ReadIndicesError),
    #[error("Unsupported primitive mode: {0:?}")]
    UnsupportedMode(Mode),
    #[error("Invalid accessor")]
    InvalidAccessor,
}

pub fn import_primitive<E: BevyImportExtensions<GltfDocument>>(
    context: &mut ImportContext,
    parent: &mut WorldChildBuilder,
    is_scale_inverted: bool,
    mesh_label: &str,
    index: usize,
    p: &mut Primitive,
) -> Result<(Entity, GltfPrimitive), ImportPrimitiveError> {
    let primitive_label = primitive_label(mesh_label, index);

    let weight = p.get(context.graph);

    let topology = match weight.mode {
        Mode::Lines => PrimitiveTopology::LineList,
        Mode::Points => PrimitiveTopology::PointList,
        Mode::LineStrip => PrimitiveTopology::LineStrip,
        Mode::Triangles => PrimitiveTopology::TriangleList,
        Mode::TriangleStrip => PrimitiveTopology::TriangleStrip,
        mode => return Err(ImportPrimitiveError::UnsupportedMode(mode)),
    };

    let mut mesh = Mesh::new(topology, RenderAssetUsages::default());

    for (semantic, accessor) in p.attributes(context.graph) {
        let (attribute, values) = convert_attribute(context, &semantic, &accessor)?;

        if values.is_empty() {
            warn!("Empty attribute: {}", semantic.to_string());
            continue;
        }

        mesh.insert_attribute(attribute, values);
    }

    if let Some(accessor) = p.indices(context.graph) {
        let indices = read_indices(context, accessor)?;
        mesh.insert_indices(indices);
    };

    let mesh = context
        .load_context
        .add_labeled_asset(primitive_label.clone(), mesh);

    let material = match p.material(context.graph) {
        Some(m) => {
            if let Some(material) = context.materials.get(&(m, is_scale_inverted)) {
                material.clone()
            } else {
                let material = import_material::<E>(context, m, is_scale_inverted);

                context
                    .materials
                    .insert((m, is_scale_inverted), material.clone());

                material
            }
        }
        None => default_material(context),
    };

    let primitive_handle = context.load_context.get_label_handle(&primitive_label);

    let mut entity = parent.spawn(PbrBundle {
        mesh: primitive_handle,
        material: material.clone(),
        ..default()
    });

    if let Some(pos) = p.attribute(context.graph, &Semantic::Positions) {
        let max = match pos.calc_max(context.graph) {
            Some(AccessorElement::F32x3(m)) => m,
            _ => return Err(ImportPrimitiveError::InvalidAccessor),
        };

        let min = match pos.calc_min(context.graph) {
            Some(AccessorElement::F32x3(m)) => m,
            _ => return Err(ImportPrimitiveError::InvalidAccessor),
        };

        entity.insert(Aabb::from_min_max(min.into(), max.into()));
    }

    let weight = p.get_mut(context.graph);

    let primitive = GltfPrimitive {
        extras: weight.extras.take(),
        material,
        mesh,
    };

    Ok((entity.id(), primitive))
}

fn primitive_label(mesh_label: &str, primitive_index: usize) -> String {
    format!("{}/Primitive{}", mesh_label, primitive_index)
}

#[derive(Debug, Error)]
pub enum ReadIndicesError {
    #[error("Failed to get accessor slice: {0}")]
    GetAccessorSliceError(#[from] GetAccessorSliceError),
}

fn read_indices(context: &ImportContext, indices: Accessor) -> Result<Indices, ReadIndicesError> {
    let weight = indices.get(context.graph);

    let read = match weight.component_type {
        ComponentType::U8 => ReadIndices::U8(ElementIter::<u8> {
            slice: &weight.data,
            normalized: weight.normalized,
            _phantom: default(),
        }),
        ComponentType::U16 => ReadIndices::U16(ElementIter::<u16> {
            slice: &weight.data,
            normalized: weight.normalized,
            _phantom: default(),
        }),
        ComponentType::U32 => ReadIndices::U32(ElementIter::<u32> {
            slice: &weight.data,
            normalized: weight.normalized,
            _phantom: default(),
        }),
        _ => unreachable!(),
    };

    Ok(match read {
        ReadIndices::U8(is) => Indices::U16(is.map(|x| x as u16).collect()),
        ReadIndices::U16(is) => Indices::U16(is.collect()),
        ReadIndices::U32(is) => Indices::U32(is.collect()),
    })
}

#[derive(Debug, Error)]
pub enum AttributeConversionError {
    #[error("Failed to create accessor iterator: {0}")]
    AccessorIter(#[from] AccessorIterCreateError),
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

    let iter = accessor.iter(context.graph)?;

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
