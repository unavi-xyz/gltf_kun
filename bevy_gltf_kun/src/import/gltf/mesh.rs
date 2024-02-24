use bevy::prelude::*;
use gltf_kun::graph::{gltf, GraphNodeWeight};

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
    let index = context.doc.mesh_index(context.graph, m).unwrap();
    let mesh_label = mesh_label(index);

    let mut primitives = Vec::new();

    for (i, p) in m.primitives(context.graph).iter_mut().enumerate() {
        match import_primitive(context, parent, &mesh_label, i, p) {
            Ok(handle) => primitives.push(handle),
            Err(e) => {
                warn!("Failed to import primitive: {}", e);
                continue;
            }
        }
    }

    let weight = m.get_mut(context.graph);

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

    context.gltf.meshes.insert(index, handle);
}

fn mesh_label(index: usize) -> String {
    format!("Mesh{}", index)
}
