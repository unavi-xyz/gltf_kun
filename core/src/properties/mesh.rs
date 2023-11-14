use std::{cell::RefCell, rc::Rc};

use super::primitive::Primitive;
use crate::graph::{GltfGraph, GraphData, GraphEdge, GraphNode, MeshData, PrimitiveData};
use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;

pub struct Mesh {
    pub(crate) node: GraphNode,
}

impl Mesh {
    pub fn new(graph: Rc<RefCell<GltfGraph>>, index: NodeIndex) -> Self {
        Self {
            node: GraphNode::new(graph, index),
        }
    }

    fn data(&self) -> MeshData {
        match self.node.data() {
            GraphData::Mesh(data) => data,
            _ => panic!("data is not a mesh"),
        }
    }

    fn set_data(&mut self, data: MeshData) {
        self.node.set_data(GraphData::Mesh(data));
    }

    pub fn name(&self) -> Option<String> {
        self.data().name
    }

    pub fn set_name(&mut self, name: Option<String>) {
        let mut data = self.data();
        data.name = name;
        self.set_data(data);
    }

    pub fn primitives(&self) -> Vec<Primitive> {
        self.node
            .graph
            .borrow()
            .edges_directed(self.node.index, petgraph::Direction::Outgoing)
            .filter_map(|edge| match edge.weight() {
                GraphEdge::Primitive => Some(edge.target()),
                _ => None,
            })
            .map(|index| Primitive::new(self.node.graph.clone(), index))
            .collect()
    }

    pub fn create_primitive(&mut self) -> Primitive {
        let mut graph = self.node.graph.borrow_mut();
        let index = graph.add_node(GraphData::Primitive(PrimitiveData::default()));
        let primitive = Primitive::new(self.node.graph.clone(), index);
        graph.add_edge(self.node.index, index, GraphEdge::Primitive);
        primitive
    }
}
