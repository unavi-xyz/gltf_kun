use bevy::prelude::*;
use gltf_kun::graph::{
    gltf::{self},
    GraphNodeWeight,
};



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
    entity: &mut EntityWorldMut,
    mut m: gltf::mesh::Mesh,
    is_scale_inverted: bool,
) -> (Vec<Entity>, Handle<GltfMesh>, Option<Vec<f32>>) {
    let index = context.doc.mesh_index(context.graph, m).unwrap();
    let mesh_label = mesh_label(index);

    let mut primitive_entities = Vec::new();
    let mut primitives = Vec::new();

    let mut morph_weights = None;

    entity.with_children(|parent| {
        for (i, p) in m.primitives(context.graph).iter_mut().enumerate() {
            match import_primitive(context, parent, is_scale_inverted, m, &mesh_label, i, p) {
                Ok((ent, handle, weights)) => {
                    morph_weights = weights;
                    primitive_entities.push(ent);
                    primitives.push(handle)
                }
                Err(e) => {
                    warn!("Failed to import primitive: {}", e);
                    continue;
                }
            }
        }
    });

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

    context.gltf.meshes.insert(index, handle.clone());

    (primitive_entities, handle, morph_weights)
}

pub fn mesh_label(index: usize) -> String {
    format!("Mesh{}", index)
}
