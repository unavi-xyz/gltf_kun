use petgraph::stable_graph::NodeIndex;

use super::{GltfGraph, Weight};

#[derive(Debug, Default)]
pub struct BufferWeight {
    pub name: Option<String>,
    pub extras: gltf::json::Extras,

    pub byte_length: usize,
    pub uri: Option<String>,

    pub blob: Option<Vec<u8>>,
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Buffer(pub NodeIndex);

impl Buffer {
    pub fn new(graph: &mut GltfGraph) -> Self {
        let index = graph.add_node(Weight::Buffer(BufferWeight::default()));
        Self(index)
    }

    pub fn get<'a>(&'a self, graph: &'a GltfGraph) -> &'a BufferWeight {
        match graph.node_weight(self.0).expect("Weight not found") {
            Weight::Buffer(weight) => weight,
            _ => panic!("Incorrect weight type"),
        }
    }
    pub fn get_mut<'a>(&'a mut self, graph: &'a mut GltfGraph) -> &'a mut BufferWeight {
        match graph.node_weight_mut(self.0).expect("Weight not found") {
            Weight::Buffer(weight) => weight,
            _ => panic!("Incorrect weight type"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer() {
        let mut graph = GltfGraph::new();
        let mut buffer = Buffer::new(&mut graph);

        buffer.get_mut(&mut graph).name = Some("Test".to_string());
        assert_eq!(buffer.get(&graph).name, Some("Test".to_string()));

        buffer.get_mut(&mut graph).byte_length = 4;
        assert_eq!(buffer.get(&graph).byte_length, 4);

        buffer.get_mut(&mut graph).uri = Some("Test".to_string());
        assert_eq!(buffer.get(&graph).uri, Some("Test".to_string()));

        buffer.get_mut(&mut graph).blob = Some(vec![0, 1, 2, 3]);
        assert_eq!(buffer.get(&graph).blob, Some(vec![0, 1, 2, 3]));
    }
}
