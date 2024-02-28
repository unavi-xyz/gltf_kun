use bevy::prelude::*;
use gltf_kun::graph::{
    gltf::{GltfDocument, Node},
    Graph, GraphNodeWeight,
};

use crate::import::extensions::BevyImportExtensions;

use super::{
    document::ImportContext,
    mesh::{import_mesh, GltfMesh},
};

#[derive(Asset, Debug, TypePath)]
pub struct GltfNode {
    pub children: Vec<Handle<GltfNode>>,
    pub mesh: Option<Handle<GltfMesh>>,
    pub transform: Transform,
    pub extras: Option<Box<serde_json::value::RawValue>>,
}

pub fn import_node<E: BevyImportExtensions<GltfDocument>>(
    context: &mut ImportContext<'_, '_>,
    builder: &mut WorldChildBuilder,
    n: &mut Node,
) -> Handle<GltfNode> {
    let index = context.doc.node_index(context.graph, *n).unwrap();
    let weight = n.get_mut(context.graph);

    let extras = weight.extras.take();
    let transform = Transform {
        translation: Vec3::from_array(weight.translation.to_array()),
        rotation: Quat::from_array(weight.rotation.to_array()),
        scale: Vec3::from_array(weight.scale.to_array()),
    };

    let mut ent = builder.spawn(SpatialBundle::from_transform(transform));

    let name = node_name(context.doc, context.graph, *n);
    ent.insert(Name::new(name.clone()));

    let mut primitive_entities = Vec::new();

    let mesh = match n.mesh(context.graph) {
        Some(m) => {
            let (ents, mesh) = import_mesh(context, &mut ent, m);
            primitive_entities.extend(ents);
            Some(mesh)
        }
        None => None,
    };

    let mut children = Vec::new();

    ent.with_children(|parent| {
        n.children(context.graph).iter_mut().for_each(|c| {
            let handle = import_node::<E>(context, parent, c);
            children.push(handle)
        })
    });

    let node = GltfNode {
        mesh,
        children,
        transform,
        extras,
    };

    let node_label = node_label(index);
    let handle = context
        .load_context
        .add_labeled_asset(node_label.clone(), node);

    context.gltf.nodes.insert(index, handle.clone());
    context.gltf.named_nodes.insert(name, handle.clone());

    context.node_entities.insert(handle.clone(), ent.id());
    context.nodes_handles.insert(*n, handle.clone());
    context
        .node_primitive_entities
        .insert(handle.clone(), primitive_entities);

    E::import_node(context, &mut ent, *n);

    handle
}

pub fn node_name(doc: &GltfDocument, graph: &Graph, node: Node) -> String {
    let weight = node.get(graph);
    weight
        .name
        .clone()
        .unwrap_or_else(|| node_label(doc.node_index(graph, node).unwrap()))
}

pub fn node_label(index: usize) -> String {
    format!("Node{}", index)
}
