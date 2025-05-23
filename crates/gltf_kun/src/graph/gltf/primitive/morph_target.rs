use gltf::Semantic;
use petgraph::{Direction, graph::NodeIndex, visit::EdgeRef};
use thiserror::Error;

use crate::graph::{
    Edge, Extensions, Graph, GraphNodeEdges, Weight,
    gltf::{Accessor, GltfEdge, GltfWeight, accessor::iter::AccessorIterCreateError},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MorphTargetEdge {
    Attribute(Semantic),
}

impl<'a> TryFrom<&'a Edge> for &'a MorphTargetEdge {
    type Error = ();
    fn try_from(value: &'a Edge) -> Result<Self, Self::Error> {
        match value {
            Edge::Gltf(GltfEdge::MorphTarget(edge)) => Ok(edge),
            _ => Err(()),
        }
    }
}

impl From<MorphTargetEdge> for Edge {
    fn from(edge: MorphTargetEdge) -> Self {
        Self::Gltf(GltfEdge::MorphTarget(edge))
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MorphTarget(pub NodeIndex);

impl From<NodeIndex> for MorphTarget {
    fn from(index: NodeIndex) -> Self {
        Self(index)
    }
}

impl From<MorphTarget> for NodeIndex {
    fn from(primitive: MorphTarget) -> Self {
        primitive.0
    }
}

impl GraphNodeEdges for MorphTarget {}
impl Extensions for MorphTarget {}

impl MorphTarget {
    pub fn new(graph: &mut Graph) -> Self {
        Self(graph.add_node(Weight::Gltf(GltfWeight::MorphTarget)))
    }

    pub fn attributes(&self, graph: &Graph) -> Vec<(Semantic, Accessor)> {
        graph
            .edges_directed(self.0, Direction::Outgoing)
            .filter_map(|edge| {
                if let Edge::Gltf(GltfEdge::MorphTarget(MorphTargetEdge::Attribute(semantic))) =
                    edge.weight()
                {
                    Some((semantic.clone(), Accessor(edge.target())))
                } else {
                    None
                }
            })
            .collect()
    }
    pub fn attribute(&self, graph: &Graph, semantic: Semantic) -> Option<Accessor> {
        self.find_edge_target(graph, &MorphTargetEdge::Attribute(semantic))
    }
    pub fn set_attribute(&self, graph: &mut Graph, semantic: Semantic, accessor: Option<Accessor>) {
        self.set_edge_target(graph, MorphTargetEdge::Attribute(semantic), accessor);
    }
}

#[derive(Debug, Error)]
pub enum MorphTargetIterError {
    #[error(transparent)]
    AccessorIterCreateError(#[from] AccessorIterCreateError),
}
