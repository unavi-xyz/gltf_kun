use petgraph::{graph::NodeIndex, visit::EdgeRef};
use thiserror::Error;

use crate::graph::{Edge, Graph, GraphNode, Property, Weight};

use super::{buffer::Buffer, GltfEdge, GltfWeight};

#[derive(Debug, PartialEq, Eq)]
pub enum BufferViewEdge {
    Buffer,
}

#[derive(Debug, Default)]
pub struct BufferViewWeight {
    pub name: Option<String>,
    pub extras: gltf::json::Extras,

    pub byte_length: usize,
    pub byte_offset: usize,
    pub byte_stride: Option<usize>,
    pub target: Option<Target>,
}

impl<'a> TryFrom<&'a Weight> for &'a BufferViewWeight {
    type Error = ();
    fn try_from(value: &'a Weight) -> Result<Self, Self::Error> {
        match value {
            Weight::Gltf(GltfWeight::BufferView(weight)) => Ok(weight),
            _ => Err(()),
        }
    }
}

impl<'a> TryFrom<&'a mut Weight> for &'a mut BufferViewWeight {
    type Error = ();
    fn try_from(value: &'a mut Weight) -> Result<Self, Self::Error> {
        match value {
            Weight::Gltf(GltfWeight::BufferView(weight)) => Ok(weight),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Target {
    ArrayBuffer,
    ElementArrayBuffer,
    Unknown(usize),
}

impl From<usize> for Target {
    fn from(value: usize) -> Self {
        match value {
            34962 => Target::ArrayBuffer,
            34963 => Target::ElementArrayBuffer,
            _ => Target::Unknown(value),
        }
    }
}

impl From<Target> for usize {
    fn from(value: Target) -> Self {
        match value {
            Target::ArrayBuffer => 34962,
            Target::ElementArrayBuffer => 34963,
            Target::Unknown(value) => value,
        }
    }
}

#[derive(Debug, Error)]
pub enum GetBufferViewSliceError {
    #[error("Buffer has no blob")]
    MissingBlob,
    #[error("Slice {0}..{1} is out of bounds for buffer of length {2}")]
    OutOfBounds(usize, usize, usize),
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BufferView(pub NodeIndex);

impl From<NodeIndex> for BufferView {
    fn from(index: NodeIndex) -> Self {
        Self(index)
    }
}

impl From<BufferView> for NodeIndex {
    fn from(buffer_view: BufferView) -> Self {
        buffer_view.0
    }
}

impl GraphNode<BufferViewWeight> for BufferView {}
impl Property for BufferView {}

impl BufferView {
    pub fn new(graph: &mut Graph) -> Self {
        let index = graph.add_node(Weight::Gltf(GltfWeight::BufferView(Default::default())));
        Self(index)
    }

    pub fn buffer(&self, graph: &Graph) -> Option<Buffer> {
        graph
            .edges_directed(self.0, petgraph::Direction::Outgoing)
            .find(|edge| {
                matches!(
                    edge.weight(),
                    Edge::Gltf(GltfEdge::BufferView(BufferViewEdge::Buffer))
                )
            })
            .map(|edge| Buffer(edge.target()))
    }
    pub fn set_buffer(&self, graph: &mut Graph, buffer: Option<&Buffer>) {
        let edge = graph
            .edges_directed(self.0, petgraph::Direction::Outgoing)
            .find(|edge| {
                matches!(
                    edge.weight(),
                    Edge::Gltf(GltfEdge::BufferView(BufferViewEdge::Buffer))
                )
            })
            .map(|edge| edge.id());

        if let Some(edge) = edge {
            graph.remove_edge(edge);
        }

        if let Some(buffer) = buffer {
            graph.add_edge(
                self.0,
                buffer.0,
                Edge::Gltf(GltfEdge::BufferView(BufferViewEdge::Buffer)),
            );
        }
    }

    /// Returns the slice of the buffer that this view represents.
    pub fn slice<'a>(
        &'a self,
        graph: &'a Graph,
        buffer: &'a Buffer,
    ) -> Result<&'a [u8], GetBufferViewSliceError> {
        let buffer = buffer.get(graph);

        let weight = self.get(graph);
        let start = weight.byte_offset;
        let end = start + weight.byte_length;

        match &buffer.blob {
            Some(blob) => {
                if end > blob.len() {
                    return Err(GetBufferViewSliceError::OutOfBounds(start, end, blob.len()));
                }

                Ok(&blob[start..end])
            }
            None => Err(GetBufferViewSliceError::MissingBlob),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer() {
        let mut graph = Graph::new();

        let buffer_view = BufferView::new(&mut graph);
        let buffer = Buffer::new(&mut graph);
        buffer_view.set_buffer(&mut graph, Some(&buffer));

        assert_eq!(buffer_view.buffer(&graph), Some(buffer));
    }
}
