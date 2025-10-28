use bevy::render::mesh::VertexAttributeValues;
use gltf_kun::graph::{
    Graph,
    gltf::accessor::{
        Accessor, ComponentType, Type,
        iter::{AccessorIter, AccessorIterCreateError},
    },
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum VertexToAccessorError {
    #[error("unsupported vertex attribute type {0:?}")]
    UnsupportedVertexAttributeType(VertexAttributeValues),
    #[error("failed to create accessor iterator: {0}")]
    IterCreateError(#[from] AccessorIterCreateError),
}

pub fn vertex_to_accessor(
    graph: &mut Graph,
    values: &VertexAttributeValues,
) -> Result<Accessor, VertexToAccessorError> {
    match values {
        VertexAttributeValues::Float32(values) => {
            let bytes = values
                .iter()
                .flat_map(|v| v.to_le_bytes())
                .collect::<Vec<u8>>();
            let iter = AccessorIter::new(&bytes, ComponentType::F32, Type::Scalar, false)?;
            Ok(Accessor::from_iter(graph, iter))
        }
        VertexAttributeValues::Float32x2(values) => {
            let bytes = values
                .iter()
                .flat_map(|v| v.map(|v| v.to_le_bytes()))
                .flatten()
                .collect::<Vec<u8>>();
            let iter = AccessorIter::new(&bytes, ComponentType::F32, Type::Vec2, false)?;
            Ok(Accessor::from_iter(graph, iter))
        }
        VertexAttributeValues::Float32x3(values) => {
            let bytes = values
                .iter()
                .flat_map(|v| v.map(|v| v.to_le_bytes()))
                .flatten()
                .collect::<Vec<u8>>();
            let iter = AccessorIter::new(&bytes, ComponentType::F32, Type::Vec3, false)?;
            Ok(Accessor::from_iter(graph, iter))
        }
        VertexAttributeValues::Float32x4(values) => {
            let bytes = values
                .iter()
                .flat_map(|v| v.map(|v| v.to_le_bytes()))
                .flatten()
                .collect::<Vec<u8>>();
            let iter = AccessorIter::new(&bytes, ComponentType::F32, Type::Vec4, false)?;
            Ok(Accessor::from_iter(graph, iter))
        }
        VertexAttributeValues::Uint32(values) => {
            let bytes = values
                .iter()
                .flat_map(|v| v.to_le_bytes())
                .collect::<Vec<u8>>();
            let iter = AccessorIter::new(&bytes, ComponentType::U32, Type::Scalar, false)?;
            Ok(Accessor::from_iter(graph, iter))
        }
        VertexAttributeValues::Uint32x2(values) => {
            let bytes = values
                .iter()
                .flat_map(|v| v.map(|v| v.to_le_bytes()))
                .flatten()
                .collect::<Vec<u8>>();
            let iter = AccessorIter::new(&bytes, ComponentType::U32, Type::Vec2, false)?;
            Ok(Accessor::from_iter(graph, iter))
        }
        VertexAttributeValues::Uint32x3(values) => {
            let bytes = values
                .iter()
                .flat_map(|v| v.map(|v| v.to_le_bytes()))
                .flatten()
                .collect::<Vec<u8>>();
            let iter = AccessorIter::new(&bytes, ComponentType::U32, Type::Vec3, false)?;
            Ok(Accessor::from_iter(graph, iter))
        }
        VertexAttributeValues::Uint32x4(values) => {
            let bytes = values
                .iter()
                .flat_map(|v| v.map(|v| v.to_le_bytes()))
                .flatten()
                .collect::<Vec<u8>>();
            let iter = AccessorIter::new(&bytes, ComponentType::U32, Type::Vec4, false)?;
            Ok(Accessor::from_iter(graph, iter))
        }
        VertexAttributeValues::Uint16x2(values) => {
            let bytes = values
                .iter()
                .flat_map(|v| v.map(|v| v.to_le_bytes()))
                .flatten()
                .collect::<Vec<u8>>();
            let iter = AccessorIter::new(&bytes, ComponentType::U16, Type::Vec2, false)?;
            Ok(Accessor::from_iter(graph, iter))
        }
        VertexAttributeValues::Uint16x4(values) => {
            let bytes = values
                .iter()
                .flat_map(|v| v.map(|v| v.to_le_bytes()))
                .flatten()
                .collect::<Vec<u8>>();
            let iter = AccessorIter::new(&bytes, ComponentType::U16, Type::Vec4, false)?;
            Ok(Accessor::from_iter(graph, iter))
        }
        VertexAttributeValues::Uint8x2(values) => {
            let bytes = values
                .iter()
                .flat_map(|v| v.map(|v| v.to_le_bytes()))
                .flatten()
                .collect::<Vec<u8>>();
            let iter = AccessorIter::new(&bytes, ComponentType::U8, Type::Vec2, false)?;
            Ok(Accessor::from_iter(graph, iter))
        }
        VertexAttributeValues::Uint8x4(values) => {
            let bytes = values
                .iter()
                .flat_map(|v| v.map(|v| v.to_le_bytes()))
                .flatten()
                .collect::<Vec<u8>>();
            let iter = AccessorIter::new(&bytes, ComponentType::U8, Type::Vec4, false)?;
            Ok(Accessor::from_iter(graph, iter))
        }
        VertexAttributeValues::Sint16x2(values) => {
            let bytes = values
                .iter()
                .flat_map(|v| v.map(|v| v.to_le_bytes()))
                .flatten()
                .collect::<Vec<u8>>();
            let iter = AccessorIter::new(&bytes, ComponentType::I16, Type::Vec2, false)?;
            Ok(Accessor::from_iter(graph, iter))
        }
        VertexAttributeValues::Sint16x4(values) => {
            let bytes = values
                .iter()
                .flat_map(|v| v.map(|v| v.to_le_bytes()))
                .flatten()
                .collect::<Vec<u8>>();
            let iter = AccessorIter::new(&bytes, ComponentType::I16, Type::Vec4, false)?;
            Ok(Accessor::from_iter(graph, iter))
        }
        VertexAttributeValues::Sint8x2(values) => {
            let bytes = values
                .iter()
                .flat_map(|v| v.map(|v| v.to_le_bytes()))
                .flatten()
                .collect::<Vec<u8>>();
            let iter = AccessorIter::new(&bytes, ComponentType::I8, Type::Vec2, false)?;
            Ok(Accessor::from_iter(graph, iter))
        }
        VertexAttributeValues::Sint8x4(values) => {
            let bytes = values
                .iter()
                .flat_map(|v| v.map(|v| v.to_le_bytes()))
                .flatten()
                .collect::<Vec<u8>>();
            let iter = AccessorIter::new(&bytes, ComponentType::I8, Type::Vec4, false)?;
            Ok(Accessor::from_iter(graph, iter))
        }
        v => Err(VertexToAccessorError::UnsupportedVertexAttributeType(
            v.to_owned(),
        )),
    }
}
