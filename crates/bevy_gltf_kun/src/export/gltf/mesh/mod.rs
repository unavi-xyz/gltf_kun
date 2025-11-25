use bevy::prelude::*;
use gltf_kun::graph::{
    GraphNodeWeight,
    gltf::{Primitive, node},
};

use self::primitive::export_primitive;

use super::{CachedMesh, ExportContext};

mod primitive;
mod vertex_to_accessor;

#[must_use]
pub fn export_meshes(
    In(mut context): In<ExportContext>,
    mesh_assets: Res<Assets<Mesh>>,
    meshes: Query<(&Mesh3d, Option<&Name>)>,
) -> ExportContext {
    context.doc.scenes(&context.graph).iter().for_each(|scene| {
        scene.nodes(&context.graph).iter().for_each(|node| {
            export_node_mesh(&mut context, &mesh_assets, &meshes, *node);
        });
    });

    context
}

fn export_node_mesh(
    context: &mut ExportContext,
    mesh_assets: &Res<Assets<Mesh>>,
    meshes: &Query<(&Mesh3d, Option<&Name>)>,
    node: node::Node,
) {
    let cached = context
        .nodes
        .iter()
        .find(|cached| cached.node == node)
        .expect("value should exist");

    let entity = cached.entity;

    // Bevy meshes roughly correspond to glTF primitives,
    // so we need to find valid Bevy meshes to add as
    // primitives to our glTF mesh.
    let mut primitive_ents = Vec::new();

    if meshes.contains(entity) {
        primitive_ents.push(entity);
    }

    let mut children = node.children(&context.graph);

    children.retain(|child| {
        // Valid primitives have no children of their own.
        if !child.children(&context.graph).is_empty() {
            return true;
        }

        // Valid primitives have no transform.
        let weight = child.get(&context.graph);
        if weight.translation != Vec3::ZERO
            || weight.rotation != Quat::IDENTITY
            || weight.scale != Vec3::ONE
        {
            return true;
        }

        // Valid primitives have a Bevy mesh.
        let cached = context
            .nodes
            .iter()
            .find(|cached| cached.node == *child)
            .expect("value should exist");

        if !meshes.contains(cached.entity) {
            return true;
        }

        // Child is a valid primitive.
        primitive_ents.push(cached.entity);

        // Remove from nodes, since it is now a primitive.
        context.graph.remove_node(cached.node.0);
        context.nodes.retain(|c| c.node != *child);

        false
    });

    if !primitive_ents.is_empty() {
        let bevy_meshes = primitive_ents
            .iter()
            .filter_map(|ent| match meshes.get(*ent) {
                Ok((handle, _)) => Some(handle.clone()),
                Err(_) => None,
            })
            .collect::<Vec<_>>();

        // Check cache for existing glTF mesh using the same Bevy meshes.
        if let Some(cached) = context.meshes.iter().find(|cached| {
            bevy_meshes.len() == cached.bevy_meshes.len()
                && bevy_meshes
                    .iter()
                    .all(|handle| cached.bevy_meshes.contains(handle))
        }) {
            return node.set_mesh(&mut context.graph, Some(cached.mesh));
        }

        // Create new mesh.
        let mut mesh = context.doc.create_mesh(&mut context.graph);

        let primitives = primitive_ents
            .iter()
            .map(|ent| -> (Entity, Primitive) {
                let (handle, name) = meshes.get(*ent).expect("key should exist in map");

                let weight = mesh.get_mut(&mut context.graph);
                if weight.name.is_none() {
                    weight.name = name.map(std::string::ToString::to_string);
                }

                let bevy_mesh = mesh_assets.get(handle).expect("key should exist in map");

                let primitive = export_primitive(context, bevy_mesh);
                mesh.add_primitive(&mut context.graph, &primitive);

                (*ent, primitive)
            })
            .collect::<Vec<_>>();

        context.meshes.push(CachedMesh {
            mesh,
            primitives,
            bevy_meshes,
        });

        node.set_mesh(&mut context.graph, Some(mesh));
    }

    // Continue down the tree
    for child in children {
        export_node_mesh(context, mesh_assets, meshes, child);
    }
}
