use petgraph::graph::NodeIndex;

use crate::graph::{Edge, Extensions, Graph, GraphNodeEdges, GraphNodeWeight, Weight};

use super::{GltfEdge, GltfWeight, Skin, mesh::Mesh};

pub use bevy_math::{Quat, Vec3};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NodeEdge {
    Child,
    Mesh,
    Skin,
}

impl<'a> TryFrom<&'a Edge> for &'a NodeEdge {
    type Error = ();
    fn try_from(value: &'a Edge) -> Result<Self, Self::Error> {
        match value {
            Edge::Gltf(GltfEdge::Node(edge)) => Ok(edge),
            _ => Err(()),
        }
    }
}

impl From<NodeEdge> for Edge {
    fn from(edge: NodeEdge) -> Self {
        Self::Gltf(GltfEdge::Node(edge))
    }
}

#[derive(Clone, Debug)]
pub struct NodeWeight {
    pub name: Option<String>,
    pub extras: gltf::json::Extras,

    pub rotation: Quat,
    pub scale: Vec3,
    pub translation: Vec3,
    pub weights: Vec<f32>,
}

impl Default for NodeWeight {
    fn default() -> Self {
        Self {
            name: None,
            extras: None,

            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
            translation: Vec3::ZERO,
            weights: Vec::new(),
        }
    }
}

impl From<NodeWeight> for Weight {
    fn from(weight: NodeWeight) -> Self {
        Self::Gltf(GltfWeight::Node(weight))
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

impl GraphNodeWeight<NodeWeight> for Node {}
impl GraphNodeEdges for Node {}
impl Extensions for Node {}

impl Node {
    pub fn children(&self, graph: &Graph) -> Vec<Node> {
        self.edge_targets(graph, &NodeEdge::Child)
    }
    pub fn add_child(&self, graph: &mut Graph, child: &Node) {
        self.add_edge_target(graph, NodeEdge::Child, *child);
    }
    pub fn remove_child(&self, graph: &mut Graph, child: &Node) {
        self.remove_edge_target(graph, NodeEdge::Child, *child);
    }

    pub fn parents(&self, graph: &Graph) -> Vec<Node> {
        self.edge_sources(graph, &NodeEdge::Child)
    }

    pub fn mesh(&self, graph: &Graph) -> Option<Mesh> {
        self.find_edge_target(graph, &NodeEdge::Mesh)
    }
    pub fn set_mesh(&self, graph: &mut Graph, mesh: Option<Mesh>) {
        self.set_edge_target(graph, NodeEdge::Mesh, mesh);
    }

    pub fn skin(&self, graph: &Graph) -> Option<Skin> {
        self.find_edge_target(graph, &NodeEdge::Skin)
    }
    pub fn set_skin(&self, graph: &mut Graph, skin: Option<Skin>) {
        self.set_edge_target(graph, NodeEdge::Skin, skin);
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
        assert_eq!(child.parents(&graph), vec![node]);
        assert!(node.parents(&graph).is_empty());
        assert!(child.children(&graph).is_empty());

        let children = node.children(&graph);
        assert_eq!(children, vec![child]);

        node.remove_child(&mut graph, &child);
        assert!(node.children(&graph).is_empty());
        assert!(child.parents(&graph).is_empty());
    }

    #[test]
    fn mesh() {
        let mut graph = Graph::default();

        let node = Node::new(&mut graph);
        let mesh = Mesh::new(&mut graph);

        node.set_mesh(&mut graph, Some(mesh));
        assert_eq!(node.mesh(&graph), Some(mesh));

        node.set_mesh(&mut graph, None);
        assert!(node.mesh(&graph).is_none());
    }

    #[test]
    fn skin() {
        let mut graph = Graph::default();

        let node = Node::new(&mut graph);
        let skin = Skin::new(&mut graph);

        node.set_skin(&mut graph, Some(skin));
        assert_eq!(node.skin(&graph), Some(skin));

        node.set_skin(&mut graph, None);
        assert!(node.skin(&graph).is_none());
    }
}
