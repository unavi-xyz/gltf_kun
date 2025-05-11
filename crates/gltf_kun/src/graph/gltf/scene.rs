use petgraph::graph::NodeIndex;

use crate::graph::{Edge, Extensions, Graph, GraphNodeEdges, GraphNodeWeight, Weight};

use super::{GltfEdge, GltfWeight, node::Node};

#[derive(Clone, Debug, PartialEq, Eq)]
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

#[derive(Clone, Debug, Default)]
pub struct SceneWeight {
    pub name: Option<String>,
    pub extras: gltf::json::Extras,
}

impl From<SceneWeight> for Weight {
    fn from(weight: SceneWeight) -> Self {
        Self::Gltf(GltfWeight::Scene(weight))
    }
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
impl GraphNodeEdges for Scene {}
impl Extensions for Scene {}

impl Scene {
    pub fn nodes(&self, graph: &Graph) -> Vec<Node> {
        self.edge_targets(graph, &SceneEdge::Node)
    }
    pub fn add_node(&self, graph: &mut Graph, node: Node) {
        self.add_edge_target(graph, SceneEdge::Node, node);
    }
    pub fn remove_node(&self, graph: &mut Graph, node: Node) {
        self.remove_edge_target(graph, SceneEdge::Node, node);
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

        scene.add_node(&mut graph, node);
        let nodes = scene.nodes(&graph);
        assert_eq!(nodes, vec![node]);

        scene.remove_node(&mut graph, node);
        assert!(scene.nodes(&graph).is_empty());
    }
}
