use bevy::prelude::*;
use gltf_kun::graph::gltf::{self, mesh::MeshWeight};

use super::{document::ImportContext, primitive::import_primitive};

#[derive(Asset, Debug, TypePath)]
pub struct GltfMesh {}

pub fn import_mesh(
    context: &mut ImportContext,
    parent: &mut WorldChildBuilder,
    m: &mut gltf::mesh::Mesh,
) {
    let index = context.doc.meshes().iter().position(|x| x == m).unwrap();
    let weight = m.get_mut(&mut context.doc.0);

    let mesh_label = mesh_label(index, weight);

    for (i, primitive) in m.primitives(&context.doc.0).iter().enumerate() {
        match import_primitive(context, parent, &mesh_label, i, primitive) {
            Ok(()) => (),
            Err(e) => {
                warn!("Failed to import primitive: {}", e);
                continue;
            }
        }
    }
}

fn mesh_label(index: usize, weight: &MeshWeight) -> String {
    match weight.name.as_ref() {
        Some(n) => format!("Mesh/{}", n),
        None => format!("Mesh{}", index),
    }
}
