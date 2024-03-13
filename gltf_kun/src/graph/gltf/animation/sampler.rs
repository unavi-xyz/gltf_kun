use petgraph::graph::NodeIndex;

use crate::graph::{
    gltf::{Accessor, GltfEdge, GltfWeight},
    Edge, Extensions, Graph, GraphNodeEdges, GraphNodeWeight, Weight,
};

pub use gltf::animation::Interpolation;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AnimationSamplerEdge {
    Input,
    Output,
}

impl<'a> TryFrom<&'a Edge> for &'a AnimationSamplerEdge {
    type Error = ();
    fn try_from(value: &'a Edge) -> Result<Self, Self::Error> {
        match value {
            Edge::Gltf(GltfEdge::AnimationSampler(edge)) => Ok(edge),
            _ => Err(()),
        }
    }
}

impl From<AnimationSamplerEdge> for Edge {
    fn from(edge: AnimationSamplerEdge) -> Self {
        Self::Gltf(GltfEdge::AnimationSampler(edge))
    }
}

#[derive(Clone, Debug, Default)]
pub struct AnimationSamplerWeight {
    pub extras: gltf::json::Extras,
    pub interpolation: Interpolation,
}

impl From<AnimationSamplerWeight> for Weight {
    fn from(weight: AnimationSamplerWeight) -> Self {
        Self::Gltf(GltfWeight::AnimationSampler(weight))
    }
}

impl<'a> TryFrom<&'a Weight> for &'a AnimationSamplerWeight {
    type Error = ();
    fn try_from(value: &'a Weight) -> Result<Self, Self::Error> {
        match value {
            Weight::Gltf(GltfWeight::AnimationSampler(weight)) => Ok(weight),
            _ => Err(()),
        }
    }
}

impl<'a> TryFrom<&'a mut Weight> for &'a mut AnimationSamplerWeight {
    type Error = ();
    fn try_from(value: &'a mut Weight) -> Result<Self, Self::Error> {
        match value {
            Weight::Gltf(GltfWeight::AnimationSampler(weight)) => Ok(weight),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AnimationSampler(pub NodeIndex);

impl From<NodeIndex> for AnimationSampler {
    fn from(index: NodeIndex) -> Self {
        Self(index)
    }
}

impl From<AnimationSampler> for NodeIndex {
    fn from(image: AnimationSampler) -> Self {
        image.0
    }
}

impl GraphNodeWeight<AnimationSamplerWeight> for AnimationSampler {}
impl GraphNodeEdges for AnimationSampler {}
impl Extensions for AnimationSampler {}

impl AnimationSampler {
    pub fn input(&self, graph: &Graph) -> Option<Accessor> {
        self.find_edge_target(graph, &AnimationSamplerEdge::Input)
    }
    pub fn set_input(&self, graph: &mut Graph, input: Option<Accessor>) {
        self.set_edge_target(graph, AnimationSamplerEdge::Input, input);
    }

    pub fn output(&self, graph: &Graph) -> Option<Accessor> {
        self.find_edge_target(graph, &AnimationSamplerEdge::Output)
    }
    pub fn set_output(&self, graph: &mut Graph, output: Option<Accessor>) {
        self.set_edge_target(graph, AnimationSamplerEdge::Output, output);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input() {
        let mut graph = Graph::default();
        let sampler = AnimationSampler::new(&mut graph);

        let accessor = Accessor::new(&mut graph);
        sampler.set_input(&mut graph, Some(accessor));
        assert_eq!(sampler.input(&graph), Some(accessor));

        sampler.set_input(&mut graph, None);
        assert_eq!(sampler.input(&graph), None);
    }

    #[test]
    fn output() {
        let mut graph = Graph::default();
        let sampler = AnimationSampler::new(&mut graph);

        let accessor = Accessor::new(&mut graph);
        sampler.set_output(&mut graph, Some(accessor));
        assert_eq!(sampler.output(&graph), Some(accessor));

        sampler.set_output(&mut graph, None);
        assert_eq!(sampler.output(&graph), None);
    }
}
