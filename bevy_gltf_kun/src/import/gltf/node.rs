use bevy::prelude::*;
use gltf_kun::graph::gltf::node::{Node, NodeWeight};

use super::{
    document::{BevyImportError, ImportContext},
    mesh::GltfMesh,
};

#[derive(Asset, Debug, TypePath)]
pub struct GltfNode {
    pub children: Vec<Handle<GltfNode>>,
    pub mesh: Option<Handle<GltfMesh>>,
    pub transform: Transform,
    pub extras: Option<Box<serde_json::value::RawValue>>,
}

pub fn import_node(
    context: &mut ImportContext<'_, '_>,
    n: &mut Node,
) -> Result<Handle<GltfNode>, BevyImportError> {
    let index = n.0.index();
    let weight = n.get_mut(&mut context.doc.0);

    let node_label = node_label(index, weight);
    let has_name = weight.name.is_some();

    let transform = Transform {
        translation: Vec3::from_array(weight.translation.to_array()),
        rotation: Quat::from_array(weight.rotation.to_array()),
        scale: Vec3::from_array(weight.scale.to_array()),
    };

    let mut node = GltfNode {
        mesh: None,
        children: Vec::new(),
        transform,
        extras: weight.extras.take(),
    };

    for mut c in n.children(&context.doc.0) {
        let child = import_node(context, &mut c)?;
        node.children.push(child);
    }

    let handle = context
        .load_context
        .add_labeled_asset(node_label.clone(), node);

    context.gltf.nodes.push(handle.clone());

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

    Ok(handle)
}

fn node_label(index: usize, weight: &NodeWeight) -> String {
    match weight.name.as_ref() {
        Some(n) => format!("Node/{}", n),
        None => format!("Node{}", index),
    }
}
