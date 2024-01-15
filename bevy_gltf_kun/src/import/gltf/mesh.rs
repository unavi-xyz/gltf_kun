use bevy::{prelude::*, render::render_resource::PrimitiveTopology};
use gltf_kun::graph::gltf::{self, primitive::Mode};

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
        let weight = primitive.get(&context.doc.0);

        let topology = match weight.mode {
            Mode::Lines => PrimitiveTopology::LineList,
            Mode::Points => PrimitiveTopology::PointList,
            Mode::LineStrip => PrimitiveTopology::LineStrip,
            Mode::Triangles => PrimitiveTopology::TriangleList,
            Mode::TriangleStrip => PrimitiveTopology::TriangleStrip,
            _ => {
                warn!("Unsupported primitive mode: {:?}", weight.mode);
                continue;
            }
        };

        let mut mesh = Mesh::new(topology);

        match import_primitive(context, &mut mesh, &primitive) {
            Ok(()) => (),
            Err(e) => {
                warn!("Failed to import primitive: {}", e);
                continue;
            }
        }
    }

    Ok(())
}
