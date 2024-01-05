use gltf_json::mesh::Mode;
use petgraph::stable_graph::NodeIndex;

use crate::{
    extension::ExtensionProperty,
    graph::{GltfGraph, Weight},
};

#[derive(Debug)]
pub struct PrimitiveWeight {
    pub name: Option<String>,
    pub extras: Option<serde_json::Value>,
    pub extensions: Vec<Box<dyn ExtensionProperty>>,

    pub mode: Mode,
}

impl Default for PrimitiveWeight {
    fn default() -> Self {
        Self {
            name: None,
            extras: None,
            extensions: Vec::new(),

            mode: Mode::Triangles,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Primitive(pub NodeIndex);

impl Primitive {
    pub fn new(graph: &mut GltfGraph) -> Self {
        let index = graph.add_node(Weight::Primitive(PrimitiveWeight::default()));
        Self(index)
    }

    pub fn get<'a>(&'a self, graph: &'a GltfGraph) -> &'a PrimitiveWeight {
        match graph.node_weight(self.0).expect("Weight not found") {
            Weight::Primitive(weight) => weight,
            _ => panic!("Incorrect weight type"),
        }
    }
    pub fn get_mut<'a>(&'a mut self, graph: &'a mut GltfGraph) -> &'a mut PrimitiveWeight {
        match graph.node_weight_mut(self.0).expect("Weight not found") {
            Weight::Primitive(weight) => weight,
            _ => panic!("Incorrect weight type"),
        }
    }
}
