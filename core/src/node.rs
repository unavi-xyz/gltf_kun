use petgraph::graph::{EdgeReference, NodeIndex};
use std::sync::{Arc, Mutex};

use crate::{
    children::{add_child, children},
    graph::{GltfGraph, GraphData, GraphEdge, GraphNode, NodeCover, NodeData, NodeName},
    scene::Scene,
};
use petgraph::visit::EdgeRef;

pub enum NodeParent {
    Scene(Scene),
    Node(Node),
}

impl NodeParent {
    fn from_index(graph: Arc<Mutex<GltfGraph>>, index: NodeIndex) -> Option<Self> {
        let graph_lock = graph.lock().unwrap();
        match graph_lock[index] {
            GraphData::Scene(_) => Some(NodeParent::Scene(Scene::new(graph.clone(), index))),
            GraphData::Node(_) => Some(NodeParent::Node(Node::new(graph.clone(), index))),
            _ => None,
        }
    }
}

pub struct Node {
    pub(crate) node: GraphNode,
}

impl Node {
    fn find_parent_edge(graph: &GltfGraph, index: NodeIndex) -> Option<EdgeReference<GraphEdge>> {
        graph
            .edges_directed(index, petgraph::Direction::Incoming)
            .find_map(|edge| match edge.weight() {
                GraphEdge::Child => Some(edge),
                _ => return None,
            })
    }

    pub fn parent(&self) -> Option<NodeParent> {
        let graph = self.node.graph.lock().unwrap();

        let edge = match Self::find_parent_edge(&graph, self.node.index) {
            Some(edge) => edge,
            None => return None,
        };

        NodeParent::from_index(self.node.graph.clone(), edge.source())
    }

    pub fn remove_parent(&mut self) {
        let graph = self.node.graph.lock().unwrap();
        let edge = Self::find_parent_edge(&graph, self.node.index);

        if let Some(edge) = edge {
            let mut graph = self.node.graph.lock().unwrap();
            graph.remove_edge(edge.id());
        }
    }

    pub fn children(&self) -> Vec<Node> {
        children(&self.node.graph, self.node.index)
    }

    pub fn add_child(&mut self, child: &mut Node) {
        add_child(&self.node.graph, self.node.index, child);
    }
}

impl NodeCover for Node {
    type Data = NodeData;

    fn new(graph: Arc<Mutex<GltfGraph>>, index: NodeIndex) -> Self {
        Self {
            node: GraphNode::new(graph, index),
        }
    }

    fn data(&self) -> Self::Data {
        match self.node.data() {
            GraphData::Node(data) => data,
            _ => panic!("data is not a node"),
        }
    }

    fn set_data(&mut self, data: Self::Data) {
        self.node.set_data(GraphData::Node(data));
    }
}

impl NodeName for Node {
    fn name(&self) -> Option<String> {
        self.data().name
    }

    fn set_name(&mut self, name: Option<String>) {
        let mut data = self.data();
        data.name = name;
        self.set_data(data);
    }
}
