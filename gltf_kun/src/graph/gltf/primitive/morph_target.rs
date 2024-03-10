use gltf::Semantic;
use petgraph::{graph::NodeIndex, visit::EdgeRef};

use crate::graph::{
    gltf::{Accessor, GltfEdge},
    Edge, Graph, GraphNodeEdges, Property,
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

impl GraphNodeEdges<MorphTargetEdge> for MorphTarget {}
impl Property for MorphTarget {}

impl MorphTarget {
    pub fn attributes(&self, graph: &Graph) -> Vec<(Semantic, Accessor)> {
        graph
            .edges_directed(self.0, petgraph::Direction::Outgoing)
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
    pub fn attribute(&self, graph: &Graph, semantic: &Semantic) -> Option<Accessor> {
        self.find_edge_target(graph, &MorphTargetEdge::Attribute(semantic.clone()))
    }
    pub fn set_attribute(
        &self,
        graph: &mut Graph,
        semantic: &Semantic,
        accessor: Option<Accessor>,
    ) {
        self.set_edge_target(
            graph,
            MorphTargetEdge::Attribute(semantic.clone()),
            accessor,
        );
    }
}
