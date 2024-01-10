use anyhow::Result;
use bevy::{prelude::*, render::render_resource::PrimitiveTopology};
use gltf_kun::graph::gltf::{
    accessor, mesh, node,
    primitive::{self, Semantic},
};

use super::{vertex_to_accessor::vertex_to_accessor, CachedMesh, ExportContext};

pub fn export_meshes(
    In(mut context): In<ExportContext>,
    mesh_assets: Res<Assets<Mesh>>,
    meshes: Query<(&Handle<Mesh>, Option<&Name>)>,
) -> ExportContext {
    context.doc.scenes().iter().for_each(|scene| {
        scene.nodes(&context.doc.0).iter().for_each(|node| {
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

    info!("meshes len: {}", meshes.iter().count());

    if meshes.contains(entity) {
        info!("meshes contains entity");
        primitive_ents.push(entity);
    }

    let mut children = node.children(&context.doc.0);

    children.retain(|child| {
        // Valid child nodes have no children of their own.
        if !node.children(&context.doc.0).is_empty() {
            return true;
        }

        // Valid child nodes have no transform.
        let weight = node.get(&context.doc.0);
        if weight.translation != glam::Vec3::ZERO
            || weight.rotation != glam::Quat::IDENTITY
            || weight.scale != glam::Vec3::ONE
        {
            return true;
        }

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

        // Remove the node, since it's now a primitive.
        context.doc.0.remove_node(cached.node.0);
        context.nodes.retain(|cached| cached.node != *child);

        false
    });

    info!("Primitive entities: {:?}", primitive_ents);

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
            return node.set_mesh(&mut context.doc.0, Some(&cached.mesh));
        }

        // Create new mesh.
        let mut mesh = mesh::Mesh::new(&mut context.doc.0);

        primitive_ents.iter().for_each(|ent| {
            let (handle, name) = meshes.get(*ent).unwrap();

            let weight = mesh.get_mut(&mut context.doc.0);
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

            match export_primitive(context, bevy_mesh) {
                Ok(primitive) => mesh.add_primitive(&mut context.doc.0, &primitive),
                Err(e) => {
                    error!("Error exporting primitive: {}", e);
                }
            }
        });

        context.meshes.push(CachedMesh { mesh, bevy_meshes });

        node.set_mesh(&mut context.doc.0, Some(&mesh));
    }

    // Continue down the tree
    children.iter().for_each(|child| {
        export_node_mesh(context, mesh_assets, meshes, *child);
    });
}

fn export_primitive(context: &mut ExportContext, mesh: &Mesh) -> Result<primitive::Primitive> {
    let mut primitive = primitive::Primitive::new(&mut context.doc.0);
    let weight = primitive.get_mut(&mut context.doc.0);

    weight.mode = match mesh.primitive_topology() {
        PrimitiveTopology::LineList => primitive::Mode::Lines,
        PrimitiveTopology::LineStrip => primitive::Mode::LineStrip,
        PrimitiveTopology::PointList => primitive::Mode::Points,
        PrimitiveTopology::TriangleList => primitive::Mode::Triangles,
        PrimitiveTopology::TriangleStrip => primitive::Mode::TriangleStrip,
    };

    mesh.attributes().for_each(|(id, values)| {
        let array = vertex_to_accessor(values);
        let accessor = accessor::Accessor::from_array(&mut context.doc.0, array, None);

        if id == Mesh::ATTRIBUTE_POSITION.id {
            primitive.set_attribute(&mut context.doc.0, &Semantic::Positions, Some(&accessor));
        }

        if id == Mesh::ATTRIBUTE_NORMAL.id {
            primitive.set_attribute(&mut context.doc.0, &Semantic::Normals, Some(&accessor));
        }

        if id == Mesh::ATTRIBUTE_UV_0.id {
            primitive.set_attribute(&mut context.doc.0, &Semantic::TexCoords(0), Some(&accessor));
        }

        if id == Mesh::ATTRIBUTE_UV_1.id {
            primitive.set_attribute(&mut context.doc.0, &Semantic::TexCoords(1), Some(&accessor));
        }

        if id == Mesh::ATTRIBUTE_COLOR.id {
            primitive.set_attribute(&mut context.doc.0, &Semantic::Colors(0), Some(&accessor));
        }

        if id == Mesh::ATTRIBUTE_TANGENT.id {
            primitive.set_attribute(&mut context.doc.0, &Semantic::Tangents, Some(&accessor));
        }

        if id == Mesh::ATTRIBUTE_JOINT_INDEX.id {
            primitive.set_attribute(&mut context.doc.0, &Semantic::Joints(0), Some(&accessor));
        }

        if id == Mesh::ATTRIBUTE_JOINT_WEIGHT.id {
            primitive.set_attribute(&mut context.doc.0, &Semantic::Weights(0), Some(&accessor));
        }
    });

    Ok(primitive)
}
