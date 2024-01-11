use petgraph::{stable_graph::NodeIndex, visit::EdgeRef};
use tracing::error;

use crate::extension::ExtensionProperty;

use super::{buffer::Buffer, Edge, GltfGraph, Weight};

#[derive(Debug, Default)]
pub struct BufferViewWeight {
    pub name: Option<String>,
    pub extras: gltf::json::Extras,
    pub extensions: Vec<Box<dyn ExtensionProperty>>,

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

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BufferView(pub NodeIndex);

impl BufferView {
    pub fn new(graph: &mut GltfGraph) -> Self {
        let index = graph.add_node(Weight::BufferView(BufferViewWeight::default()));
        Self(index)
    }

    pub fn get<'a>(&'a self, graph: &'a GltfGraph) -> &'a BufferViewWeight {
        match graph.node_weight(self.0).expect("Weight not found") {
            Weight::BufferView(weight) => weight,
            _ => panic!("Incorrect weight type"),
        }
    }
    pub fn get_mut<'a>(&'a mut self, graph: &'a mut GltfGraph) -> &'a mut BufferViewWeight {
        match graph.node_weight_mut(self.0).expect("Weight not found") {
            Weight::BufferView(weight) => weight,
            _ => panic!("Incorrect weight type"),
        }
    }

    pub fn buffer(&self, graph: &GltfGraph) -> Option<Buffer> {
        graph
            .edges_directed(self.0, petgraph::Direction::Outgoing)
            .find(|edge| matches!(edge.weight(), Edge::Buffer))
            .map(|edge| Buffer(edge.target()))
    }
    pub fn set_buffer(&self, graph: &mut GltfGraph, buffer: Option<&Buffer>) {
        let edge = graph
            .edges_directed(self.0, petgraph::Direction::Outgoing)
            .find(|edge| matches!(edge.weight(), Edge::Buffer))
            .map(|edge| edge.id());

        if let Some(edge) = edge {
            graph.remove_edge(edge);
        }

        if let Some(buffer) = buffer {
            graph.add_edge(self.0, buffer.0, Edge::Buffer);
        }
    }

    /// Returns the slice of the buffer that this view represents.
    pub fn slice<'a>(&'a self, graph: &'a GltfGraph, buffer: &'a Buffer) -> Option<&'a [u8]> {
        let weight = self.get(graph);

        let buffer = buffer.get(graph);

        let start = weight.byte_offset;
        let end = start + weight.byte_length;

        match &buffer.blob {
            Some(blob) => {
                if end > blob.len() {
                    panic!(
                        "Buffer view slice out of bounds: {}..{} > {}",
                        start,
                        end,
                        blob.len()
                    );
                }

                Some(&blob[start..end])
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_view() {
        let mut graph = GltfGraph::new();
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
