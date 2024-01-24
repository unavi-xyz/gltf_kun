use glam::{Quat, Vec3};
use petgraph::{graph::NodeIndex, visit::EdgeRef};

use crate::graph::{Edge, Graph, GraphNode, Property, Weight};

use super::{mesh::Mesh, GltfEdge, GltfWeight};

#[derive(Debug, PartialEq, Eq)]
pub enum NodeEdge {
    Child,
    Mesh,
}

#[derive(Debug)]
pub struct NodeWeight {
    pub name: Option<String>,
    pub extras: gltf::json::Extras,

    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Default for NodeWeight {
    fn default() -> Self {
        Self {
            name: None,
            extras: None,

            translation: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }
}

impl<'a> TryFrom<&'a Weight> for &'a NodeWeight {
    type Error = ();
    fn try_from(value: &'a Weight) -> Result<Self, Self::Error> {
        match value {
            Weight::Gltf(GltfWeight::Node(weight)) => Ok(weight),
            _ => Err(()),
        }
    }
}

impl<'a> TryFrom<&'a mut Weight> for &'a mut NodeWeight {
    type Error = ();
    fn try_from(value: &'a mut Weight) -> Result<Self, Self::Error> {
        match value {
            Weight::Gltf(GltfWeight::Node(weight)) => Ok(weight),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Node(pub NodeIndex);

impl From<NodeIndex> for Node {
    fn from(index: NodeIndex) -> Self {
        Self(index)
    }
}

impl From<Node> for NodeIndex {
    fn from(node: Node) -> Self {
        node.0
    }
}

impl GraphNode<NodeWeight> for Node {}
impl Property for Node {}

impl Node {
    pub fn new(graph: &mut Graph) -> Self {
        let index = graph.add_node(Weight::Gltf(GltfWeight::Node(NodeWeight::default())));
        Self(index)
    }

    pub fn children(&self, graph: &Graph) -> Vec<Node> {
        let mut vec = graph
            .edges_directed(self.0, petgraph::Direction::Outgoing)
            .filter_map(|edge| {
                if let Edge::Gltf(GltfEdge::Node(NodeEdge::Child)) = edge.weight() {
                    Some(Node(edge.target()))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        vec.sort();

        vec
    }
    pub fn add_child(&self, graph: &mut Graph, child: &Node) {
        graph.add_edge(self.0, child.0, Edge::Gltf(GltfEdge::Node(NodeEdge::Child)));
    }
    pub fn remove_child(&self, graph: &mut Graph, child: &Node) {
        let edge = graph
            .edges_directed(self.0, petgraph::Direction::Outgoing)
            .find(|edge| edge.target() == child.0)
            .expect("Child not found");

        graph.remove_edge(edge.id());
    }
    pub fn parent(&self, graph: &Graph) -> Option<Node> {
        graph
            .edges_directed(self.0, petgraph::Direction::Incoming)
            .find_map(|edge| {
                if let Edge::Gltf(GltfEdge::Node(NodeEdge::Child)) = edge.weight() {
                    Some(
                        match graph.node_weight(edge.source()).expect("Weight not found") {
                            Weight::Gltf(GltfWeight::Node(_)) => Node(edge.source()),
                            _ => panic!("Incorrect weight type"),
                        },
                    )
                } else {
                    None
                }
            })
    }

    pub fn mesh(&self, graph: &Graph) -> Option<Mesh> {
        graph
            .edges_directed(self.0, petgraph::Direction::Outgoing)
            .find(|edge| matches!(edge.weight(), Edge::Gltf(GltfEdge::Node(NodeEdge::Mesh))))
            .map(|edge| Mesh(edge.target()))
    }
    pub fn set_mesh(&self, graph: &mut Graph, mesh: Option<&Mesh>) {
        let edge = graph
            .edges_directed(self.0, petgraph::Direction::Outgoing)
            .find(|edge| matches!(edge.weight(), Edge::Gltf(GltfEdge::Node(NodeEdge::Mesh))))
            .map(|edge| edge.id());

        if let Some(edge) = edge {
            graph.remove_edge(edge);
        }

        if let Some(mesh) = mesh {
            graph.add_edge(self.0, mesh.0, Edge::Gltf(GltfEdge::Node(NodeEdge::Mesh)));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn children() {
        let mut graph = Graph::default();

        let node = Node::new(&mut graph);
        let child = Node::new(&mut graph);

        node.add_child(&mut graph, &child);
        assert_eq!(child.parent(&graph), Some(node));
        assert!(node.parent(&graph).is_none());
        assert!(child.children(&graph).is_empty());

        let children = node.children(&graph);
        assert_eq!(children, vec![child]);

        node.remove_child(&mut graph, &child);
        assert!(node.children(&graph).is_empty());
        assert!(child.parent(&graph).is_none());
    }
}
