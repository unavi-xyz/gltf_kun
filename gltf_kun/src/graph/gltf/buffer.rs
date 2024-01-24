use petgraph::graph::NodeIndex;

use crate::graph::{Graph, GraphNodeWeight, Property, Weight};

use super::GltfWeight;

#[derive(Debug, Default)]
pub struct BufferWeight {
    pub name: Option<String>,
    pub extras: gltf::json::Extras,

    pub uri: Option<String>,
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

impl GraphNodeWeight<BufferWeight> for Buffer {}
impl Property for Buffer {}

impl Buffer {
    pub fn new(graph: &mut Graph) -> Self {
        let index = graph.add_node(Weight::Gltf(GltfWeight::Buffer(Default::default())));
        Self(index)
    }
}
