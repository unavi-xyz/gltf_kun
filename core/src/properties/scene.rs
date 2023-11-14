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

    fn data(&self) -> SceneData {
        match self.node.data() {
            GraphData::Scene(data) => data,
            _ => panic!("data is not a scene"),
        }
    }

    fn set_data(&mut self, data: SceneData) {
        self.node.set_data(GraphData::Scene(data));
    }

    pub fn name(&self) -> Option<String> {
        self.data().name
    }

    pub fn set_name(&mut self, name: Option<String>) {
        let mut data = self.data();
        data.name = name;
        self.set_data(data);
    }

    pub fn nodes(&self) -> Vec<Node> {
        children(&self.node.graph, self.node.index)
    }

    pub fn add_node(&mut self, child: &mut Node) {
        add_child(&self.node.graph, self.node.index, child);
    }
}
