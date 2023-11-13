use std::{cell::RefCell, rc::Rc};

use crate::graph::{AccessorData, GltfGraph, GraphData, GraphNode, NodeCover};
use petgraph::graph::NodeIndex;

pub struct Accessor {
    pub(crate) node: GraphNode,
}

impl Accessor {}

impl NodeCover for Accessor {
    type Data = AccessorData;

    fn new(graph: Rc<RefCell<GltfGraph>>, index: NodeIndex) -> Self {
        Self {
            node: GraphNode::new(graph, index),
        }
    }

    fn data(&self) -> Self::Data {
        match self.node.data() {
            GraphData::Accessor(data) => data,
            _ => panic!("data is not an accessor"),
        }
    }

    fn set_data(&mut self, data: Self::Data) {
        self.node.set_data(GraphData::Accessor(data));
    }
}
