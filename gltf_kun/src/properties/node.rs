use std::{cell::RefCell, rc::Rc};

use petgraph::graph::{EdgeReference, NodeIndex};

use crate::graph::{GltfGraph, GraphData, GraphEdge, GraphNode, NodeData};
use petgraph::visit::EdgeRef;

use super::{
    mesh::Mesh,
    scene::Scene,
    util::{add_child, children},
};

pub enum NodeParent {
    Scene(Scene),
    Node(Node),
}

impl NodeParent {
    fn from_index(graph: &Rc<RefCell<GltfGraph>>, index: NodeIndex) -> Option<Self> {
        match graph.borrow()[index] {
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
    pub fn new(graph: Rc<RefCell<GltfGraph>>, index: NodeIndex) -> Self {
        Self {
            node: GraphNode::new(graph, index),
        }
    }

    fn data(&self) -> NodeData {
        match self.node.data() {
            GraphData::Node(data) => data,
            _ => panic!("data is not a node"),
        }
    }

    fn set_data(&mut self, data: NodeData) {
        self.node.set_data(GraphData::Node(data));
    }

    fn find_parent_edge(graph: &GltfGraph, index: NodeIndex) -> Option<EdgeReference<GraphEdge>> {
        graph
            .edges_directed(index, petgraph::Direction::Incoming)
            .find(|edge| matches!(edge.weight(), GraphEdge::Child))
    }

    pub fn name(&self) -> Option<String> {
        self.data().name
    }

    pub fn set_name(&mut self, name: Option<String>) {
        let mut data = self.data();
        data.name = name;
        self.set_data(data);
    }

    pub fn translation(&self) -> [f32; 3] {
        self.data().translation
    }

    pub fn set_translation(&mut self, translation: [f32; 3]) {
        let mut data = self.data();
        data.translation = translation;
        self.set_data(data);
    }

    pub fn rotation(&self) -> [f32; 4] {
        self.data().rotation
    }

    pub fn set_rotation(&mut self, rotation: [f32; 4]) {
        let mut data = self.data();
        data.rotation = rotation;
        self.set_data(data);
    }

    pub fn scale(&self) -> [f32; 3] {
        self.data().scale
    }

    pub fn set_scale(&mut self, scale: [f32; 3]) {
        let mut data = self.data();
        data.scale = scale;
        self.set_data(data);
    }

    pub fn parent(&self) -> Option<NodeParent> {
        let graph = self.node.graph.borrow();
        let edge = match Self::find_parent_edge(&graph, self.node.index) {
            Some(edge) => edge,
            None => return None,
        };

        NodeParent::from_index(&self.node.graph, edge.source())
    }

    pub fn remove_parent(&mut self) {
        let graph = self.node.graph.borrow();
        let edge = Self::find_parent_edge(&graph, self.node.index);

        if let Some(edge) = edge {
            self.node.graph.borrow_mut().remove_edge(edge.id());
        }
    }

    pub fn children(&self) -> Vec<Node> {
        children(&self.node.graph, self.node.index)
    }

    pub fn add_child(&mut self, child: &mut Node) {
        add_child(&self.node.graph, self.node.index, child);
    }

    pub fn mesh(&self) -> Option<Mesh> {
        find_mesh_edge(&self.node.graph.borrow(), self.node.index)
            .map(|edge| Mesh::new(self.node.graph.clone(), edge.target()))
    }

    pub fn set_mesh(&mut self, mesh: Option<Mesh>) {
        // Remove existing mesh
        match find_mesh_edge(&self.node.graph.borrow(), self.node.index) {
            Some(edge) => self.node.graph.borrow_mut().remove_edge(edge.id()),
            None => None,
        };

        // Add new mesh
        if let Some(mesh) = mesh {
            self.node.graph.borrow_mut().add_edge(
                self.node.index,
                mesh.node.index,
                GraphEdge::Mesh,
            );
        }
    }
}

fn find_mesh_edge(graph: &GltfGraph, index: NodeIndex) -> Option<EdgeReference<GraphEdge>> {
    graph
        .edges_directed(index, petgraph::Direction::Outgoing)
        .find(|edge| matches!(edge.weight(), GraphEdge::Mesh))
}
