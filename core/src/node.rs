use std::{cell::RefCell, rc::Rc};

use petgraph::graph::{EdgeReference, NodeIndex};

use crate::{
    children::{add_child, children},
    graph::{GltfGraph, GraphData, GraphEdge, GraphNode, NodeCover, NodeData},
    mesh::Mesh,
    scene::Scene,
};
use petgraph::visit::EdgeRef;

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
    fn find_parent_edge(graph: &GltfGraph, index: NodeIndex) -> Option<EdgeReference<GraphEdge>> {
        graph
            .edges_directed(index, petgraph::Direction::Incoming)
            .find_map(|edge| match edge.weight() {
                GraphEdge::Child => Some(edge),
                _ => None,
            })
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
        find_mesh_edge(&self.node.graph.borrow(), self.node.index).map(|edge| Mesh::new(self.node.graph.clone(), edge.target()))
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

impl NodeCover for Node {
    type Data = NodeData;

    fn new(graph: Rc<RefCell<GltfGraph>>, index: NodeIndex) -> Self {
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

fn find_mesh_edge(graph: &GltfGraph, index: NodeIndex) -> Option<EdgeReference<GraphEdge>> {
    graph
        .edges_directed(index, petgraph::Direction::Outgoing)
        .find_map(|edge| match edge.weight() {
            GraphEdge::Mesh => Some(edge),
            _ => None,
        })
}
