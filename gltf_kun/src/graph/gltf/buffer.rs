use petgraph::graph::NodeIndex;

use crate::graph::{Graph, GraphNode, Property, Weight};

use super::GltfWeight;

#[derive(Debug, Default)]
pub struct BufferWeight {
    pub name: Option<String>,
    pub extras: gltf::json::Extras,

    pub byte_length: usize,
    pub uri: Option<String>,

    pub blob: Option<Vec<u8>>,
}

impl<'a> TryFrom<&'a Weight> for &'a BufferWeight {
    type Error = ();
    fn try_from(value: &'a Weight) -> Result<Self, Self::Error> {
        match value {
            Weight::Gltf(GltfWeight::Buffer(weight)) => Ok(weight),
            _ => Err(()),
        }
    }
}

impl<'a> TryFrom<&'a mut Weight> for &'a mut BufferWeight {
    type Error = ();
    fn try_from(value: &'a mut Weight) -> Result<Self, Self::Error> {
        match value {
            Weight::Gltf(GltfWeight::Buffer(weight)) => Ok(weight),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Buffer(pub NodeIndex);

impl From<NodeIndex> for Buffer {
    fn from(index: NodeIndex) -> Self {
        Self(index)
    }
}

impl From<Buffer> for NodeIndex {
    fn from(buffer: Buffer) -> Self {
        buffer.0
    }
}

impl GraphNode<BufferWeight> for Buffer {}
impl Property for Buffer {}

impl Buffer {
    pub fn new(graph: &mut Graph) -> Self {
        let index = graph.add_node(Weight::Gltf(GltfWeight::Buffer(Default::default())));
        Self(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer() {
        let mut graph = Graph::new();
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
