use glam::Vec3;
use petgraph::stable_graph::NodeIndex;

use crate::{
    extension::ExtensionProperty,
    graph::{GltfGraph, Property, Weight},
};

#[derive(Debug)]
pub struct NodeWeight {
    pub name: Option<String>,
    pub extras: Option<serde_json::Value>,
    pub extensions: Vec<Box<dyn ExtensionProperty>>,

    pub translation: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,

    pub children_ids: Vec<NodeIndex>,
}

impl Default for NodeWeight {
    fn default() -> Self {
        Self {
            name: None,
            extras: None,
            extensions: Vec::new(),

            translation: Vec3::ZERO,
            rotation: Vec3::ZERO,
            scale: Vec3::ONE,

            children_ids: Vec::new(),
        }
    }
}

impl NodeWeight {
    pub fn children(&self) -> Vec<Node> {
        self.children_ids.iter().map(|index| Node(*index)).collect()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Node(pub NodeIndex);

impl Node {
    pub fn new(graph: &mut GltfGraph) -> Self {
        let index = graph.add_node(Weight::Node(NodeWeight::default()));
        Self(index)
    }

    pub fn get<'a>(&'a self, graph: &'a GltfGraph) -> &'a NodeWeight {
        match graph.node_weight(self.0).expect("Weight not found") {
            Weight::Node(node) => node,
            _ => panic!("Incorrect weight type"),
        }
    }
    pub fn get_mut<'a>(&'a mut self, graph: &'a mut GltfGraph) -> &'a mut NodeWeight {
        match graph.node_weight_mut(self.0).expect("Weight not found") {
            Weight::Node(node) => node,
            _ => panic!("Incorrect weight type"),
        }
    }
}

impl Property for Node {
    fn name<'a>(&'a self, graph: &'a GltfGraph) -> Option<&'a str> {
        self.get(graph).name.as_deref()
    }
    fn set_name(&mut self, graph: &mut GltfGraph, name: Option<String>) {
        self.get_mut(graph).name = name;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node() {
        let mut graph = GltfGraph::default();
        let mut node = Node::new(&mut graph);

        node.set_name(&mut graph, Some("test".to_string()));
        assert_eq!(node.name(&graph), Some("test"));

        node.get_mut(&mut graph).translation = [1.0, 2.0, 3.0].into();
        assert_eq!(node.get(&graph).translation, [1.0, 2.0, 3.0].into());

        node.get_mut(&mut graph).rotation = [1.0, 2.0, 3.0].into();
        assert_eq!(node.get(&graph).rotation, [1.0, 2.0, 3.0].into());

        node.get_mut(&mut graph).scale = [1.0, 2.0, 3.0].into();
        assert_eq!(node.get(&graph).scale, [1.0, 2.0, 3.0].into());

        let child = Node::new(&mut graph);
        node.get_mut(&mut graph).children_ids.push(child.0);

        let children = node.get(&graph).children();
        assert_eq!(children.len(), 1);
        assert_eq!(children[0], child);
    }
}
