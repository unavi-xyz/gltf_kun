use petgraph::graph::NodeIndex;

use crate::graph::{Edge, Extensions, Graph, GraphNodeEdges, GraphNodeWeight, Weight};

use super::{image::Image, GltfEdge, GltfWeight};

pub use gltf::texture::{MagFilter, MinFilter, WrappingMode};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TextureInfoEdge {
    Image,
}

impl<'a> TryFrom<&'a Edge> for &'a TextureInfoEdge {
    type Error = ();
    fn try_from(value: &'a Edge) -> Result<Self, Self::Error> {
        match value {
            Edge::Gltf(GltfEdge::TextureInfo(edge)) => Ok(edge),
            _ => Err(()),
        }
    }
}

impl From<TextureInfoEdge> for Edge {
    fn from(edge: TextureInfoEdge) -> Self {
        Self::Gltf(GltfEdge::TextureInfo(edge))
    }
}

#[derive(Clone, Debug, Default)]
pub struct TextureInfoWeight {
    pub extras: gltf::json::Extras,

    pub tex_coord: usize,

    pub mag_filter: Option<MagFilter>,
    pub min_filter: Option<MinFilter>,
    pub wrap_s: WrappingMode,
    pub wrap_t: WrappingMode,
}

impl From<TextureInfoWeight> for Weight {
    fn from(weight: TextureInfoWeight) -> Self {
        Self::Gltf(GltfWeight::TextureInfo(weight))
    }
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

impl GraphNodeWeight<TextureInfoWeight> for TextureInfo {}
impl GraphNodeEdges<TextureInfoEdge> for TextureInfo {}
impl Extensions for TextureInfo {}

impl TextureInfo {
    pub fn image(&self, graph: &Graph) -> Option<Image> {
        self.find_edge_target(graph, &TextureInfoEdge::Image)
    }
    pub fn set_image(&self, graph: &mut Graph, image: Option<Image>) {
        self.set_edge_target(graph, TextureInfoEdge::Image, image);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn image() {
        let mut graph = Graph::default();

        let texture_info = TextureInfo::new(&mut graph);
        let image = Image::new(&mut graph);

        texture_info.set_image(&mut graph, Some(image));
        assert_eq!(texture_info.image(&graph), Some(image));

        texture_info.set_image(&mut graph, None);
        assert!(texture_info.image(&graph).is_none());
    }
}
