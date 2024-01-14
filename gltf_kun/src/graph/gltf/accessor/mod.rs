use gltf::accessor::util::ItemIter;
use petgraph::{stable_graph::NodeIndex, visit::EdgeRef};

use crate::extension::ExtensionProperty;

use self::iter::{AccessorIter, Element};

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
            component_type: ComponentType::F32,
            count: 0,
            element_type: Type::Scalar,
            normalized: false,
        }
    }
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

    pub fn from_iter(
        iter: iter::AccessorIter,
        graph: &mut GltfGraph,
        buffer: Option<Buffer>,
    ) -> Self {
        let buffer = buffer.unwrap_or_else(|| Buffer::new(graph));

        let buffer_view = BufferView::new(graph);
        buffer_view.set_buffer(graph, Some(&buffer));

        let mut accessor = Self::new(graph);
        accessor.set_buffer_view(graph, Some(&buffer_view));

        let accessor_weight = accessor.get_mut(graph);
        accessor_weight.element_type = iter.element_type();
        accessor_weight.component_type = iter.component_type();

        todo!()
    }

    pub fn iter<'a>(
        &self,
        graph: &'a GltfGraph,
        buffer_view: &'a BufferView,
        buffer: &'a Buffer,
    ) -> Option<AccessorIter<'a>> {
        let slice = self.slice(graph, buffer_view, buffer)?;

        let weight = self.get(graph);
        let stride = weight.element_size() * weight.component_type.size();

        let iter = match (weight.component_type, weight.element_type) {
            (ComponentType::F32, Type::Scalar) => ItemIter::<f32>::new(slice, stride).into(),
            (ComponentType::F32, Type::Vec2) => ItemIter::<[f32; 2]>::new(slice, stride).into(),
            (ComponentType::F32, Type::Vec3) => ItemIter::<[f32; 3]>::new(slice, stride).into(),
            (ComponentType::F32, Type::Vec4) => ItemIter::<[f32; 4]>::new(slice, stride).into(),
            (ComponentType::U32, Type::Scalar) => ItemIter::<u32>::new(slice, stride).into(),
            (ComponentType::U32, Type::Vec2) => ItemIter::<[u32; 2]>::new(slice, stride).into(),
            (ComponentType::U32, Type::Vec3) => ItemIter::<[u32; 3]>::new(slice, stride).into(),
            (ComponentType::U32, Type::Vec4) => ItemIter::<[u32; 4]>::new(slice, stride).into(),
            (ComponentType::U16, Type::Scalar) => ItemIter::<u16>::new(slice, stride).into(),
            (ComponentType::U16, Type::Vec2) => ItemIter::<[u16; 2]>::new(slice, stride).into(),
            (ComponentType::U16, Type::Vec3) => ItemIter::<[u16; 3]>::new(slice, stride).into(),
            (ComponentType::U16, Type::Vec4) => ItemIter::<[u16; 4]>::new(slice, stride).into(),
            (ComponentType::U8, Type::Scalar) => ItemIter::<u8>::new(slice, stride).into(),
            (ComponentType::U8, Type::Vec2) => ItemIter::<[u8; 2]>::new(slice, stride).into(),
            (ComponentType::U8, Type::Vec3) => ItemIter::<[u8; 3]>::new(slice, stride).into(),
            (ComponentType::U8, Type::Vec4) => ItemIter::<[u8; 4]>::new(slice, stride).into(),
            (ComponentType::I16, Type::Scalar) => ItemIter::<i16>::new(slice, stride).into(),
            (ComponentType::I16, Type::Vec2) => ItemIter::<[i16; 2]>::new(slice, stride).into(),
            (ComponentType::I16, Type::Vec3) => ItemIter::<[i16; 3]>::new(slice, stride).into(),
            (ComponentType::I16, Type::Vec4) => ItemIter::<[i16; 4]>::new(slice, stride).into(),
            (ComponentType::I8, Type::Scalar) => ItemIter::<i8>::new(slice, stride).into(),
            (ComponentType::I8, Type::Vec2) => ItemIter::<[i8; 2]>::new(slice, stride).into(),
            (ComponentType::I8, Type::Vec3) => ItemIter::<[i8; 3]>::new(slice, stride).into(),
            (ComponentType::I8, Type::Vec4) => ItemIter::<[i8; 4]>::new(slice, stride).into(),
            _ => panic!(
                "Unsupported accessor type {:?} {:?}",
                weight.component_type, weight.element_type
            ),
        };

        Some(iter)
    }

    pub fn slice<'a>(
        &self,
        graph: &'a GltfGraph,
        buffer_view: &'a BufferView,
        buffer: &'a Buffer,
    ) -> Option<&'a [u8]> {
        let slice = buffer_view.slice(graph, buffer)?;

        let weight = self.get(graph);
        let start = weight.byte_offset;
        let end = start + weight.count * weight.element_size() * weight.component_type.size();

        if end > slice.len() {
            panic!(
                "Accessor slice out of bounds: {}..{} > {}",
                start,
                end,
                slice.len()
            );
        }

        Some(&slice[start..end])
    }

    pub fn calc_max(&self, graph: &GltfGraph) -> Option<Element> {
        let buffer_view = self.buffer_view(graph)?;
        let buffer = buffer_view.buffer(graph)?;

        let iter = self.iter(graph, &buffer_view, &buffer)?;
        Some(iter.max())
    }

    pub fn calc_min(&self, graph: &GltfGraph) -> Option<Element> {
        let buffer_view = self.buffer_view(graph)?;
        let buffer = buffer_view.buffer(graph)?;

        let iter = self.iter(graph, &buffer_view, &buffer)?;
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
        let mut graph = GltfGraph::new();
    }
}
