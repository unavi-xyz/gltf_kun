use std::{cell::RefCell, rc::Rc};

use crate::graph::{
    AttributeData, AttributeSemantic, GltfGraph, GraphData, GraphEdge, GraphNode, PrimitiveData,
    PrimitiveMode,
};

use super::accessor::Accessor;
use super::attribute::Attribute;

use petgraph::graph::{EdgeReference, NodeIndex};
use petgraph::visit::EdgeRef;

pub struct Primitive {
    pub(crate) node: GraphNode,
}

impl Primitive {
    pub fn new(graph: Rc<RefCell<GltfGraph>>, index: NodeIndex) -> Self {
        Self {
            node: GraphNode::new(graph, index),
        }
    }

    fn data(&self) -> PrimitiveData {
        match self.node.data() {
            GraphData::Primitive(data) => data,
            _ => panic!("data is not a primitive"),
        }
    }

    fn set_data(&mut self, data: PrimitiveData) {
        self.node.set_data(GraphData::Primitive(data));
    }

    pub fn mode(&self) -> PrimitiveMode {
        self.data().mode
    }

    pub fn set_mode(&mut self, mode: PrimitiveMode) {
        let mut data = self.data();
        data.mode = mode;
        self.set_data(data);
    }

    pub fn attributes(&self) -> Vec<Attribute> {
        self.node
            .graph
            .borrow()
            .edges_directed(self.node.index, petgraph::Direction::Outgoing)
            .filter_map(|edge| match edge.weight() {
                GraphEdge::Attribute => Some(edge.target()),
                _ => None,
            })
            .map(|index| Attribute::new(self.node.graph.clone(), index))
            .collect()
    }

    pub fn create_attribute(&mut self, semantic: AttributeSemantic) -> Attribute {
        let mut graph = self.node.graph.borrow_mut();
        let index = graph.add_node(GraphData::Attribute(AttributeData { semantic }));

        let attribute = Attribute::new(self.node.graph.clone(), index);
        graph.add_edge(self.node.index, index, GraphEdge::Attribute);

        attribute
    }

    pub fn indices(&self) -> Option<Accessor> {
        find_indices_edge(&self.node.graph.borrow(), self.node.index)
            .map(|edge| Accessor::new(self.node.graph.clone(), edge.target()))
    }

    pub fn set_indices(&mut self, indices: Option<Accessor>) {
        let mut graph = self.node.graph.borrow_mut();

        // Remove existing indices
        match find_indices_edge(&graph, self.node.index) {
            Some(edge) => self.node.graph.borrow_mut().remove_edge(edge.id()),
            None => None,
        };

        // Add new indices
        if let Some(indices) = indices {
            graph.add_edge(self.node.index, indices.node.index, GraphEdge::Indices);
        }
    }
}

fn find_indices_edge(graph: &GltfGraph, index: NodeIndex) -> Option<EdgeReference<GraphEdge>> {
    graph
        .edges_directed(index, petgraph::Direction::Outgoing)
        .find(|edge| matches!(edge.weight(), GraphEdge::Indices))
}
