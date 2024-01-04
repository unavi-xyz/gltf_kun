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

    pub children: Vec<NodeIndex>,
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

            children: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Node(NodeIndex);

impl Node {
    pub fn new(graph: &mut GltfGraph) -> Self {
        let index = graph.add_node(Weight::Node(NodeWeight::default()));
        Self(index)
    }

    pub fn weight<'a>(&'a self, graph: &'a GltfGraph) -> &'a NodeWeight {
        match graph.node_weight(self.0).expect("Weight not found") {
            Weight::Node(node) => node,
            _ => panic!("Incorrect weight type"),
        }
    }
    pub fn weight_mut<'a>(&'a mut self, graph: &'a mut GltfGraph) -> &'a mut NodeWeight {
        match graph.node_weight_mut(self.0).expect("Weight not found") {
            Weight::Node(node) => node,
            _ => panic!("Incorrect weight type"),
        }
    }

    pub fn translation(&self, graph: &GltfGraph) -> Vec3 {
        self.weight(graph).translation
    }
    pub fn set_translation(&mut self, graph: &mut GltfGraph, translation: Vec3) {
        self.weight_mut(graph).translation = translation;
    }

    pub fn rotation(&self, graph: &GltfGraph) -> Vec3 {
        self.weight(graph).rotation
    }
    pub fn set_rotation(&mut self, graph: &mut GltfGraph, rotation: Vec3) {
        self.weight_mut(graph).rotation = rotation;
    }

    pub fn scale(&self, graph: &GltfGraph) -> Vec3 {
        self.weight(graph).scale
    }
    pub fn set_scale(&mut self, graph: &mut GltfGraph, scale: Vec3) {
        self.weight_mut(graph).scale = scale;
    }

    pub fn children(&self, graph: &GltfGraph) -> Vec<Self> {
        let children = self.weight(graph).children.clone();
        children.into_iter().map(Self).collect()
    }
    pub fn set_children(&mut self, graph: &mut GltfGraph, children: Vec<Self>) {
        self.weight_mut(graph).children = children.into_iter().map(|node| node.0).collect();
    }
    pub fn add_child(&mut self, graph: &mut GltfGraph, child: Self) {
        self.weight_mut(graph).children.push(child.0);
    }
}

impl Property for Node {
    fn name<'a>(&'a self, graph: &'a GltfGraph) -> Option<&'a str> {
        self.weight(graph).name.as_deref()
    }
    fn set_name(&mut self, graph: &mut GltfGraph, name: Option<String>) {
        self.weight_mut(graph).name = name;
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

        node.set_translation(&mut graph, [1.0, 2.0, 3.0].into());
        assert_eq!(node.translation(&graph), [1.0, 2.0, 3.0].into());

        node.set_rotation(&mut graph, [4.0, 5.0, 6.0].into());
        assert_eq!(node.rotation(&graph), [4.0, 5.0, 6.0].into());

        node.set_scale(&mut graph, [7.0, 8.0, 9.0].into());
        assert_eq!(node.scale(&graph), [7.0, 8.0, 9.0].into());

        let child = Node::new(&mut graph);
        node.add_child(&mut graph, child.clone());

        let children = node.children(&graph);
        assert_eq!(children.len(), 1);
        assert_eq!(children[0], child);
    }
}
