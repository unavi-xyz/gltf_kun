use petgraph::graph::NodeIndex;
use std::sync::{Arc, Mutex};

use petgraph::visit::EdgeRef;

use crate::{
    graph::{GltfGraph, GraphEdge, NodeCover},
    node::Node,
};

pub fn children(graph: &Arc<Mutex<GltfGraph>>, index: NodeIndex) -> Vec<Node> {
    graph
        .lock()
        .unwrap()
        .edges(index)
        .filter_map(|edge| {
            let index = match edge.weight() {
                GraphEdge::Child => edge.target(),
                _ => return None,
            };

            Some(Node::new(graph.clone(), index))
        })
        .collect()
}

pub fn add_child(graph: &Arc<Mutex<GltfGraph>>, parent: NodeIndex, child: &mut Node) {
    child.remove_parent();
    graph
        .lock()
        .unwrap()
        .add_edge(parent, child.node.index, GraphEdge::Child);
}
