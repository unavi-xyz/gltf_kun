pub use gltf::json::accessor::ComponentType;
use std::{cell::RefCell, rc::Rc};

use crate::graph::{AccessorArray, AccessorData, GltfGraph, GraphData, GraphNode};
use petgraph::graph::NodeIndex;

pub struct Accessor {
    pub(crate) node: GraphNode,
}

impl Accessor {
    pub fn new(graph: Rc<RefCell<GltfGraph>>, index: NodeIndex) -> Self {
        Self {
            node: GraphNode::new(graph, index),
        }
    }

    pub fn data(&self) -> AccessorData {
        match self.node.data() {
            GraphData::Accessor(data) => data,
            _ => panic!("data is not an accessor"),
        }
    }

    pub fn set_data(&mut self, data: AccessorData) {
        self.node.set_data(GraphData::Accessor(data));
    }

    /// The number of bytes required to store this accessor's data.
    pub fn byte_length(&self) -> usize {
        self.data().array.len() * self.component_size()
    }

    /// The datatype of components in this accessor.
    pub fn component_type(&self) -> ComponentType {
        match self.data().array {
            AccessorArray::I8(_) => ComponentType::I8,
            AccessorArray::U8(_) => ComponentType::U8,
            AccessorArray::I16(_) => ComponentType::I16,
            AccessorArray::U16(_) => ComponentType::U16,
            AccessorArray::U32(_) => ComponentType::U32,
            AccessorArray::F32(_) => ComponentType::F32,
        }
    }

    /// The number of bytes per component in this accessor.
    pub fn component_size(&self) -> usize {
        match self.component_type() {
            ComponentType::I8 => 1,
            ComponentType::U8 => 1,
            ComponentType::I16 => 2,
            ComponentType::U16 => 2,
            ComponentType::U32 => 4,
            ComponentType::F32 => 4,
        }
    }

    /// Number of elements in this accessor.
    pub fn count(&self) -> usize {
        let len = self.data().array.len();
        let element_size = self.element_size();
        len / element_size
    }

    /// Size of each element in this accessor.
    pub fn element_size(&self) -> usize {
        self.data().element_type.size()
    }

    /// Maxiumum value of each component in this accessor.
    pub fn max(&self) -> AccessorArray {
        let element_size = self.element_size();

        match self.data().array {
            AccessorArray::I8(array) => get_max(&array, element_size).into(),
            AccessorArray::U8(array) => get_max(&array, element_size).into(),
            AccessorArray::I16(array) => get_max(&array, element_size).into(),
            AccessorArray::U16(array) => get_max(&array, element_size).into(),
            AccessorArray::U32(array) => get_max(&array, element_size).into(),
            AccessorArray::F32(array) => get_max(&array, element_size).into(),
        }
    }

    /// Minimum value of each component in this accessor.
    pub fn min(&self) -> AccessorArray {
        let element_size = self.element_size();

        match self.data().array {
            AccessorArray::I8(array) => get_min(&array, element_size).into(),
            AccessorArray::U8(array) => get_min(&array, element_size).into(),
            AccessorArray::I16(array) => get_min(&array, element_size).into(),
            AccessorArray::U16(array) => get_min(&array, element_size).into(),
            AccessorArray::U32(array) => get_min(&array, element_size).into(),
            AccessorArray::F32(array) => get_min(&array, element_size).into(),
        }
    }
}

fn get_max<T>(array: &[T], element_size: usize) -> Vec<T>
where
    T: Copy + PartialOrd,
{
    array
        .chunks(element_size)
        .fold(Vec::with_capacity(element_size), |max: Vec<T>, chunk| {
            max.iter()
                .zip(chunk.iter())
                .map(|(a, b)| if a > b { *a } else { *b })
                .collect()
        })
}

fn get_min<T>(array: &[T], element_size: usize) -> Vec<T>
where
    T: Copy + PartialOrd,
{
    array
        .chunks(element_size)
        .fold(Vec::with_capacity(element_size), |min: Vec<T>, chunk| {
            min.iter()
                .zip(chunk.iter())
                .map(|(a, b)| if a < b { *a } else { *b })
                .collect()
        })
}
