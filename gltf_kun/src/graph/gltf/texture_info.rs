use petgraph::graph::NodeIndex;

use crate::graph::{Graph, GraphNode, Property, Weight};

use super::GltfWeight;

#[derive(Debug, PartialEq, Eq)]
pub enum TextureInfoEdge {
    Texture,
}

#[derive(Debug, Default)]
pub struct TextureInfoWeight {
    pub extras: gltf::json::Extras,
    pub tex_coord: usize,
}

impl<'a> TryFrom<&'a Weight> for &'a TextureInfoWeight {
    type Error = ();
    fn try_from(value: &'a Weight) -> Result<Self, Self::Error> {
        match value {
            Weight::Gltf(GltfWeight::TextureInfo(weight)) => Ok(weight),
            _ => Err(()),
        }
    }
}

impl<'a> TryFrom<&'a mut Weight> for &'a mut TextureInfoWeight {
    type Error = ();
    fn try_from(value: &'a mut Weight) -> Result<Self, Self::Error> {
        match value {
            Weight::Gltf(GltfWeight::TextureInfo(weight)) => Ok(weight),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TextureInfo(pub NodeIndex);

impl From<NodeIndex> for TextureInfo {
    fn from(index: NodeIndex) -> Self {
        Self(index)
    }
}

impl From<TextureInfo> for NodeIndex {
    fn from(texture_info: TextureInfo) -> Self {
        texture_info.0
    }
}

impl GraphNode<TextureInfoWeight> for TextureInfo {}
impl Property for TextureInfo {}

impl TextureInfo {
    pub fn new(graph: &mut Graph) -> Self {
        let index = graph.add_node(Weight::Gltf(GltfWeight::TextureInfo(Default::default())));
        Self(index)
    }
}
