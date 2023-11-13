use petgraph::graph::NodeIndex;
use std::{cell::RefCell, rc::Rc};

use petgraph::visit::EdgeRef;

use crate::{
    graph::{GltfGraph, GraphEdge, NodeCover},
    node::Node,
};

pub fn children(graph: &Rc<RefCell<GltfGraph>>, index: NodeIndex) -> Vec<Node> {
    graph
        .borrow()
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

pub fn add_child(graph: &mut GltfGraph, parent: NodeIndex, child: &mut Node) {
    child.remove_parent();
    graph.add_edge(parent, child.node.index, GraphEdge::Child);
}
