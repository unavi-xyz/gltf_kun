use std::{cell::RefCell, rc::Rc};

use crate::{
    accessor::Accessor,
    attribute::Attribute,
    graph::{
        AttributeData, AttributeSemantic, GltfGraph, GraphData, GraphEdge, GraphNode, NodeCover,
        PrimitiveData,
    },
};
use petgraph::graph::{EdgeReference, NodeIndex};
use petgraph::visit::EdgeRef;

pub struct Primitive {
    pub(crate) node: GraphNode,
}

impl Primitive {
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
        match find_indices_edge(&self.node.graph.borrow(), self.node.index) {
            Some(edge) => Some(Accessor::new(self.node.graph.clone(), edge.target())),
            None => None,
        }
    }

    pub fn set_indices(&mut self, indices: Option<Accessor>) {
        let graph = self.node.graph.borrow();

        // Remove existing indices
        match find_indices_edge(&graph, self.node.index) {
            Some(edge) => self.node.graph.borrow_mut().remove_edge(edge.id()),
            None => None,
        };

        // Add new indices
        if let Some(indices) = indices {
            self.node.graph.borrow_mut().add_edge(
                self.node.index,
                indices.node.index,
                GraphEdge::Indices,
            );
        }
    }
}

impl NodeCover for Primitive {
    type Data = PrimitiveData;

    fn new(graph: Rc<RefCell<GltfGraph>>, index: NodeIndex) -> Self {
        Self {
            node: GraphNode::new(graph, index),
        }
    }

    fn data(&self) -> Self::Data {
        match self.node.data() {
            GraphData::Primitive(data) => data,
            _ => panic!("data is not a primitive"),
        }
    }

    fn set_data(&mut self, data: Self::Data) {
        self.node.set_data(GraphData::Primitive(data));
    }
}

fn find_indices_edge(graph: &GltfGraph, index: NodeIndex) -> Option<EdgeReference<GraphEdge>> {
    graph
        .edges_directed(index, petgraph::Direction::Outgoing)
        .find_map(|edge| match edge.weight() {
            GraphEdge::Indices => Some(edge),
            _ => return None,
        })
}
