use petgraph::graph::NodeIndex;

use crate::graph::{
    gltf::{GltfEdge, Node},
    Edge, Extensions, Graph, GraphNodeEdges, GraphNodeWeight, Weight,
};

use super::{AnimationSampler, GltfWeight};

pub use gltf::animation::Property as TargetPath;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AnimationChannelEdge {
    Sampler,
    Target,
}

impl<'a> TryFrom<&'a Edge> for &'a AnimationChannelEdge {
    type Error = ();
    fn try_from(value: &'a Edge) -> Result<Self, Self::Error> {
        match value {
            Edge::Gltf(GltfEdge::AnimationChannel(edge)) => Ok(edge),
            _ => Err(()),
        }
    }
}

impl From<AnimationChannelEdge> for Edge {
    fn from(edge: AnimationChannelEdge) -> Self {
        Self::Gltf(GltfEdge::AnimationChannel(edge))
    }
}

#[derive(Clone, Debug)]
pub struct AnimationChannelWeight {
    pub extras: gltf::json::Extras,
    pub path: TargetPath,
}

impl Default for AnimationChannelWeight {
    fn default() -> Self {
        Self {
            extras: Default::default(),
            path: TargetPath::Translation,
        }
    }
}

impl From<AnimationChannelWeight> for Weight {
    fn from(weight: AnimationChannelWeight) -> Self {
        Self::Gltf(GltfWeight::AnimationChannel(weight))
    }
}

impl<'a> TryFrom<&'a Weight> for &'a AnimationChannelWeight {
    type Error = ();
    fn try_from(value: &'a Weight) -> Result<Self, Self::Error> {
        match value {
            Weight::Gltf(GltfWeight::AnimationChannel(weight)) => Ok(weight),
            _ => Err(()),
        }
    }
}

impl<'a> TryFrom<&'a mut Weight> for &'a mut AnimationChannelWeight {
    type Error = ();
    fn try_from(value: &'a mut Weight) -> Result<Self, Self::Error> {
        match value {
            Weight::Gltf(GltfWeight::AnimationChannel(weight)) => Ok(weight),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AnimationChannel(pub NodeIndex);

impl From<NodeIndex> for AnimationChannel {
    fn from(index: NodeIndex) -> Self {
        Self(index)
    }
}

impl From<AnimationChannel> for NodeIndex {
    fn from(image: AnimationChannel) -> Self {
        image.0
    }
}

impl GraphNodeWeight<AnimationChannelWeight> for AnimationChannel {}
impl GraphNodeEdges<AnimationChannelEdge> for AnimationChannel {}
impl Extensions for AnimationChannel {}

impl AnimationChannel {
    pub fn sampler(&self, graph: &Graph) -> Option<AnimationSampler> {
        self.find_edge_target(graph, &AnimationChannelEdge::Sampler)
    }
    pub fn set_sampler(&self, graph: &mut Graph, sampler: Option<AnimationSampler>) {
        self.set_edge_target(graph, AnimationChannelEdge::Sampler, sampler);
    }

    pub fn target(&self, graph: &Graph) -> Option<Node> {
        self.find_edge_target(graph, &AnimationChannelEdge::Target)
    }
    pub fn set_target(&self, graph: &mut Graph, target: Option<Node>) {
        self.set_edge_target(graph, AnimationChannelEdge::Target, target);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sampler() {
        let mut graph = Graph::default();
        let channel = AnimationChannel::new(&mut graph);

        let sampler = AnimationSampler::new(&mut graph);
        channel.set_sampler(&mut graph, Some(sampler));
        assert_eq!(channel.sampler(&graph), Some(sampler));

        channel.set_sampler(&mut graph, None);
        assert_eq!(channel.sampler(&graph), None);
    }

    #[test]
    fn target() {
        let mut graph = Graph::default();
        let channel = AnimationChannel::new(&mut graph);

        let target = Node::new(&mut graph);
        channel.set_target(&mut graph, Some(target));
        assert_eq!(channel.target(&graph), Some(target));

        channel.set_target(&mut graph, None);
        assert_eq!(channel.target(&graph), None);
    }
}
