use bevy::{
    prelude::*,
    render::mesh::skinning::{SkinnedMesh, SkinnedMeshInverseBindposes},
};
use gltf_kun::graph::{
    gltf::accessor::{ComponentType, Type},
    GraphNodeWeight,
};

use super::ExportContext;

pub fn export_skins(
    In(mut context): In<ExportContext>,
    inverse_bindposes: Res<Assets<SkinnedMeshInverseBindposes>>,
    skinned_meshes: Query<(Entity, &SkinnedMesh, Option<&Name>)>,
) -> ExportContext {
    for (entity, mesh, name) in skinned_meshes.iter() {
        let mut skin = context.doc.create_skin(&mut context.graph);
        let weight = skin.get_mut(&mut context.graph);

        if let Some(name) = name {
            weight.name = Some(name.to_string());
        }

        mesh.joints
            .iter()
            .filter_map(|joint| context.nodes.iter().find(|n| n.entity == *joint))
            .for_each(|cached| {
                skin.add_joint(&mut context.graph, &cached.node);
            });

        let inverse_bindposes_handle = &mesh.inverse_bindposes;
        let inverse_bindposes = inverse_bindposes.get(inverse_bindposes_handle).unwrap();
        let data = inverse_bindposes.iter().fold(Vec::new(), |mut data, m| {
            let array = m.to_cols_array();
            let bytes = array
                .map(|f| f.to_le_bytes())
                .to_vec()
                .iter()
                .flatten()
                .copied()
                .collect::<Vec<_>>();
            data.extend_from_slice(&bytes);
            data
        });

        let mut accessor = context.doc.create_accessor(&mut context.graph);
        skin.set_inverse_bind_matrices(&mut context.graph, Some(accessor));

        let buffer = context.doc.create_buffer(&mut context.graph);
        accessor.set_buffer(&mut context.graph, Some(buffer));

        let accessor_weight = accessor.get_mut(&mut context.graph);
        accessor_weight.component_type = ComponentType::F32;
        accessor_weight.element_type = Type::Mat4;
        accessor_weight.data = data;

        let mut found_node = false;

        // Find which node this skinned mesh is attached to.
        // `entity` is a gltf primitive.
        for cached_node in &context.nodes {
            let mesh = match cached_node.node.mesh(&context.graph) {
                Some(mesh) => mesh,
                None => continue,
            };

            let cached_mesh = context
                .meshes
                .iter()
                .find(|cached| cached.mesh == mesh)
                .unwrap();

            for (ent, _) in &cached_mesh.primitives {
                if *ent == entity {
                    cached_node.node.set_skin(&mut context.graph, Some(skin));
                    found_node = true;
                }
            }
        }

        if !found_node {
            warn!("Node not found for skinned mesh: {:?}", entity);
        }
    }

    context
}
