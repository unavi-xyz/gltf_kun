use glam::{Quat, Vec3};
use petgraph::{stable_graph::NodeIndex, visit::EdgeRef};

use crate::extension::ExtensionProperty;

use super::{mesh::Mesh, scene::Scene, Edge, GltfGraph, Weight};

#[derive(Debug)]
pub struct NodeWeight {
    pub name: Option<String>,
    pub extras: gltf::json::Extras,
    pub extensions: Vec<Box<dyn ExtensionProperty>>,

    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Default for NodeWeight {
    fn default() -> Self {
        Self {
            name: None,
            extras: None,
            extensions: Vec::new(),

            translation: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Node(pub NodeIndex);

impl Node {
    pub fn new(graph: &mut GltfGraph) -> Self {
        let index = graph.add_node(Weight::Node(NodeWeight::default()));
        Self(index)
    }

    pub fn get<'a>(&'a self, graph: &'a GltfGraph) -> &'a NodeWeight {
        match graph.node_weight(self.0).expect("Weight not found") {
            Weight::Node(weight) => weight,
            _ => panic!("Incorrect weight type"),
        }
    }
    pub fn get_mut<'a>(&'a mut self, graph: &'a mut GltfGraph) -> &'a mut NodeWeight {
        match graph.node_weight_mut(self.0).expect("Weight not found") {
            Weight::Node(weight) => weight,
            _ => panic!("Incorrect weight type"),
        }
    }

    pub fn children(&self, graph: &GltfGraph) -> Vec<Node> {
        graph
            .edges_directed(self.0, petgraph::Direction::Outgoing)
            .filter_map(|edge| {
                if let Edge::Child = edge.weight() {
                    Some(Node(edge.target()))
                } else {
                    None
                }
            })
            .collect()
    }
    pub fn add_child(&self, graph: &mut GltfGraph, child: &Node) {
        graph.add_edge(self.0, child.0, Edge::Child);
    }
    pub fn remove_child(&self, graph: &mut GltfGraph, child: &Node) {
        let edge = graph
            .edges_directed(self.0, petgraph::Direction::Outgoing)
            .find(|edge| edge.target() == child.0)
            .expect("Child not found");

        graph.remove_edge(edge.id());
    }
    pub fn parent(&self, graph: &GltfGraph) -> Option<Parent> {
        graph
            .edges_directed(self.0, petgraph::Direction::Incoming)
            .find_map(|edge| {
                if let Edge::Child = edge.weight() {
                    Some(
                        match graph.node_weight(edge.source()).expect("Weight not found") {
                            Weight::Node(_) => Parent::Node(Node(edge.source())),
                            Weight::Scene(_) => Parent::Scene(Scene(edge.source())),
                            _ => panic!("Incorrect weight type"),
                        },
                    )
                } else {
                    None
                }
            })
    }

    pub fn mesh(&self, graph: &GltfGraph) -> Option<Mesh> {
        graph
            .edges_directed(self.0, petgraph::Direction::Outgoing)
            .find(|edge| matches!(edge.weight(), Edge::Mesh))
            .map(|edge| Mesh(edge.target()))
    }
    pub fn set_mesh(&self, graph: &mut GltfGraph, mesh: Option<&Mesh>) {
        let edge = graph
            .edges_directed(self.0, petgraph::Direction::Outgoing)
            .find(|edge| matches!(edge.weight(), Edge::Mesh))
            .map(|edge| edge.id());

        if let Some(edge) = edge {
            graph.remove_edge(edge);
        }

        if let Some(mesh) = mesh {
            graph.add_edge(self.0, mesh.0, Edge::Mesh);
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Parent {
    Node(Node),
    Scene(Scene),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node() {
        let mut graph = GltfGraph::default();
        let mut node = Node::new(&mut graph);

        node.get_mut(&mut graph).name = Some("Test".to_string());
        assert_eq!(node.get(&graph).name, Some("Test".to_string()));

        node.get_mut(&mut graph).translation = [1.0, 2.0, 3.0].into();
        assert_eq!(node.get(&graph).translation, [1.0, 2.0, 3.0].into());

        node.get_mut(&mut graph).rotation = Quat::from_xyzw(0.5, 0.5, 0.5, 0.5);
        assert_eq!(
            node.get(&graph).rotation,
            Quat::from_xyzw(0.5, 0.5, 0.5, 0.5)
        );

        node.get_mut(&mut graph).scale = [1.0, 2.0, 3.0].into();
        assert_eq!(node.get(&graph).scale, [1.0, 2.0, 3.0].into());

        let child = Node::new(&mut graph);
        node.add_child(&mut graph, &child);

        let children = node.children(&graph);
        assert_eq!(children.len(), 1);
        assert_eq!(children[0], child);
        assert_eq!(child.parent(&graph).unwrap(), Parent::Node(node));
        assert_eq!(node.parent(&graph), None);
        assert_eq!(child.children(&graph).len(), 0);

        node.remove_child(&mut graph, &child);
        assert_eq!(node.children(&graph).len(), 0);
        assert_eq!(child.parent(&graph), None);
    }
}
