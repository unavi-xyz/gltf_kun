use petgraph::graph::NodeIndex;
use std::sync::{Arc, Mutex};

use crate::{
    children::{add_child, children},
    graph::{GltfGraph, GraphData, GraphNode, NodeCover, NodeName, SceneData},
    node::Node,
};

pub struct Scene {
    node: GraphNode,
}

impl Scene {
    pub fn nodes(&self) -> Vec<Node> {
        children(&self.node.graph, self.node.index)
    }

    pub fn add_node(&mut self, child: &mut Node) {
        add_child(&self.node.graph, self.node.index, child);
    }
}

impl NodeCover for Scene {
    type Data = SceneData;

    fn new(graph: Arc<Mutex<GltfGraph>>, index: NodeIndex) -> Self {
        Self {
            node: GraphNode::new(graph, index),
        }
    }

    fn data(&self) -> Self::Data {
        match self.node.data() {
            GraphData::Scene(data) => data,
            _ => panic!("data is not a scene"),
        }
    }

    fn set_data(&mut self, data: Self::Data) {
        self.node.set_data(GraphData::Scene(data));
    }
}

impl NodeName for Scene {
    fn name(&self) -> Option<String> {
        self.data().name
    }

    fn set_name(&mut self, name: Option<String>) {
        let mut data = self.data();
        data.name = name;
        self.set_data(data);
    }
}
