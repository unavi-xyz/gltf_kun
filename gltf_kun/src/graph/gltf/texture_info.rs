use petgraph::graph::NodeIndex;

use crate::graph::{Edge, Graph, GraphNodeEdges, GraphNodeWeight, Property, Weight};

use super::{texture::Texture, GltfEdge, GltfWeight};

#[derive(Debug, PartialEq, Eq)]
pub enum TextureInfoEdge {
    Texture,
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
    pub fn texture(&self, graph: &Graph) -> Option<Texture> {
        self.find_edge_target(graph, &TextureInfoEdge::Texture)
    }
    pub fn set_texture(&self, graph: &mut Graph, texture: Option<Texture>) {
        self.set_edge_target(graph, TextureInfoEdge::Texture, texture);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn texture() {
        let mut graph = Graph::new();

        let texture_info = TextureInfo::new(&mut graph);
        let texture = Texture::new(&mut graph);

        texture_info.set_texture(&mut graph, Some(texture));
        assert_eq!(texture_info.texture(&graph), Some(texture));

        texture_info.set_texture(&mut graph, None);
        assert!(texture_info.texture(&graph).is_none());
    }
}
