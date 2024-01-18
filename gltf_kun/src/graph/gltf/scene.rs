use petgraph::{graph::NodeIndex, visit::EdgeRef};

use crate::graph::{Edge, Graph, Weight};

use super::{node::Node, GltfEdge, GltfWeight};

#[derive(Debug, PartialEq, Eq)]
pub enum SceneEdge {
    Node,
}

#[derive(Debug, Default)]
pub struct SceneWeight {
    pub name: Option<String>,
    pub extras: gltf::json::Extras,
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Scene(pub NodeIndex);

impl From<NodeIndex> for Scene {
    fn from(index: NodeIndex) -> Self {
        Self(index)
    }
}

impl From<Scene> for NodeIndex {
    fn from(scene: Scene) -> Self {
        scene.0
    }
}

impl Scene {
    pub fn new(graph: &mut Graph) -> Self {
        let index = graph.add_node(Weight::Gltf(GltfWeight::Scene(SceneWeight::default())));
        Self(index)
    }

    pub fn get<'a>(&'a self, graph: &'a Graph) -> &'a SceneWeight {
        match graph.node_weight(self.0).expect("Weight not found") {
            Weight::Gltf(GltfWeight::Scene(weight)) => weight,
            _ => panic!("Incorrect weight type"),
        }
    }
    pub fn get_mut<'a>(&'a mut self, graph: &'a mut Graph) -> &'a mut SceneWeight {
        match graph.node_weight_mut(self.0).expect("Weight not found") {
            Weight::Gltf(GltfWeight::Scene(weight)) => weight,
            _ => panic!("Incorrect weight type"),
        }
    }

    pub fn nodes(&self, graph: &Graph) -> Vec<Node> {
        graph
            .edges_directed(self.0, petgraph::Direction::Outgoing)
            .filter_map(|edge| {
                if let Edge::Gltf(GltfEdge::Scene(SceneEdge::Node)) = edge.weight() {
                    Some(Node(edge.target()))
                } else {
                    None
                }
            })
            .collect()
    }
    pub fn add_node(&self, graph: &mut Graph, node: &Node) {
        graph.add_edge(self.0, node.0, Edge::Gltf(GltfEdge::Scene(SceneEdge::Node)));
    }
    pub fn remove_node(&self, graph: &mut Graph, node: &Node) {
        let edge = graph
            .edges_directed(self.0, petgraph::Direction::Outgoing)
            .find(|edge| edge.target() == node.0)
            .expect("Child not found");

        graph.remove_edge(edge.id());
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_nodes() {
        let mut graph = Graph::default();
        let scene = Scene::new(&mut graph);

        let node = Node::new(&mut graph);
        scene.add_node(&mut graph, &node);

        let nodes = scene.nodes(&graph);
        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0], node);

        scene.remove_node(&mut graph, &node);
        assert_eq!(scene.nodes(&graph).len(), 0);
    }
}
