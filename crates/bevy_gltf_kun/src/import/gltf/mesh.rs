use bevy::prelude::*;
use gltf_kun::graph::{
    GraphNodeWeight,
    gltf::{self, GltfDocument},
};

use crate::import::extensions::BevyExtensionImport;

use super::{
    document::ImportContext,
    primitive::{GltfPrimitive, import_primitive},
};

#[derive(Asset, Debug, TypePath)]
pub struct GltfMesh {
    pub primitives: Vec<GltfPrimitive>,
    pub extras: Option<Box<serde_json::value::RawValue>>,
}

pub fn import_mesh<E: BevyExtensionImport<GltfDocument>>(
    context: &mut ImportContext,
    entity: &mut EntityWorldMut,
    mut m: gltf::mesh::Mesh,
    is_scale_inverted: bool,
) -> (Vec<Entity>, Handle<GltfMesh>, Option<Vec<f32>>) {
    let index = context
        .doc
        .mesh_index(context.graph, m)
        .expect("index should exist for mesh");
    let mesh_label = mesh_label(index);

    let mut primitive_entities = Vec::new();
    let mut primitives = Vec::new();

    let mut morph_weights = None;

    entity.with_children(|parent| {
        for (i, p) in m.primitives(context.graph).iter_mut().enumerate() {
            if let Ok((ent, handle, weights)) =
                import_primitive::<E>(context, parent, is_scale_inverted, m, &mesh_label, i, p)
            {
                morph_weights = weights;
                primitive_entities.push(ent);
                primitives.push(handle);
            } else if let Err(e) =
                import_primitive::<E>(context, parent, is_scale_inverted, m, &mesh_label, i, p)
            {
                warn!("Failed to import primitive: {}", e);
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

#[must_use]
pub fn mesh_label(index: usize) -> String {
    format!("Mesh{index}")
}
