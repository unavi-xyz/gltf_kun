use std::{cell::RefCell, rc::Rc};

use crate::{
    accessor::Accessor,
    graph::{AttributeData, GltfGraph, GraphData, GraphEdge, GraphNode, NodeCover},
};
use petgraph::graph::{EdgeReference, NodeIndex};
use petgraph::visit::EdgeRef;

pub struct Attribute {
    pub(crate) node: GraphNode,
}

impl Attribute {
    pub fn accessor(&self) -> Option<Accessor> {
        find_accessor_edge(&self.node.graph.borrow(), self.node.index).map(|edge| Accessor::new(self.node.graph.clone(), edge.target()))
    }

    pub fn set_accessor(&mut self, accessor: Option<Accessor>) {
        let mut graph = self.node.graph.borrow_mut();

        // Remove existing accessor
        match find_accessor_edge(&graph, self.node.index) {
            Some(edge) => self.node.graph.borrow_mut().remove_edge(edge.id()),
            None => None,
        };

        // Add new accessor
        if let Some(accessor) = accessor {
            graph.add_edge(self.node.index, accessor.node.index, GraphEdge::Accessor);
        }
    }
}

impl NodeCover for Attribute {
    type Data = AttributeData;

    fn new(graph: Rc<RefCell<GltfGraph>>, index: NodeIndex) -> Self {
        Self {
            node: GraphNode::new(graph, index),
        }
    }

    fn data(&self) -> Self::Data {
        match self.node.data() {
            GraphData::Attribute(data) => data,
            _ => panic!("data is not an attribute"),
        }
    }

    fn set_data(&mut self, data: Self::Data) {
        self.node.set_data(GraphData::Attribute(data));
    }
}

fn find_accessor_edge(graph: &GltfGraph, index: NodeIndex) -> Option<EdgeReference<GraphEdge>> {
    graph
        .edges_directed(index, petgraph::Direction::Outgoing)
        .find_map(|edge| match edge.weight() {
            GraphEdge::Accessor => Some(edge),
            _ => None,
        })
}