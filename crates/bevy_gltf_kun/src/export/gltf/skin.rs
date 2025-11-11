use bevy::{
    mesh::skinning::{SkinnedMesh, SkinnedMeshInverseBindposes},
    prelude::*,
};
use gltf_kun::graph::{
    GraphNodeWeight,
    gltf::accessor::{ComponentType, Type},
};

use super::ExportContext;

pub fn export_skins(
    In(mut ctx): In<ExportContext>,
    inverse_bindposes: Res<Assets<SkinnedMeshInverseBindposes>>,
    skinned_meshes: Query<(Entity, &SkinnedMesh, Option<&Name>)>,
) -> ExportContext {
    for (entity, mesh, name) in skinned_meshes.iter() {
        let mut skin = ctx.doc.create_skin(&mut ctx.graph);
        let weight = skin.get_mut(&mut ctx.graph);

        if let Some(name) = name {
            weight.name = Some(name.to_string());
        }

        mesh.joints
            .iter()
            .filter_map(|joint| ctx.nodes.iter().find(|n| n.entity == *joint))
            .enumerate()
            .for_each(|(i, cached)| {
                skin.add_joint(&mut ctx.graph, &cached.node, i);
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

        let mut accessor = ctx.doc.create_accessor(&mut ctx.graph);
        skin.set_inverse_bind_matrices(&mut ctx.graph, Some(accessor));

        let buffer = ctx.doc.create_buffer(&mut ctx.graph);
        accessor.set_buffer(&mut ctx.graph, Some(buffer));

        let accessor_weight = accessor.get_mut(&mut ctx.graph);
        accessor_weight.component_type = ComponentType::F32;
        accessor_weight.element_type = Type::Mat4;
        accessor_weight.data = data;

        let mut found_node = false;

        // Find which node this skinned mesh is attached to.
        // `entity` is a gltf primitive.
        for cached_node in &ctx.nodes {
            let mesh = match cached_node.node.mesh(&ctx.graph) {
                Some(mesh) => mesh,
                None => continue,
            };

            let cached_mesh = ctx
                .meshes
                .iter()
                .find(|cached| cached.mesh == mesh)
                .unwrap();

            for (ent, _) in &cached_mesh.primitives {
                if *ent == entity {
                    cached_node.node.set_skin(&mut ctx.graph, Some(skin));
                    found_node = true;
                }
            }
        }

        if !found_node {
            warn!("Node not found for skinned mesh: {:?}", entity);
        }
    }

    ctx
}
