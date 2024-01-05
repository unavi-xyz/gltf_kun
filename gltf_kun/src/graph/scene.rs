use petgraph::{stable_graph::NodeIndex, visit::EdgeRef};

use crate::{
    extension::ExtensionProperty,
    graph::{Edge, GltfGraph, Weight},
};

use super::node::Node;

#[derive(Debug, Default)]
pub struct SceneWeight {
    pub name: Option<String>,
    pub extras: Option<gltf::json::Extras>,
    pub extensions: Vec<Box<dyn ExtensionProperty>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
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
    pub fn add_child(&mut self, graph: &mut GltfGraph, child: &Node) {
        graph.add_edge(self.0, child.0, Edge::Child);
    }
    pub fn remove_child(&mut self, graph: &mut GltfGraph, child: &Node) {
        let edge = graph
            .edges_directed(self.0, petgraph::Direction::Outgoing)
            .find(|edge| edge.target() == child.0)
            .expect("Child not found");

        graph.remove_edge(edge.id());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::node::{Node, Parent};

    #[test]
    fn test_node() {
        let mut graph = GltfGraph::default();
        let mut scene = Scene::new(&mut graph);

        scene.get_mut(&mut graph).name = Some("Test".to_string());
        assert_eq!(scene.get(&graph).name, Some("Test".to_string()));

        let child = Node::new(&mut graph);
        scene.add_child(&mut graph, &child);

        let children = scene.children(&graph);
        assert_eq!(children.len(), 1);
        assert_eq!(children[0], child);
        assert_eq!(child.parent(&graph), Some(Parent::Scene(scene.clone())));
        assert_eq!(child.children(&graph).len(), 0);

        scene.remove_child(&mut graph, &child);
        assert_eq!(scene.children(&graph).len(), 0);
        assert_eq!(child.parent(&graph), None);
    }
}
