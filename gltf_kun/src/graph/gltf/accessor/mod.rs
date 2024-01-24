use petgraph::graph::NodeIndex;
use thiserror::Error;

use crate::graph::{Edge, Graph, GraphNodeEdges, GraphNodeWeight, Property, Weight};

use self::iter::{AccessorElement, AccessorIter};

use super::{buffer::Buffer, GltfEdge, GltfWeight};

pub use gltf::json::accessor::{ComponentType, Type};

pub mod colors;
pub mod indices;
pub mod iter;
pub mod joints;
pub mod normalize;
pub mod tex_coords;
pub mod weights;

#[derive(Debug, PartialEq, Eq)]
pub enum AccessorEdge {
    Buffer,
}

impl<'a> TryFrom<&'a Edge> for &'a AccessorEdge {
    type Error = ();
    fn try_from(value: &'a Edge) -> Result<Self, Self::Error> {
        match value {
            Edge::Gltf(GltfEdge::Accessor(edge)) => Ok(edge),
            _ => Err(()),
        }
    }
}

impl From<AccessorEdge> for Edge {
    fn from(edge: AccessorEdge) -> Self {
        Self::Gltf(GltfEdge::Accessor(edge))
    }
}

#[derive(Debug)]
pub struct AccessorWeight {
    pub name: Option<String>,
    pub extras: gltf::json::Extras,

    pub component_type: ComponentType,
    pub element_type: Type,
    pub normalized: bool,

    pub data: Vec<u8>,
}

impl Default for AccessorWeight {
    fn default() -> Self {
        Self {
            name: None,
            extras: None,

            component_type: ComponentType::F32,
            element_type: Type::Scalar,
            normalized: false,

            data: Vec::new(),
        }
    }
}

impl From<AccessorWeight> for Weight {
    fn from(weight: AccessorWeight) -> Self {
        Self::Gltf(GltfWeight::Accessor(weight))
    }
}

impl<'a> TryFrom<&'a Weight> for &'a AccessorWeight {
    type Error = ();
    fn try_from(value: &'a Weight) -> Result<Self, Self::Error> {
        match value {
            Weight::Gltf(GltfWeight::Accessor(weight)) => Ok(weight),
            _ => Err(()),
        }
    }
}

impl<'a> TryFrom<&'a mut Weight> for &'a mut AccessorWeight {
    type Error = ();
    fn try_from(value: &'a mut Weight) -> Result<Self, Self::Error> {
        match value {
            Weight::Gltf(GltfWeight::Accessor(weight)) => Ok(weight),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Error)]
pub enum GetAccessorSliceError {
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

impl From<NodeIndex> for Accessor {
    fn from(index: NodeIndex) -> Self {
        Self(index)
    }
}

impl From<Accessor> for NodeIndex {
    fn from(accessor: Accessor) -> Self {
        accessor.0
    }
}

impl GraphNodeWeight<AccessorWeight> for Accessor {}
impl GraphNodeEdges<AccessorEdge> for Accessor {}
impl Property for Accessor {}

impl Accessor {
    pub fn buffer(&self, graph: &Graph) -> Option<Buffer> {
        self.find_edge_target(graph, &AccessorEdge::Buffer)
    }
    pub fn set_buffer(&self, graph: &mut Graph, buffer: Option<Buffer>) {
        self.set_edge_target(graph, AccessorEdge::Buffer, buffer);
    }

    pub fn from_iter(graph: &mut Graph, iter: AccessorIter) -> Self {
        let mut accessor = Self::new(graph);

        let accessor_weight = accessor.get_mut(graph);
        accessor_weight.component_type = iter.component_type();
        accessor_weight.element_type = iter.element_type();
        accessor_weight.normalized = iter.normalized();
        accessor_weight.data = iter.slice().to_vec();

        accessor
    }

    pub fn iter<'a>(&self, graph: &'a Graph) -> Result<AccessorIter<'a>, GetAccessorIterError> {
        let weight = self.get(graph);
        Ok(AccessorIter::new(
            &weight.data,
            weight.component_type,
            weight.element_type,
        )?)
    }

    pub fn calc_max(&self, graph: &Graph) -> Option<AccessorElement> {
        let iter = self.iter(graph).ok()?;
        Some(iter.max())
    }

    pub fn calc_min(&self, graph: &Graph) -> Option<AccessorElement> {
        let iter = self.iter(graph).ok()?;
        Some(iter.min())
    }

    pub fn count(&self, graph: &Graph) -> usize {
        let weight = self.get(graph);
        weight.data.len() / weight.element_type.multiplicity() / weight.component_type.size()
    }
}
