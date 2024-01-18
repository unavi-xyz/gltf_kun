use petgraph::{graph::NodeIndex, visit::EdgeRef};
use thiserror::Error;

use crate::graph::{Edge, Graph, Weight};

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

impl BufferView {
    pub fn new(graph: &mut Graph) -> Self {
        let index = graph.add_node(Weight::Gltf(GltfWeight::BufferView(
            BufferViewWeight::default(),
        )));
        Self(index)
    }

    pub fn get<'a>(&'a self, graph: &'a Graph) -> &'a BufferViewWeight {
        match graph.node_weight(self.0).expect("Weight not found") {
            Weight::Gltf(GltfWeight::BufferView(weight)) => weight,
            _ => panic!("Incorrect weight type"),
        }
    }
    pub fn get_mut<'a>(&'a mut self, graph: &'a mut Graph) -> &'a mut BufferViewWeight {
        match graph.node_weight_mut(self.0).expect("Weight not found") {
            Weight::Gltf(GltfWeight::BufferView(weight)) => weight,
            _ => panic!("Incorrect weight type"),
        }
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
    fn test_buffer_view() {
        let mut graph = Graph::new();
        let mut buffer_view = BufferView::new(&mut graph);

        buffer_view.get_mut(&mut graph).name = Some("Test".to_string());
        assert_eq!(buffer_view.get(&graph).name, Some("Test".to_string()));

        buffer_view.get_mut(&mut graph).byte_length = 4;
        assert_eq!(buffer_view.get(&graph).byte_length, 4);

        buffer_view.get_mut(&mut graph).byte_offset = 4;
        assert_eq!(buffer_view.get(&graph).byte_offset, 4);

        buffer_view.get_mut(&mut graph).byte_stride = Some(4);
        assert_eq!(buffer_view.get(&graph).byte_stride, Some(4));

        buffer_view.get_mut(&mut graph).target = Some(Target::ElementArrayBuffer);
        assert_eq!(
            buffer_view.get(&graph).target,
            Some(Target::ElementArrayBuffer)
        );

        let buffer = Buffer::new(&mut graph);
        buffer_view.set_buffer(&mut graph, Some(&buffer));
        assert_eq!(buffer_view.buffer(&graph), Some(buffer));
    }
}
