use bevy::prelude::*;
use gltf_kun::graph::gltf::{self, mesh::MeshWeight};

use super::{
    document::ImportContext,
    primitive::{import_primitive, GltfPrimitive},
};

#[derive(Asset, Debug, TypePath)]
pub struct GltfMesh {
    pub primitives: Vec<GltfPrimitive>,
    pub extras: Option<Box<serde_json::value::RawValue>>,
}

pub fn import_mesh(
    context: &mut ImportContext,
    parent: &mut WorldChildBuilder,
    m: &mut gltf::mesh::Mesh,
) {
    let index = context.doc.meshes().iter().position(|x| x == m).unwrap();
    let weight = m.get(&context.doc.0);
    let mesh_label = mesh_label(index, weight);

    let mut primitives = Vec::new();

    for (i, p) in m.primitives(&context.doc.0).iter_mut().enumerate() {
        match import_primitive(context, parent, &mesh_label, i, p) {
            Ok(handle) => primitives.push(handle),
            Err(e) => {
                warn!("Failed to import primitive: {}", e);
                continue;
            }
        }
    }

    let weight = m.get_mut(&mut context.doc.0);

    let mesh = GltfMesh {
        primitives,
        extras: weight.extras.take(),
    };

    let handle = context.load_context.add_labeled_asset(mesh_label, mesh);

    if let Some(name) = &weight.name {
        context
            .gltf
            .named_meshes
            .insert(name.clone(), handle.clone());
    }

    context.gltf.meshes.push(handle);
}

fn mesh_label(index: usize, weight: &MeshWeight) -> String {
    match weight.name.as_ref() {
        Some(n) => format!("Mesh/{}", n),
        None => format!("Mesh{}", index),
    }
}
