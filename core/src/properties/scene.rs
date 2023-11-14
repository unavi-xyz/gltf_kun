use petgraph::graph::NodeIndex;
use std::{cell::RefCell, rc::Rc};

use crate::graph::{GltfGraph, GraphData, GraphNode, SceneData};

use super::{
    node::Node,
    util::{add_child, children},
};

pub struct Scene {
    node: GraphNode,
}

impl Scene {
    pub fn new(graph: Rc<RefCell<GltfGraph>>, index: NodeIndex) -> Self {
        Self {
            node: GraphNode::new(graph, index),
        }
    }

    pub fn data(&self) -> SceneData {
        match self.node.data() {
            GraphData::Scene(data) => data,
            _ => panic!("data is not a scene"),
        }
    }

    pub fn set_data(&mut self, data: SceneData) {
        self.node.set_data(GraphData::Scene(data));
    }

    pub fn nodes(&self) -> Vec<Node> {
        children(&self.node.graph, self.node.index)
    }

    pub fn add_node(&mut self, child: &mut Node) {
        add_child(&self.node.graph, self.node.index, child);
    }
}
