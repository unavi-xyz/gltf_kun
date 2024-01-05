use gltf_json::accessor::Type;
use petgraph::stable_graph::NodeIndex;

use crate::{
    extension::ExtensionProperty,
    graph::{GltfGraph, Weight},
};

#[derive(Debug)]
pub struct AccessorWeight {
    pub name: Option<String>,
    pub extras: Option<serde_json::Value>,
    pub extensions: Vec<Box<dyn ExtensionProperty>>,

    pub element_type: Type,
    pub normalized: bool,
    pub array: AccessorArray,
}

#[derive(Debug, PartialEq)]
pub enum AccessorArray {
    I8(Vec<i8>),
    U8(Vec<u8>),
    I16(Vec<i16>),
    U16(Vec<u16>),
    U32(Vec<u32>),
    F32(Vec<f32>),
}

impl Default for AccessorWeight {
    fn default() -> Self {
        Self {
            name: None,
            extras: None,
            extensions: Vec::new(),

            element_type: Type::Scalar,
            normalized: false,
            array: AccessorArray::F32(Vec::new()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Accessor(pub NodeIndex);

impl Accessor {
    pub fn new(graph: &mut GltfGraph) -> Self {
        let index = graph.add_node(Weight::Accessor(AccessorWeight::default()));
        Self(index)
    }

    pub fn get<'a>(&'a self, graph: &'a GltfGraph) -> &'a AccessorWeight {
        match graph.node_weight(self.0).expect("Weight not found") {
            Weight::Accessor(weight) => weight,
            _ => panic!("Incorrect weight type"),
        }
    }
    pub fn get_mut<'a>(&'a mut self, graph: &'a mut GltfGraph) -> &'a mut AccessorWeight {
        match graph.node_weight_mut(self.0).expect("Weight not found") {
            Weight::Accessor(weight) => weight,
            _ => panic!("Incorrect weight type"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accessor() {
        let mut graph = GltfGraph::new();
        let mut accessor = Accessor::new(&mut graph);

        accessor.get_mut(&mut graph).name = Some("Test".to_string());
        assert_eq!(accessor.get(&graph).name, Some("Test".to_string()));

        accessor.get_mut(&mut graph).normalized = true;
        assert!(accessor.get(&graph).normalized);

        accessor.get_mut(&mut graph).element_type = Type::Vec3;
        assert_eq!(accessor.get(&graph).element_type, Type::Vec3);

        accessor.get_mut(&mut graph).array = AccessorArray::I8(vec![1, 2, 3, 4]);
        assert_eq!(
            accessor.get(&graph).array,
            AccessorArray::I8(vec![1, 2, 3, 4])
        );
    }
}
