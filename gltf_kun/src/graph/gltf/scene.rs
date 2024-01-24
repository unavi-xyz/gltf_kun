use petgraph::{graph::NodeIndex, visit::EdgeRef};

use crate::graph::{Edge, Graph, GraphNodeEdges, GraphNodeWeight, Property, Weight};

use super::{node::Node, GltfEdge, GltfWeight};

#[derive(Debug, PartialEq, Eq)]
pub enum SceneEdge {
    Node,
}

impl<'a> TryFrom<&'a Edge> for &'a SceneEdge {
    type Error = ();
    fn try_from(value: &'a Edge) -> Result<Self, Self::Error> {
        match value {
            Edge::Gltf(GltfEdge::Scene(edge)) => Ok(edge),
            _ => Err(()),
        }
    }
}

impl From<SceneEdge> for Edge {
    fn from(edge: SceneEdge) -> Self {
        Self::Gltf(GltfEdge::Scene(edge))
    }
}

#[derive(Debug, Default)]
pub struct SceneWeight {
    pub name: Option<String>,
    pub extras: gltf::json::Extras,
}

impl<'a> TryFrom<&'a Weight> for &'a SceneWeight {
    type Error = ();
    fn try_from(value: &'a Weight) -> Result<Self, Self::Error> {
        match value {
            Weight::Gltf(GltfWeight::Scene(weight)) => Ok(weight),
            _ => Err(()),
        }
    }
}

impl<'a> TryFrom<&'a mut Weight> for &'a mut SceneWeight {
    type Error = ();
    fn try_from(value: &'a mut Weight) -> Result<Self, Self::Error> {
        match value {
            Weight::Gltf(GltfWeight::Scene(weight)) => Ok(weight),
            _ => Err(()),
        }
    }
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

impl GraphNodeWeight<SceneWeight> for Scene {}
impl GraphNodeEdges<SceneEdge> for Scene {}
impl Property for Scene {}

impl Scene {
    pub fn new(graph: &mut Graph) -> Self {
        let index = graph.add_node(Weight::Gltf(GltfWeight::Scene(SceneWeight::default())));
        Self(index)
    }

    pub fn nodes(&self, graph: &Graph) -> Vec<Node> {
        let mut vec = graph
            .edges_directed(self.0, petgraph::Direction::Outgoing)
            .filter_map(|edge| {
                if let Edge::Gltf(GltfEdge::Scene(SceneEdge::Node)) = edge.weight() {
                    Some(Node(edge.target()))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        vec.sort();

        vec
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
    fn nodes() {
        let mut graph = Graph::default();

        let scene = Scene::new(&mut graph);
        let node = Node::new(&mut graph);

        scene.add_node(&mut graph, &node);
        let nodes = scene.nodes(&graph);
        assert_eq!(nodes, vec![node]);

        scene.remove_node(&mut graph, &node);
        assert!(scene.nodes(&graph).is_empty());
    }
}
