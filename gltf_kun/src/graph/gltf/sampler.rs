use petgraph::graph::NodeIndex;

use crate::graph::{Graph, GraphNode, Property, Weight};

use super::GltfWeight;

#[derive(Debug, PartialEq, Eq)]
pub enum SamplerEdge {}

#[derive(Debug, Default)]
pub struct SamplerWeight {
    pub name: Option<String>,
    pub extras: gltf::json::Extras,

    pub mag_filter: Option<MagFilter>,
    pub min_filter: Option<MinFilter>,
    pub wrap_s: Option<Wrap>,
    pub wrap_t: Option<Wrap>,
}

#[derive(Debug)]
#[repr(usize)]
pub enum MagFilter {
    Nearest = 9728,
    Linear = 9729,
    Other(usize) = 0,
}

#[derive(Debug)]
#[repr(usize)]
pub enum MinFilter {
    Nearest = 9728,
    Linear = 9729,
    NearestMipmapNearest = 9984,
    LinearMipmapNearest = 9985,
    NearestMipmapLinear = 9986,
    LinearMipmapLinear = 9987,
    Other(usize) = 0,
}

#[derive(Debug)]
#[repr(usize)]
pub enum Wrap {
    ClampToEdge = 33071,
    MirroredRepeat = 33648,
    Repeat = 10497,
    Other(usize) = 0,
}

impl<'a> TryFrom<&'a Weight> for &'a SamplerWeight {
    type Error = ();
    fn try_from(value: &'a Weight) -> Result<Self, Self::Error> {
        match value {
            Weight::Gltf(GltfWeight::Sampler(weight)) => Ok(weight),
            _ => Err(()),
        }
    }
}

impl<'a> TryFrom<&'a mut Weight> for &'a mut SamplerWeight {
    type Error = ();
    fn try_from(value: &'a mut Weight) -> Result<Self, Self::Error> {
        match value {
            Weight::Gltf(GltfWeight::Sampler(weight)) => Ok(weight),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Sampler(pub NodeIndex);

impl From<NodeIndex> for Sampler {
    fn from(index: NodeIndex) -> Self {
        Self(index)
    }
}

impl From<Sampler> for NodeIndex {
    fn from(sampler: Sampler) -> Self {
        sampler.0
    }
}

impl GraphNode<SamplerWeight> for Sampler {}
impl Property for Sampler {}

impl Sampler {
    pub fn new(graph: &mut Graph) -> Self {
        let index = graph.add_node(Weight::Gltf(GltfWeight::Sampler(Default::default())));
        Self(index)
    }
}
