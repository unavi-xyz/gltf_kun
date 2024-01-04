use std::{cell::RefCell, rc::Rc};

use glam::Vec3;
use petgraph::stable_graph::NodeIndex;

use crate::{
    extension::ExtensionProperty,
    graph::{GltfGraph, GraphNode, Property},
};

#[derive(Debug)]
pub struct RawNode {
    pub name: Option<String>,
    pub extras: Option<serde_json::Value>,
    pub extensions: Vec<Box<dyn ExtensionProperty>>,

    pub translation: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,

    pub children: Vec<NodeIndex>,
}

impl Default for RawNode {
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

#[derive(Clone, Debug)]
pub struct Node {
    graph: Rc<RefCell<GltfGraph>>,
    index: NodeIndex,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index && Rc::ptr_eq(&self.graph, &other.graph)
    }
}

impl Eq for Node {}

impl Node {
    pub fn new(graph: Rc<RefCell<GltfGraph>>) -> Self {
        let index = graph
            .borrow_mut()
            .add_node(GraphNode::Node(RawNode::default()));

        Self { graph, index }
    }

    fn raw(graph: &GltfGraph, index: NodeIndex) -> &RawNode {
        let GraphNode::Node(node) = graph.node_weight(index).expect("Weight not found");
        node
    }
    fn raw_mut(graph: &mut GltfGraph, index: NodeIndex) -> &mut RawNode {
        let GraphNode::Node(node) = graph.node_weight_mut(index).expect("Weight not found");
        node
    }

    pub fn translation(&self) -> Vec3 {
        Self::raw(&self.graph.borrow(), self.index).translation
    }
    pub fn set_translation(&mut self, translation: Vec3) {
        Self::raw_mut(&mut self.graph.borrow_mut(), self.index).translation = translation;
    }

    pub fn rotation(&self) -> Vec3 {
        Self::raw(&self.graph.borrow(), self.index).rotation
    }
    pub fn set_rotation(&mut self, rotation: Vec3) {
        Self::raw_mut(&mut self.graph.borrow_mut(), self.index).rotation = rotation;
    }

    pub fn scale(&self) -> Vec3 {
        Self::raw(&self.graph.borrow(), self.index).scale
    }
    pub fn set_scale(&mut self, scale: Vec3) {
        Self::raw_mut(&mut self.graph.borrow_mut(), self.index).scale = scale;
    }

    pub fn children(&self) -> Vec<Node> {
        let graph = self.graph.borrow();
        let node = Self::raw(&graph, self.index);

        node.children
            .iter()
            .map(|index| Node {
                graph: self.graph.clone(),
                index: *index,
            })
            .collect()
    }
    pub fn set_children(&mut self, children: Vec<Node>) {
        let mut graph = self.graph.borrow_mut();
        let node = Self::raw_mut(&mut graph, self.index);

        node.children = children.iter().map(|node| node.index).collect();
    }
    pub fn add_child(&mut self, child: Node) {
        let mut graph = self.graph.borrow_mut();
        let node = Self::raw_mut(&mut graph, self.index);

        node.children.push(child.index);
    }
}

impl Property for Node {
    fn name(&self) -> Option<String> {
        Self::raw(&self.graph.borrow(), self.index).name.clone()
    }
    fn set_name(&mut self, name: Option<String>) {
        Self::raw_mut(&mut self.graph.borrow_mut(), self.index).name = name;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node() {
        let graph = Rc::new(RefCell::new(GltfGraph::default()));

        let mut node = Node::new(graph.clone());

        node.set_name(Some("test".to_string()));
        assert_eq!(node.name(), Some("test".to_string()));

        node.set_translation([1.0, 2.0, 3.0].into());
        assert_eq!(node.translation(), [1.0, 2.0, 3.0].into());

        node.set_rotation([4.0, 5.0, 6.0].into());
        assert_eq!(node.rotation(), [4.0, 5.0, 6.0].into());

        node.set_scale([7.0, 8.0, 9.0].into());
        assert_eq!(node.scale(), [7.0, 8.0, 9.0].into());

        let child = Node::new(graph.clone());
        node.add_child(child.clone());

        let children = node.children();
        assert_eq!(children.len(), 1);
        assert_eq!(children[0], child);
    }
}
