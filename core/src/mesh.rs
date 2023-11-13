use std::sync::{Arc, Mutex};

use crate::{
    graph::{GltfGraph, GraphData, GraphEdge, GraphNode, MeshData, NodeCover, PrimitiveData},
    primitive::Primitive,
};
use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;

pub struct Mesh {
    pub(crate) node: GraphNode,
}

impl Mesh {
    pub fn primitives(&self) -> Vec<Primitive> {
        self.node
            .graph
            .lock()
            .unwrap()
            .edges_directed(self.node.index, petgraph::Direction::Outgoing)
            .filter_map(|edge| match edge.weight() {
                GraphEdge::Primitive => Some(edge.target()),
                _ => None,
            })
            .map(|index| Primitive::new(self.node.graph.clone(), index))
            .collect()
    }

    pub fn create_primitive(&mut self) -> Primitive {
        let mut graph = self.node.graph.lock().unwrap();
        let index = graph.add_node(GraphData::Primitive(PrimitiveData::default()));
        let primitive = Primitive::new(self.node.graph.clone(), index);
        graph.add_edge(self.node.index, index, GraphEdge::Primitive);
        primitive
    }
}

impl NodeCover for Mesh {
    type Data = MeshData;

    fn new(graph: Arc<Mutex<GltfGraph>>, index: NodeIndex) -> Self {
        Self {
            node: GraphNode::new(graph, index),
        }
    }

    fn data(&self) -> Self::Data {
        match self.node.data() {
            GraphData::Mesh(data) => data,
            _ => panic!("data is not a mesh"),
        }
    }

    fn set_data(&mut self, data: Self::Data) {
        self.node.set_data(GraphData::Mesh(data));
    }
}
