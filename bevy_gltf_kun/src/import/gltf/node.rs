use bevy::prelude::*;
use gltf_kun::graph::{
    gltf::{
        document::GltfDocument,
        node::{Node, NodeWeight},
    },
    GraphNode,
};

use crate::extensions::BevyImportExtensions;

use super::{
    document::{DocumentImportError, ImportContext},
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
) -> Result<Handle<GltfNode>, DocumentImportError> {
    let index = context
        .doc
        .nodes(context.graph)
        .iter()
        .position(|x| x == n)
        .unwrap();
    let weight = n.get_mut(context.graph);
    let node_label = node_label(index, weight);

    let has_name = weight.name.is_some();
    let extras = weight.extras.take();
    let transform = Transform {
        translation: Vec3::from_array(weight.translation.to_array()),
        rotation: Quat::from_array(weight.rotation.to_array()),
        scale: Vec3::from_array(weight.scale.to_array()),
    };

    let mut ent = builder.spawn(SpatialBundle::from_transform(transform));

    if let Some(name) = &weight.name {
        ent.insert(Name::new(name.clone()));
    }

    if let Some(ref mut mesh) = n.mesh(context.graph) {
        ent.with_children(|parent| import_mesh(context, parent, mesh));
    }

    let mut children = Vec::new();

    ent.with_children(|parent| {
        n.children(context.graph).iter_mut().for_each(|c| {
            match import_node::<E>(context, parent, c) {
                Ok(handle) => children.push(handle),
                Err(e) => warn!("Failed to import node: {}", e),
            }
        })
    });

    let node = GltfNode {
        mesh: None,
        children,
        transform,
        extras,
    };

    let handle = context
        .load_context
        .add_labeled_asset(node_label.clone(), node);

    context.gltf.nodes.insert(index, handle.clone());
    context.gltf.node_entities.insert(handle.clone(), ent.id());

    if has_name {
        if context.gltf.named_nodes.contains_key(&node_label) {
            warn!(
                "Duplicate node name: {}. May cause issues if using name-based resolution.",
                node_label
            );
        } else {
            context.gltf.named_nodes.insert(node_label, handle.clone());
        }
    }

    E::process_node(context, &mut ent, *n);

    Ok(handle)
}

fn node_label(index: usize, weight: &NodeWeight) -> String {
    match weight.name.as_ref() {
        Some(n) => format!("Node/{}", n),
        None => format!("Node{}", index),
    }
}
