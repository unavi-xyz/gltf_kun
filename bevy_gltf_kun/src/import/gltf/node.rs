use bevy::prelude::*;
use gltf_kun::{
    document::GltfDocument,
    graph::gltf::{node::Node, GltfGraph},
};

use super::{document::BevyImportError, Gltf};

#[derive(Asset, Debug, TypePath)]
pub struct GltfNode {}

pub fn import_node(
    doc: &mut GltfDocument,
    gltf: &mut Gltf,
    node: &Node,
) -> Result<(), BevyImportError> {
    let node_label = node_label(node, &doc.0);

    Ok(())
}

fn node_label(node: &Node, graph: &GltfGraph) -> Option<String> {
    node.get(graph).name.as_ref().map(|n| format!("Node/{}", n))
}
