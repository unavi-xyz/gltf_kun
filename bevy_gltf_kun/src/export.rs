use std::collections::BTreeMap;

use anyhow::Result;
use bevy::prelude::*;
use gltf_kun::{
    document::GltfDocument,
    graph::gltf::{mesh, node, primitive, scene, GltfGraph},
};

use crate::{Export, ExportResult};

struct CachedMesh {
    mesh: mesh::Mesh,
    /// Corresponding Bevy mesh handles used to create this mesh.
    bevy_meshes: Vec<Handle<Mesh>>,
}

#[derive(Default)]
struct ExportContext {
    meshes: Vec<CachedMesh>,
}

pub fn export_gltf(
    mut events: ResMut<Events<Export<GltfDocument>>>,
    mut writer: EventWriter<ExportResult<GltfDocument>>,
    scenes: Query<Option<&Children>, With<Handle<Scene>>>,
    mesh_assets: Res<Assets<Mesh>>,
    meshes: Query<&Handle<Mesh>>,
    nodes: Query<(&Transform, Option<&Children>)>,
    names: Query<&Name>,
) {
    for event in events.drain() {
        let mut doc = event.document;

        let mut context = ExportContext::default();

        event.scenes.iter().for_each(|entity| {
            match export_scene(
                &mut doc.0,
                &mut context,
                &names,
                &scenes,
                &mesh_assets,
                &meshes,
                &nodes,
                *entity,
            ) {
                Ok(scene) => {
                    if let Some(default_scene) = event.default_scene {
                        if default_scene == *entity {
                            doc.set_default_scene(Some(&scene));
                        }
                    }
                }
                Err(e) => {
                    error!("Error exporting scene: {}", e);
                }
            }
        });

        writer.send(ExportResult { result: Ok(doc) });
    }
}

fn export_scene(
    graph: &mut GltfGraph,
    context: &mut ExportContext,
    names: &Query<&Name>,
    scenes: &Query<Option<&Children>, With<Handle<Scene>>>,
    mesh_assets: &Res<Assets<Mesh>>,
    meshes: &Query<&Handle<Mesh>>,
    nodes: &Query<(&Transform, Option<&Children>)>,
    entity: Entity,
) -> Result<scene::Scene> {
    let mut scene = scene::Scene::new(graph);
    let weight = scene.get_mut(graph);

    if let Ok(name) = names.get(entity) {
        weight.name = Some(name.to_string());
    }

    let children = scenes.get(entity)?;

    if let Some(children) = children {
        children.iter().for_each(|child| {
            match export_node(graph, context, names, mesh_assets, meshes, nodes, *child) {
                Ok(node) => scene.add_node(graph, &node),
                Err(e) => {
                    error!("Error exporting node: {}", e);
                }
            }
        })
    }

    Ok(scene)
}

fn export_node(
    graph: &mut GltfGraph,
    context: &mut ExportContext,
    names: &Query<&Name>,
    mesh_assets: &Res<Assets<Mesh>>,
    meshes: &Query<&Handle<Mesh>>,
    nodes: &Query<(&Transform, Option<&Children>)>,
    entity: Entity,
) -> Result<node::Node> {
    let mut node = node::Node::new(graph);
    let weight = node.get_mut(graph);

    let (transform, children) = nodes.get(entity)?;

    if let Ok(name) = names.get(entity) {
        weight.name = Some(name.to_string());
    }

    weight.translation = transform.translation.to_array().into();
    weight.rotation = glam::Quat::from_array(transform.rotation.to_array());
    weight.scale = transform.scale.to_array().into();

    let mut child_ents = BTreeMap::<node::Node, Entity>::new();

    if let Some(children) = children {
        children.iter().for_each(|child| {
            match export_node(graph, context, names, mesh_assets, meshes, nodes, *child) {
                Ok(node) => {
                    child_ents.insert(node, *child);
                    node.add_child(graph, &node)
                }
                Err(_) => {
                    warn!("Node not found: {:?}", child);
                }
            }
        })
    }

    if let Some(mesh) = export_mesh(
        graph,
        context,
        mesh_assets,
        meshes,
        names,
        &child_ents,
        entity,
    ) {
        node.set_mesh(graph, Some(&mesh));
    }

    Ok(node)
}

fn export_mesh(
    graph: &mut GltfGraph,
    context: &mut ExportContext,
    mesh_assets: &Res<Assets<Mesh>>,
    meshes: &Query<&Handle<Mesh>>,
    names: &Query<&Name>,
    child_ents: &BTreeMap<node::Node, Entity>,
    entity: Entity,
) -> Option<mesh::Mesh> {
    // Bevy meshes roughly correspond to glTF primitives,
    // so we need to find valid Bevy meshes to add as
    // primitives to our glTF mesh.
    let mut primitive_ents = Vec::new();

    if meshes.contains(entity) {
        primitive_ents.push(entity);
    }

    child_ents.iter().for_each(|(node, ent)| {
        // Valid child nodes have no children of their own.
        if !node.children(graph).is_empty() {
            return;
        }

        // Valid child nodes have no transform.
        let weight = node.get(graph);
        if weight.translation != glam::Vec3::ZERO
            || weight.rotation != glam::Quat::IDENTITY
            || weight.scale != glam::Vec3::ONE
        {
            return;
        }

        if meshes.contains(*ent) {
            primitive_ents.push(*ent);
        }
    });

    if primitive_ents.is_empty() {
        return None;
    }

    let bevy_meshes = primitive_ents
        .iter()
        .map(|ent| meshes.get(*ent).unwrap().clone())
        .collect::<Vec<_>>();

    // Check cache for existing glTF mesh using the same Bevy meshes.
    if let Some(cached) = context.meshes.iter().find(|cached| {
        bevy_meshes.len() == cached.bevy_meshes.len()
            && bevy_meshes
                .iter()
                .all(|mesh| cached.bevy_meshes.contains(mesh))
    }) {
        return Some(cached.mesh);
    }

    // Create new mesh.
    let mut mesh = mesh::Mesh::new(graph);
    let weight = mesh.get_mut(graph);

    if let Ok(name) = names.get(entity) {
        weight.name = Some(name.to_string());
    }

    primitive_ents.iter().for_each(|ent| {
        let handle = meshes.get(*ent).unwrap();
        let bevy_mesh = match mesh_assets.get(handle) {
            Some(mesh) => mesh,
            None => {
                error!("Mesh not found: {:?}", handle);
                return;
            }
        };

        match export_primitive(graph, context, bevy_mesh) {
            Ok(primitive) => mesh.add_primitive(graph, &primitive),
            Err(e) => {
                error!("Error exporting primitive: {}", e);
            }
        }
    });

    context.meshes.push(CachedMesh { mesh, bevy_meshes });

    Some(mesh)
}

fn export_primitive(
    graph: &mut GltfGraph,
    context: &mut ExportContext,
    mesh: &Mesh,
) -> Result<primitive::Primitive> {
    let mut primitive = primitive::Primitive::new(graph);
    let weight = primitive.get_mut(graph);

    Ok(primitive)
}
