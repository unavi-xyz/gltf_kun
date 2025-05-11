use petgraph::graph::NodeIndex;

use crate::graph::{
    Edge, Extensions, Graph, GraphNodeEdges, GraphNodeWeight, Weight, gltf::GltfEdge,
};

use super::GltfWeight;

mod channel;
mod sampler;

pub use channel::*;
pub use sampler::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AnimationEdge {
    Channel,
}

impl<'a> TryFrom<&'a Edge> for &'a AnimationEdge {
    type Error = ();
    fn try_from(value: &'a Edge) -> Result<Self, Self::Error> {
        match value {
            Edge::Gltf(GltfEdge::Animation(edge)) => Ok(edge),
            _ => Err(()),
        }
    }
}

impl From<AnimationEdge> for Edge {
    fn from(edge: AnimationEdge) -> Self {
        Self::Gltf(GltfEdge::Animation(edge))
    }
}

#[derive(Clone, Debug, Default)]
pub struct AnimationWeight {
    pub name: Option<String>,
    pub extras: gltf::json::Extras,
}

impl From<AnimationWeight> for Weight {
    fn from(weight: AnimationWeight) -> Self {
        Self::Gltf(GltfWeight::Animation(weight))
    }
}

impl<'a> TryFrom<&'a Weight> for &'a AnimationWeight {
    type Error = ();
    fn try_from(value: &'a Weight) -> Result<Self, Self::Error> {
        match value {
            Weight::Gltf(GltfWeight::Animation(weight)) => Ok(weight),
            _ => Err(()),
        }
    }
}

impl<'a> TryFrom<&'a mut Weight> for &'a mut AnimationWeight {
    type Error = ();
    fn try_from(value: &'a mut Weight) -> Result<Self, Self::Error> {
        match value {
            Weight::Gltf(GltfWeight::Animation(weight)) => Ok(weight),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Animation(pub NodeIndex);

impl From<NodeIndex> for Animation {
    fn from(index: NodeIndex) -> Self {
        Self(index)
    }
}

impl From<Animation> for NodeIndex {
    fn from(image: Animation) -> Self {
        image.0
    }
}

impl GraphNodeWeight<AnimationWeight> for Animation {}
impl GraphNodeEdges for Animation {}
impl Extensions for Animation {}

impl Animation {
    pub fn channels(&self, graph: &Graph) -> Vec<AnimationChannel> {
        self.edge_targets(graph, &AnimationEdge::Channel)
    }
    pub fn add_channel(&self, graph: &mut Graph, channel: &AnimationChannel) {
        self.add_edge_target(graph, AnimationEdge::Channel, *channel);
    }
    pub fn remove_channel(&self, graph: &mut Graph, channel: &AnimationChannel) {
        self.remove_edge_target(graph, AnimationEdge::Channel, *channel);
    }
    pub fn create_channel(&self, graph: &mut Graph) -> AnimationChannel {
        self.create_edge_target(graph, AnimationEdge::Channel)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn channels() {
        let mut graph = Graph::default();
        let animation = Animation::new(&mut graph);

        let channel = animation.create_channel(&mut graph);
        assert_eq!(animation.channels(&graph), vec![channel]);

        let channel2 = AnimationChannel::new(&mut graph);
        animation.add_channel(&mut graph, &channel2);
        assert_eq!(animation.channels(&graph), vec![channel, channel2]);

        animation.remove_channel(&mut graph, &channel);
        assert_eq!(animation.channels(&graph), vec![channel2]);
    }
}
