use petgraph::graph::NodeIndex;

use crate::graph::{Edge, Graph, GraphNodeEdges, GraphNodeWeight, Property, Weight};

use super::{image::Image, GltfEdge, GltfWeight};

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, Default)]
pub struct TextureInfoWeight {
    pub extras: gltf::json::Extras,

    pub tex_coord: usize,

    pub mag_filter: Option<MagFilter>,
    pub min_filter: Option<MinFilter>,
    pub wrap_s: Option<Wrap>,
    pub wrap_t: Option<Wrap>,
}

#[derive(Copy, Clone, Debug)]
#[repr(usize)]
pub enum MagFilter {
    Nearest = 9728,
    Linear = 9729,
    Other(usize) = 0,
}

#[derive(Copy, Clone, Debug)]
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

#[derive(Copy, Clone, Debug)]
#[repr(usize)]
pub enum Wrap {
    ClampToEdge = 33071,
    MirroredRepeat = 33648,
    Repeat = 10497,
    Other(usize) = 0,
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
impl Property for TextureInfo {}

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
        let mut graph = Graph::new();

        let texture_info = TextureInfo::new(&mut graph);
        let image = Image::new(&mut graph);

        texture_info.set_image(&mut graph, Some(image));
        assert_eq!(texture_info.image(&graph), Some(image));

        texture_info.set_image(&mut graph, None);
        assert!(texture_info.image(&graph).is_none());
    }
}
