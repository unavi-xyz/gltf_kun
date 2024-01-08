use bevy::render::mesh::VertexAttributeValues;
use gltf_kun::graph::gltf::accessor::{AccessorArray, DataType, Type};

pub fn vertex_to_accessor(values: &VertexAttributeValues) -> AccessorArray {
    match values {
        VertexAttributeValues::Float32(values) => AccessorArray {
            element_type: Type::Scalar,
            data_type: DataType::F32,
            vec: values.iter().flat_map(|v| v.to_le_bytes()).collect(),
            normalized: false,
        },
        VertexAttributeValues::Float32x2(values) => AccessorArray {
            element_type: Type::Vec2,
            data_type: DataType::F32,
            vec: values
                .iter()
                .flat_map(|v| v.iter().flat_map(|v| v.to_le_bytes()))
                .collect(),
            normalized: false,
        },
        VertexAttributeValues::Float32x3(values) => AccessorArray {
            element_type: Type::Vec3,
            data_type: DataType::F32,
            vec: values
                .iter()
                .flat_map(|v| v.iter().flat_map(|v| v.to_le_bytes()))
                .collect(),
            normalized: false,
        },
        VertexAttributeValues::Float32x4(values) => AccessorArray {
            element_type: Type::Vec4,
            data_type: DataType::F32,
            vec: values
                .iter()
                .flat_map(|v| v.iter().flat_map(|v| v.to_le_bytes()))
                .collect(),
            normalized: false,
        },
        VertexAttributeValues::Uint32(values) => AccessorArray {
            element_type: Type::Scalar,
            data_type: DataType::U32,
            vec: values.iter().flat_map(|v| v.to_le_bytes()).collect(),
            normalized: false,
        },
        VertexAttributeValues::Uint32x2(values) => AccessorArray {
            element_type: Type::Vec2,
            data_type: DataType::U32,
            vec: values
                .iter()
                .flat_map(|v| v.iter().flat_map(|v| v.to_le_bytes()))
                .collect(),
            normalized: false,
        },
        VertexAttributeValues::Uint32x3(values) => AccessorArray {
            element_type: Type::Vec3,
            data_type: DataType::U32,
            vec: values
                .iter()
                .flat_map(|v| v.iter().flat_map(|v| v.to_le_bytes()))
                .collect(),
            normalized: false,
        },
        VertexAttributeValues::Uint32x4(values) => AccessorArray {
            element_type: Type::Vec4,
            data_type: DataType::U32,
            vec: values
                .iter()
                .flat_map(|v| v.iter().flat_map(|v| v.to_le_bytes()))
                .collect(),
            normalized: false,
        },
        VertexAttributeValues::Uint16x2(values) => AccessorArray {
            element_type: Type::Vec2,
            data_type: DataType::U16,
            vec: values
                .iter()
                .flat_map(|v| v.iter().flat_map(|v| v.to_le_bytes()))
                .collect(),
            normalized: false,
        },
        VertexAttributeValues::Uint16x4(values) => AccessorArray {
            element_type: Type::Vec4,
            data_type: DataType::U16,
            vec: values
                .iter()
                .flat_map(|v| v.iter().flat_map(|v| v.to_le_bytes()))
                .collect(),
            normalized: false,
        },
        VertexAttributeValues::Uint8x2(values) => AccessorArray {
            element_type: Type::Vec2,
            data_type: DataType::U8,
            vec: values
                .iter()
                .flat_map(|v| v.iter().flat_map(|v| v.to_le_bytes()))
                .collect(),
            normalized: false,
        },
        VertexAttributeValues::Uint8x4(values) => AccessorArray {
            element_type: Type::Vec4,
            data_type: DataType::U8,
            vec: values
                .iter()
                .flat_map(|v| v.iter().flat_map(|v| v.to_le_bytes()))
                .collect(),
            normalized: false,
        },
        VertexAttributeValues::Sint16x2(values) => AccessorArray {
            element_type: Type::Vec2,
            data_type: DataType::I16,
            vec: values
                .iter()
                .flat_map(|v| v.iter().flat_map(|v| v.to_le_bytes()))
                .collect(),
            normalized: false,
        },
        VertexAttributeValues::Sint16x4(values) => AccessorArray {
            element_type: Type::Vec4,
            data_type: DataType::I16,
            vec: values
                .iter()
                .flat_map(|v| v.iter().flat_map(|v| v.to_le_bytes()))
                .collect(),
            normalized: false,
        },
        VertexAttributeValues::Sint8x2(values) => AccessorArray {
            element_type: Type::Vec2,
            data_type: DataType::I8,
            vec: values
                .iter()
                .flat_map(|v| v.iter().flat_map(|v| v.to_le_bytes()))
                .collect(),
            normalized: false,
        },
        VertexAttributeValues::Sint8x4(values) => AccessorArray {
            element_type: Type::Vec4,
            data_type: DataType::I8,
            vec: values
                .iter()
                .flat_map(|v| v.iter().flat_map(|v| v.to_le_bytes()))
                .collect(),
            normalized: false,
        },
        VertexAttributeValues::Unorm16x2(values) => AccessorArray {
            element_type: Type::Vec2,
            data_type: DataType::U16,
            vec: values
                .iter()
                .flat_map(|v| v.iter().flat_map(|v| v.to_le_bytes()))
                .collect(),
            normalized: true,
        },
        VertexAttributeValues::Unorm16x4(values) => AccessorArray {
            element_type: Type::Vec4,
            data_type: DataType::U16,
            vec: values
                .iter()
                .flat_map(|v| v.iter().flat_map(|v| v.to_le_bytes()))
                .collect(),
            normalized: true,
        },
        VertexAttributeValues::Unorm8x2(values) => AccessorArray {
            element_type: Type::Vec2,
            data_type: DataType::U8,
            vec: values
                .iter()
                .flat_map(|v| v.iter().flat_map(|v| v.to_le_bytes()))
                .collect(),
            normalized: true,
        },
        VertexAttributeValues::Unorm8x4(values) => AccessorArray {
            element_type: Type::Vec4,
            data_type: DataType::U8,
            vec: values
                .iter()
                .flat_map(|v| v.iter().flat_map(|v| v.to_le_bytes()))
                .collect(),
            normalized: true,
        },
        VertexAttributeValues::Snorm16x2(values) => AccessorArray {
            element_type: Type::Vec2,
            data_type: DataType::I16,
            vec: values
                .iter()
                .flat_map(|v| v.iter().flat_map(|v| v.to_le_bytes()))
                .collect(),
            normalized: true,
        },
        VertexAttributeValues::Snorm16x4(values) => AccessorArray {
            element_type: Type::Vec4,
            data_type: DataType::I16,
            vec: values
                .iter()
                .flat_map(|v| v.iter().flat_map(|v| v.to_le_bytes()))
                .collect(),
            normalized: true,
        },
        VertexAttributeValues::Snorm8x2(values) => AccessorArray {
            element_type: Type::Vec2,
            data_type: DataType::I8,
            vec: values
                .iter()
                .flat_map(|v| v.iter().flat_map(|v| v.to_le_bytes()))
                .collect(),
            normalized: true,
        },
        VertexAttributeValues::Snorm8x4(values) => AccessorArray {
            element_type: Type::Vec4,
            data_type: DataType::I8,
            vec: values
                .iter()
                .flat_map(|v| v.iter().flat_map(|v| v.to_le_bytes()))
                .collect(),
            normalized: true,
        },
        _ => panic!("Unsupported vertex attribute type"),
    }
}
