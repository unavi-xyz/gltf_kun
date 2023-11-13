use std::sync::{Arc, Mutex};

use crate::graph::{AccessorData, GltfGraph, GraphData, GraphNode, NodeCover};
use petgraph::graph::NodeIndex;

pub struct Accessor {
    pub(crate) node: GraphNode,
}

impl Accessor {}

impl NodeCover for Accessor {
    type Data = AccessorData;

    fn new(graph: Arc<Mutex<GltfGraph>>, index: NodeIndex) -> Self {
        Self {
            node: GraphNode::new(graph, index),
        }
    }

    fn data(&self) -> Self::Data {
        match self.node.data() {
            GraphData::Accessor(data) => data,
            _ => panic!("data is not a attribute"),
        }
    }

    fn set_data(&mut self, data: Self::Data) {
        self.node.set_data(GraphData::Accessor(data));
    }
}
