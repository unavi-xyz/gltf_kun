use petgraph::graph::NodeIndex;

use crate::graph::{Graph, GraphNode, Property, Weight};

use super::GltfWeight;

#[derive(Debug, PartialEq, Eq)]
pub enum TextureEdge {
    Sampler,
    Source,
}

#[derive(Debug, Default)]
pub struct TextureWeight {
    pub name: Option<String>,
    pub extras: gltf::json::Extras,
}

impl<'a> TryFrom<&'a Weight> for &'a TextureWeight {
    type Error = ();
    fn try_from(value: &'a Weight) -> Result<Self, Self::Error> {
        match value {
            Weight::Gltf(GltfWeight::Texture(weight)) => Ok(weight),
            _ => Err(()),
        }
    }
}

impl<'a> TryFrom<&'a mut Weight> for &'a mut TextureWeight {
    type Error = ();
    fn try_from(value: &'a mut Weight) -> Result<Self, Self::Error> {
        match value {
            Weight::Gltf(GltfWeight::Texture(weight)) => Ok(weight),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Texture(pub NodeIndex);

impl From<NodeIndex> for Texture {
    fn from(index: NodeIndex) -> Self {
        Self(index)
    }
}

impl From<Texture> for NodeIndex {
    fn from(texture: Texture) -> Self {
        texture.0
    }
}

impl GraphNode<TextureWeight> for Texture {}
impl Property for Texture {}

impl Texture {
    pub fn new(graph: &mut Graph) -> Self {
        let index = graph.add_node(Weight::Gltf(GltfWeight::Texture(Default::default())));
        Self(index)
    }
}
