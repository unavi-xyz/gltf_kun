use bevy::prelude::*;
use gltf_kun::graph::{gltf::node, GraphNodeWeight};

use super::{primitive::export_primitive, CachedMesh, ExportContext};

pub fn export_meshes(
    In(mut context): In<ExportContext>,
    mesh_assets: Res<Assets<Mesh>>,
    meshes: Query<(&Handle<Mesh>, Option<&Name>)>,
) -> ExportContext {
    context.doc.scenes(&context.graph).iter().for_each(|scene| {
        scene.nodes(&context.graph).iter().for_each(|node| {
            export_node_mesh(&mut context, &mesh_assets, &meshes, *node);
        })
    });

    context
}

fn export_node_mesh(
    context: &mut ExportContext,
    mesh_assets: &Res<Assets<Mesh>>,
    meshes: &Query<(&Handle<Mesh>, Option<&Name>)>,
    node: node::Node,
) {
    let cached = context
        .nodes
        .iter()
        .find(|cached| cached.node == node)
        .unwrap();

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
        // Valid child nodes have no children of their own.
        if !node.children(&context.graph).is_empty() {
            return true;
        }

        // Valid child nodes have no transform.
        let weight = node.get(&context.graph);
        if weight.translation != glam::Vec3::ZERO
            || weight.rotation != glam::Quat::IDENTITY
            || weight.scale != glam::Vec3::ONE
        {
            return true;
        }

        // Valid child nodes have a mesh.
        let cached = context
            .nodes
            .iter()
            .find(|cached| cached.node == *child)
            .unwrap();

        if !meshes.contains(cached.entity) {
            return true;
        }

        // Child is a valid primitive.
        primitive_ents.push(cached.entity);

        // Remove the node, since it is now a primitive.
        context.graph.remove_node(cached.node.0);
        context.nodes.retain(|cached| cached.node != *child);

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

        primitive_ents.iter().for_each(|ent| {
            let (handle, name) = meshes.get(*ent).unwrap();

            let weight = mesh.get_mut(&mut context.graph);
            if weight.name.is_none() {
                weight.name = name.map(|name| name.to_string());
            }

            let bevy_mesh = match mesh_assets.get(handle) {
                Some(mesh) => mesh,
                None => {
                    error!("Mesh not found: {:?}", handle);
                    return;
                }
            };

            let primitive = export_primitive(context, bevy_mesh);
            mesh.add_primitive(&mut context.graph, &primitive);
        });

        context.meshes.push(CachedMesh { mesh, bevy_meshes });

        node.set_mesh(&mut context.graph, Some(mesh));
    }

    // Continue down the tree
    children.iter().for_each(|child| {
        export_node_mesh(context, mesh_assets, meshes, *child);
    });
}
