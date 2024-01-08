use petgraph::{stable_graph::NodeIndex, visit::EdgeRef};

use crate::extension::ExtensionProperty;

use super::{node::Node, Edge, GltfGraph, Weight};

#[derive(Debug, Default)]
pub struct SceneWeight {
    pub name: Option<String>,
    pub extras: gltf::json::Extras,
    pub extensions: Vec<Box<dyn ExtensionProperty>>,
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Scene(pub NodeIndex);

impl Scene {
    pub fn new(graph: &mut GltfGraph) -> Self {
        let index = graph.add_node(Weight::Scene(SceneWeight::default()));
        Self(index)
    }

    pub fn get<'a>(&'a self, graph: &'a GltfGraph) -> &'a SceneWeight {
        match graph.node_weight(self.0).expect("Weight not found") {
            Weight::Scene(weight) => weight,
            _ => panic!("Incorrect weight type"),
        }
    }
    pub fn get_mut<'a>(&'a mut self, graph: &'a mut GltfGraph) -> &'a mut SceneWeight {
        match graph.node_weight_mut(self.0).expect("Weight not found") {
            Weight::Scene(weight) => weight,
            _ => panic!("Incorrect weight type"),
        }
    }

    pub fn nodes(&self, graph: &GltfGraph) -> Vec<Node> {
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
    pub fn add_node(&self, graph: &mut GltfGraph, node: &Node) {
        graph.add_edge(self.0, node.0, Edge::Child);
    }
    pub fn remove_node(&self, graph: &mut GltfGraph, node: &Node) {
        let edge = graph
            .edges_directed(self.0, petgraph::Direction::Outgoing)
            .find(|edge| edge.target() == node.0)
            .expect("Child not found");

        graph.remove_edge(edge.id());
    }
}

#[cfg(test)]
mod tests {
    use crate::graph::gltf::node::Parent;

    use super::*;

    #[test]
    fn test_node() {
        let mut graph = GltfGraph::default();
        let mut scene = Scene::new(&mut graph);

        scene.get_mut(&mut graph).name = Some("Test".to_string());
        assert_eq!(scene.get(&graph).name, Some("Test".to_string()));

        let node = Node::new(&mut graph);
        scene.add_node(&mut graph, &node);

        let nodes = scene.nodes(&graph);
        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0], node);
        assert_eq!(node.parent(&graph), Some(Parent::Scene(scene)));

        scene.remove_node(&mut graph, &node);
        assert_eq!(scene.nodes(&graph).len(), 0);
        assert_eq!(node.parent(&graph), None);
    }
}
