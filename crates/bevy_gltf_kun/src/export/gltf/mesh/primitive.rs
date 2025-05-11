use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};
use gltf_kun::graph::{
    GraphNodeWeight,
    gltf::{
        accessor::{self, ComponentType, Type, iter::AccessorIter},
        primitive::{self, Semantic},
    },
};

use super::{ExportContext, vertex_to_accessor::vertex_to_accessor};

pub fn export_primitive(context: &mut ExportContext, mesh: &Mesh) -> primitive::Primitive {
    let mut primitive = primitive::Primitive::new(&mut context.graph);
    let weight = primitive.get_mut(&mut context.graph);

    weight.mode = match mesh.primitive_topology() {
        PrimitiveTopology::LineList => primitive::Mode::Lines,
        PrimitiveTopology::LineStrip => primitive::Mode::LineStrip,
        PrimitiveTopology::PointList => primitive::Mode::Points,
        PrimitiveTopology::TriangleList => primitive::Mode::Triangles,
        PrimitiveTopology::TriangleStrip => primitive::Mode::TriangleStrip,
    };

    if mesh.attributes().count() == 0 && mesh.indices().is_none() {
        return primitive;
    }

    let buffer = context.doc.create_buffer(&mut context.graph);

    if let Some(indices) = mesh.indices() {
        let bytes = match indices {
            Indices::U32(indices) => indices
                .iter()
                .flat_map(|v| v.to_le_bytes())
                .collect::<Vec<_>>(),
            Indices::U16(indices) => indices
                .iter()
                .flat_map(|v| v.to_le_bytes())
                .collect::<Vec<_>>(),
        };

        let iter = match indices {
            Indices::U32(_) => {
                AccessorIter::new(bytes.as_slice(), ComponentType::U32, Type::Scalar, false)
            }
            Indices::U16(_) => {
                AccessorIter::new(bytes.as_slice(), ComponentType::U16, Type::Scalar, false)
            }
        };

        let iter = match iter {
            Ok(iter) => iter,
            Err(err) => {
                error!("Failed to create indices accessor iterator: {}", err);
                return primitive;
            }
        };

        let accessor = accessor::Accessor::from_iter(&mut context.graph, iter);

        accessor.set_buffer(&mut context.graph, Some(buffer));
        context.doc.add_accessor(&mut context.graph, accessor);

        primitive.set_indices(&mut context.graph, Some(accessor));
    }

    mesh.attributes().for_each(|(attr, values)| {
        let id = attr.id;

        let accessor = match vertex_to_accessor(&mut context.graph, values) {
            Ok(accessor) => accessor,
            Err(err) => {
                error!(
                    "Failed to convert vertex attribute {:?} to accessor: {}",
                    id, err
                );
                return;
            }
        };

        accessor.set_buffer(&mut context.graph, Some(buffer));
        context.doc.add_accessor(&mut context.graph, accessor);

        if id == Mesh::ATTRIBUTE_POSITION.id {
            primitive.set_attribute(&mut context.graph, Semantic::Positions, Some(accessor));
        }

        if id == Mesh::ATTRIBUTE_NORMAL.id {
            primitive.set_attribute(&mut context.graph, Semantic::Normals, Some(accessor));
        }

        if id == Mesh::ATTRIBUTE_UV_0.id {
            primitive.set_attribute(&mut context.graph, Semantic::TexCoords(0), Some(accessor));
        }

        if id == Mesh::ATTRIBUTE_UV_1.id {
            primitive.set_attribute(&mut context.graph, Semantic::TexCoords(1), Some(accessor));
        }

        if id == Mesh::ATTRIBUTE_COLOR.id {
            primitive.set_attribute(&mut context.graph, Semantic::Colors(0), Some(accessor));
        }

        if id == Mesh::ATTRIBUTE_TANGENT.id {
            primitive.set_attribute(&mut context.graph, Semantic::Tangents, Some(accessor));
        }

        if id == Mesh::ATTRIBUTE_JOINT_INDEX.id {
            primitive.set_attribute(&mut context.graph, Semantic::Joints(0), Some(accessor));
        }

        if id == Mesh::ATTRIBUTE_JOINT_WEIGHT.id {
            primitive.set_attribute(&mut context.graph, Semantic::Weights(0), Some(accessor));
        }
    });

    primitive
}
