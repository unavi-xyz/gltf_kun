use petgraph::graph::NodeIndex;
use std::{cell::RefCell, rc::Rc};

use petgraph::visit::EdgeRef;

use crate::graph::{GltfGraph, GraphEdge, NodeCover};

use super::node::Node;

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

pub fn add_child(graph: &Rc<RefCell<GltfGraph>>, parent: NodeIndex, child: &mut Node) {
    child.remove_parent();
    let mut graph = graph.borrow_mut();
    graph.add_edge(parent, child.node.index, GraphEdge::Child);
}
