use petgraph::{stable_graph::NodeIndex, visit::EdgeRef};

use crate::extension::ExtensionProperty;

use super::{buffer::Buffer, buffer_view::BufferView, Edge, GltfGraph, Weight};

pub use gltf::{accessor::DataType, json::accessor::Type};

#[derive(Debug)]
pub struct AccessorWeight {
    pub name: Option<String>,
    pub extras: gltf::json::Extras,
    pub extensions: Vec<Box<dyn ExtensionProperty>>,

    pub byte_offset: u64,
    pub component_type: DataType,
    pub element_type: Type,
    pub normalized: bool,
}

impl AccessorWeight {
    pub fn element_size(&self) -> usize {
        match self.element_type {
            Type::Scalar => 1,
            Type::Vec2 => 2,
            Type::Vec3 => 3,
            Type::Vec4 => 4,
            Type::Mat2 => 4,
            Type::Mat3 => 9,
            Type::Mat4 => 16,
        }
    }
}

impl Default for AccessorWeight {
    fn default() -> Self {
        Self {
            name: None,
            extras: None,
            extensions: Vec::new(),

            byte_offset: 0,
            component_type: DataType::F32,
            element_type: Type::Scalar,
            normalized: false,
        }
    }
}

pub struct AccessorArray {
    pub vec: Vec<u8>,
    pub data_type: DataType,
    pub element_type: Type,
    pub normalized: bool,
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Accessor(pub NodeIndex);

impl Accessor {
    pub fn new(graph: &mut GltfGraph) -> Self {
        let index = graph.add_node(Weight::Accessor(AccessorWeight::default()));
        Self(index)
    }

    /// Creates a new [Accessor], [BufferView], and possibly [Buffer](super::buffer::Buffer) from the given array.
    pub fn from_array(graph: &mut GltfGraph, array: AccessorArray, buffer: Option<Buffer>) -> Self {
        let mut buffer = buffer.unwrap_or_else(|| Buffer::new(graph));

        let mut buffer_view = BufferView::new(graph);
        buffer_view.set_buffer(graph, Some(&buffer));

        let mut accessor = Self::new(graph);
        accessor.set_buffer_view(graph, Some(&buffer_view));

        let accessor_weight = accessor.get_mut(graph);
        accessor_weight.element_type = array.element_type;
        let element_size = accessor_weight.element_size();

        let buffer_weight = buffer.get_mut(graph);
        let prev_buffer_length = buffer_weight.byte_length;

        let byte_length = element_size * array.vec.len();
        buffer_weight.byte_length += byte_length;

        match buffer_weight.blob {
            Some(ref mut blob) => blob.extend(array.vec),
            None => buffer_weight.blob = Some(array.vec),
        }

        let buffer_view_weight = buffer_view.get_mut(graph);
        buffer_view_weight.byte_length = byte_length;
        buffer_view_weight.byte_stride = Some(element_size);
        buffer_view_weight.byte_offset = prev_buffer_length;

        accessor
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

    pub fn count(&self, graph: &GltfGraph) -> Option<usize> {
        let buffer_view = self.buffer_view(graph)?;
        let byte_length = buffer_view.get(graph).byte_length;
        let element_size = self.get(graph).element_size();
        Some(byte_length / element_size)
    }

    pub fn calc_max(&self, graph: &GltfGraph) -> Option<Vec<f32>> {
        let buffer_view = self.buffer_view(graph)?;
        let buffer = buffer_view.buffer(graph)?;
        let slice = buffer_view.slice(graph, &buffer)?;

        let count = self.count(graph)?;
        let element_size = self.get(graph).element_size();

        let mut max = vec![f32::MIN; element_size];

        for i in 0..count {
            let start = i * element_size;
            let end = start + element_size;

            let slice = &slice[start..end];
            let slice = slice.chunks_exact(4);

            for (i, value) in slice.enumerate() {
                let value = f32::from_le_bytes(value.try_into().unwrap());
                max[i] = max[i].max(value);
            }
        }

        Some(max)
    }

    pub fn calc_min(&self, graph: &GltfGraph) -> Option<Vec<f32>> {
        let buffer_view = self.buffer_view(graph)?;
        let buffer = buffer_view.buffer(graph)?;
        let slice = buffer_view.slice(graph, &buffer)?;

        let count = self.count(graph)?;
        let element_size = self.get(graph).element_size();

        let mut min = vec![f32::MAX; element_size];

        for i in 0..count {
            let start = i * element_size;
            let end = start + element_size;

            let slice = &slice[start..end];
            let slice = slice.chunks_exact(4);

            for (i, value) in slice.enumerate() {
                let value = f32::from_le_bytes(value.try_into().unwrap());
                min[i] = min[i].min(value);
            }
        }

        Some(min)
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

        accessor.get_mut(&mut graph).element_type = Type::Vec3;
        assert_eq!(accessor.get(&graph).element_type, Type::Vec3);

        accessor.get_mut(&mut graph).normalized = true;
        assert!(accessor.get(&graph).normalized);

        let buffer_view = BufferView::new(&mut graph);
        accessor.set_buffer_view(&mut graph, Some(&buffer_view));
        assert_eq!(accessor.buffer_view(&graph), Some(buffer_view));
    }
}
