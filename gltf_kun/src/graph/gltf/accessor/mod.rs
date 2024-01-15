use petgraph::{stable_graph::NodeIndex, visit::EdgeRef};
use thiserror::Error;

use crate::extension::ExtensionProperty;

use self::iter::{AccessorElement, AccessorIter};

use super::{buffer::Buffer, buffer_view::BufferView, Edge, GltfGraph, Weight};

pub use gltf::json::accessor::{ComponentType, Type};

pub mod iter;

#[derive(Debug)]
pub struct AccessorWeight {
    pub name: Option<String>,
    pub extras: gltf::json::Extras,
    pub extensions: Vec<Box<dyn ExtensionProperty>>,

    pub byte_offset: usize,
    pub component_type: ComponentType,
    pub count: usize,
    pub element_type: Type,
    pub normalized: bool,
}

impl Default for AccessorWeight {
    fn default() -> Self {
        Self {
            name: None,
            extras: None,
            extensions: Vec::new(),

            byte_offset: 0,
            component_type: ComponentType::F32,
            count: 0,
            element_type: Type::Scalar,
            normalized: false,
        }
    }
}

#[derive(Debug, Error)]
pub enum GetAccessorSliceError {
    #[error("Failed to get buffer slice: {0}")]
    GetBufferViewSliceError(#[from] super::buffer_view::GetBufferViewSliceError),
    #[error("Accessor slice {0}..{1} is out of bounds for buffer view of length {2}")]
    OutOfBounds(usize, usize, usize),
}

#[derive(Debug, Error)]
pub enum GetAccessorIterError {
    #[error("Failed to create accessor iterator: {0}")]
    CreateError(#[from] iter::AccessorIterCreateError),
    #[error("Failed to get accessor slice: {0}")]
    GetSliceError(#[from] GetAccessorSliceError),
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
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

    pub fn buffer_view(&self, graph: &GltfGraph) -> Option<BufferView> {
        graph
            .edges_directed(self.0, petgraph::Direction::Outgoing)
            .find(|edge| matches!(edge.weight(), Edge::BufferView))
            .map(|edge| BufferView(edge.target()))
    }
    pub fn set_buffer_view(&self, graph: &mut GltfGraph, buffer_view: Option<&BufferView>) {
        let edge = graph
            .edges_directed(self.0, petgraph::Direction::Outgoing)
            .find(|edge| matches!(edge.weight(), Edge::BufferView))
            .map(|edge| edge.id());

        if let Some(edge) = edge {
            graph.remove_edge(edge);
        }

        if let Some(buffer_view) = buffer_view {
            graph.add_edge(self.0, buffer_view.0, Edge::BufferView);
        }
    }

    pub fn from_iter(graph: &mut GltfGraph, iter: AccessorIter, buffer: Option<Buffer>) -> Self {
        let buffer = buffer.unwrap_or_else(|| Buffer::new(graph));

        let buffer_view = BufferView::new(graph);
        buffer_view.set_buffer(graph, Some(&buffer));

        let mut accessor = Self::new(graph);
        accessor.set_buffer_view(graph, Some(&buffer_view));

        let accessor_weight = accessor.get_mut(graph);
        accessor_weight.component_type = iter.component_type();
        accessor_weight.count = iter.count();
        accessor_weight.element_type = iter.element_type();

        accessor
    }

    pub fn iter<'a>(
        &self,
        graph: &'a GltfGraph,
        buffer_view: &'a BufferView,
        buffer: &'a Buffer,
    ) -> Result<AccessorIter<'a>, GetAccessorIterError> {
        let slice = self.slice(graph, buffer_view, buffer)?;
        let weight = self.get(graph);
        Ok(AccessorIter::new(
            slice,
            weight.component_type,
            weight.element_type,
        )?)
    }

    pub fn slice<'a>(
        &self,
        graph: &'a GltfGraph,
        buffer_view: &'a BufferView,
        buffer: &'a Buffer,
    ) -> Result<&'a [u8], GetAccessorSliceError> {
        let slice = buffer_view.slice(graph, buffer)?;

        let weight = self.get(graph);
        let start = weight.byte_offset;
        let end = start
            + weight.count * weight.element_type.multiplicity() * weight.component_type.size();

        if end > slice.len() {
            return Err(GetAccessorSliceError::OutOfBounds(start, end, slice.len()));
        }

        Ok(&slice[start..end])
    }

    pub fn calc_max(&self, graph: &GltfGraph) -> Option<AccessorElement> {
        let buffer_view = self.buffer_view(graph)?;
        let buffer = buffer_view.buffer(graph)?;
        let iter = self.iter(graph, &buffer_view, &buffer).ok()?;
        Some(iter.max())
    }

    pub fn calc_min(&self, graph: &GltfGraph) -> Option<AccessorElement> {
        let buffer_view = self.buffer_view(graph)?;
        let buffer = buffer_view.buffer(graph)?;
        let iter = self.iter(graph, &buffer_view, &buffer).ok()?;
        Some(iter.min())
    }
}

#[cfg(test)]
mod tests {
    use tracing_test::traced_test;

    use super::*;

    #[test]
    #[traced_test]
    fn test_iter() {
        let _graph = GltfGraph::new();
    }
}
