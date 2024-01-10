use petgraph::{stable_graph::NodeIndex, visit::EdgeRef};

use crate::extension::ExtensionProperty;

use super::{buffer::Buffer, buffer_view::BufferView, Edge, GltfGraph, Weight};

pub use gltf::{accessor::DataType, json::accessor::Type};

#[derive(Debug)]
pub struct AccessorWeight {
    pub name: Option<String>,
    pub extras: gltf::json::Extras,
    pub extensions: Vec<Box<dyn ExtensionProperty>>,

    pub byte_offset: usize,
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

        let weight = self.get(graph);
        let element_size = weight.element_size();
        let component_size = weight.component_type.size();
        Some(byte_length / (element_size * component_size))
    }

    pub fn calc_max(&self, graph: &GltfGraph) -> Option<Vec<f32>> {
        let buffer_view = self.buffer_view(graph)?;
        let buffer = buffer_view.buffer(graph)?;
        let slice = buffer_view.slice(graph, &buffer)?;

        let count = self.count(graph)?;

        let weight = self.get(graph);
        let element_size = weight.element_size();

        let max = calc_max(slice, count, element_size, weight.component_type);
        Some(max)
    }

    pub fn calc_min(&self, graph: &GltfGraph) -> Option<Vec<f32>> {
        let buffer_view = self.buffer_view(graph)?;
        let buffer = buffer_view.buffer(graph)?;
        let slice = buffer_view.slice(graph, &buffer)?;

        let count = self.count(graph)?;

        let weight = self.get(graph);
        let element_size = weight.element_size();

        let min = calc_min(slice, count, element_size, weight.component_type);
        Some(min)
    }
}

fn calc_max(slice: &[u8], count: usize, element_size: usize, component_type: DataType) -> Vec<f32> {
    match component_type {
        DataType::F32 => {
            let mut max = vec![f32::MIN; element_size];
            let component_size = component_type.size();

            for i in 0..count {
                max.iter_mut().enumerate().for_each(|(j, v)| {
                    let index = i * element_size * component_size + j * component_size;

                    let value = f32::from_le_bytes(
                        slice[index..index + component_size].try_into().unwrap(),
                    );

                    if value > *v {
                        *v = value;
                    }
                });
            }

            max
        }
        DataType::U32 => {
            let mut max = vec![u32::MIN; element_size];
            let component_size = component_type.size();

            for i in 0..count {
                max.iter_mut().enumerate().for_each(|(j, v)| {
                    let index = i * element_size * component_size + j * component_size;

                    let value = u32::from_le_bytes(
                        slice[index..index + component_size].try_into().unwrap(),
                    );

                    if value > *v {
                        *v = value;
                    }
                });
            }

            max.iter().map(|&v| v as f32).collect()
        }
        DataType::U16 => {
            let mut max = vec![u16::MIN; element_size];
            let component_size = component_type.size();

            for i in 0..count {
                max.iter_mut().enumerate().for_each(|(j, v)| {
                    let index = i * element_size * component_size + j * component_size;

                    let value = u16::from_le_bytes(
                        slice[index..index + component_size].try_into().unwrap(),
                    );

                    if value > *v {
                        *v = value;
                    }
                });
            }

            max.iter().map(|&v| v as f32).collect()
        }
        DataType::U8 => {
            let mut max = vec![u8::MIN; element_size];
            let component_size = component_type.size();

            for i in 0..count {
                max.iter_mut().enumerate().for_each(|(j, v)| {
                    let index = i * element_size * component_size + j * component_size;

                    let value =
                        u8::from_le_bytes(slice[index..index + component_size].try_into().unwrap());

                    if value > *v {
                        *v = value;
                    }
                });
            }

            max.iter().map(|&v| v as f32).collect()
        }
        DataType::I16 => {
            let mut max = vec![i16::MIN; element_size];
            let component_size = component_type.size();

            for i in 0..count {
                max.iter_mut().enumerate().for_each(|(j, v)| {
                    let index = i * element_size * component_size + j * component_size;

                    let value = i16::from_le_bytes(
                        slice[index..index + component_size].try_into().unwrap(),
                    );

                    if value > *v {
                        *v = value;
                    }
                });
            }

            max.iter().map(|&v| v as f32).collect()
        }
        DataType::I8 => {
            let mut max = vec![i8::MIN; element_size];
            let component_size = component_type.size();

            for i in 0..count {
                max.iter_mut().enumerate().for_each(|(j, v)| {
                    let index = i * element_size * component_size + j * component_size;

                    let value =
                        i8::from_le_bytes(slice[index..index + component_size].try_into().unwrap());

                    if value > *v {
                        *v = value;
                    }
                });
            }

            max.iter().map(|&v| v as f32).collect()
        }
    }
}

pub fn calc_min(
    slice: &[u8],
    count: usize,
    element_size: usize,
    component_type: DataType,
) -> Vec<f32> {
    match component_type {
        DataType::F32 => {
            let mut min = vec![f32::MAX; element_size];
            let component_size = component_type.size();

            for i in 0..count {
                min.iter_mut().enumerate().for_each(|(j, v)| {
                    let index = i * element_size * component_size + j * component_size;

                    let value = f32::from_le_bytes(
                        slice[index..index + component_size].try_into().unwrap(),
                    );

                    if value < *v {
                        *v = value;
                    }
                });
            }

            min
        }
        DataType::U32 => {
            let mut min = vec![u32::MAX; element_size];
            let component_size = component_type.size();

            for i in 0..count {
                min.iter_mut().enumerate().for_each(|(j, v)| {
                    let index = i * element_size * component_size + j * component_size;

                    let value = u32::from_le_bytes(
                        slice[index..index + component_size].try_into().unwrap(),
                    );

                    if value < *v {
                        *v = value;
                    }
                });
            }

            min.iter().map(|&v| v as f32).collect()
        }
        DataType::U16 => {
            let mut min = vec![u16::MAX; element_size];
            let component_size = component_type.size();

            for i in 0..count {
                min.iter_mut().enumerate().for_each(|(j, v)| {
                    let index = i * element_size * component_size + j * component_size;

                    let value = u16::from_le_bytes(
                        slice[index..index + component_size].try_into().unwrap(),
                    );

                    if value < *v {
                        *v = value;
                    }
                });
            }

            min.iter().map(|&v| v as f32).collect()
        }
        DataType::U8 => {
            let mut min = vec![u8::MAX; element_size];
            let component_size = std::mem::size_of::<u8>();

            for i in 0..count {
                min.iter_mut().enumerate().for_each(|(j, v)| {
                    let index = i * element_size * component_size + j * component_size;

                    let value =
                        u8::from_le_bytes(slice[index..index + component_size].try_into().unwrap());

                    if value < *v {
                        *v = value;
                    }
                });
            }

            min.iter().map(|&v| v as f32).collect()
        }
        DataType::I16 => {
            let mut min = vec![i16::MAX; element_size];
            let component_size = std::mem::size_of::<u8>();

            for i in 0..count {
                min.iter_mut().enumerate().for_each(|(j, v)| {
                    let index = i * element_size * component_size + j * component_size;

                    let value = i16::from_le_bytes(
                        slice[index..index + component_size].try_into().unwrap(),
                    );

                    if value < *v {
                        *v = value;
                    }
                });
            }

            min.iter().map(|&v| v as f32).collect()
        }
        DataType::I8 => {
            let mut min = vec![i8::MAX; element_size];
            let component_size = std::mem::size_of::<u8>();

            for i in 0..count {
                min.iter_mut().enumerate().for_each(|(j, v)| {
                    let index = i * element_size * component_size + j * component_size;

                    let value =
                        i8::from_le_bytes(slice[index..index + component_size].try_into().unwrap());

                    if value < *v {
                        *v = value;
                    }
                });
            }

            min.iter().map(|&v| v as f32).collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use tracing_test::traced_test;

    use super::*;

    #[test]
    #[traced_test]
    fn test_max_min_scalar() {
        let vec: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0];
        let slice = vec.iter().flat_map(|v| v.to_le_bytes()).collect::<Vec<_>>();

        let max = calc_max(&slice, vec.len(), 1, DataType::F32);
        assert_eq!(max, vec![4.0]);

        let min = calc_min(&slice, vec.len(), 1, DataType::F32);
        assert_eq!(min, vec![1.0]);
    }

    #[test]
    #[traced_test]
    fn test_max_min_vec2() {
        let vec: Vec<[f32; 2]> = vec![[1.0, 4.0], [2.0, 3.0]];
        let slice = vec
            .iter()
            .flat_map(|v| v.map(|v| v.to_le_bytes()))
            .flatten()
            .collect::<Vec<_>>();

        let max = calc_max(&slice, vec.len(), 2, DataType::F32);
        assert_eq!(max, vec![2.0, 4.0]);

        let min = calc_min(&slice, vec.len(), 2, DataType::F32);
        assert_eq!(min, vec![1.0, 3.0]);
    }

    #[test]
    #[traced_test]
    fn test_max_min_vec3() {
        let vec: Vec<[f32; 3]> = vec![[1.0, 4.0, 5.0], [2.0, 3.0, 6.0]];
        let slice = vec
            .iter()
            .flat_map(|v| v.map(|v| v.to_le_bytes()))
            .flatten()
            .collect::<Vec<_>>();

        let max = calc_max(&slice, vec.len(), 3, DataType::F32);
        assert_eq!(max, vec![2.0, 4.0, 6.0]);

        let min = calc_min(&slice, vec.len(), 3, DataType::F32);
        assert_eq!(min, vec![1.0, 3.0, 5.0]);
    }

    #[test]
    #[traced_test]
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
