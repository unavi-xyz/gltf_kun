use petgraph::graph::NodeIndex;

use crate::graph::{Edge, Extensions, Graph, GraphNodeEdges, GraphNodeWeight, Weight};

use super::{image::Image, GltfEdge, GltfWeight};

pub use gltf::texture::{MagFilter, MinFilter, WrappingMode};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TextureEdge {
    Image,
}

impl<'a> TryFrom<&'a Edge> for &'a TextureEdge {
    type Error = ();
    fn try_from(value: &'a Edge) -> Result<Self, Self::Error> {
        match value {
            Edge::Gltf(GltfEdge::Texture(edge)) => Ok(edge),
            _ => Err(()),
        }
    }
}

impl From<TextureEdge> for Edge {
    fn from(edge: TextureEdge) -> Self {
        Self::Gltf(GltfEdge::Texture(edge))
    }
}

#[derive(Clone, Debug, Default)]
pub struct TextureWeight {
    pub name: Option<String>,
    pub extras: gltf::json::Extras,

    pub mag_filter: Option<MagFilter>,
    pub min_filter: Option<MinFilter>,
    pub wrap_s: WrappingMode,
    pub wrap_t: WrappingMode,
}

impl From<TextureWeight> for Weight {
    fn from(weight: TextureWeight) -> Self {
        Self::Gltf(GltfWeight::Texture(weight))
    }
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

impl GraphNodeWeight<TextureWeight> for Texture {}
impl GraphNodeEdges<TextureEdge> for Texture {}
impl Extensions for Texture {}

impl Texture {
    pub fn image(&self, graph: &Graph) -> Option<Image> {
        self.find_edge_target(graph, &TextureEdge::Image)
    }
    pub fn set_image(&self, graph: &mut Graph, image: Option<Image>) {
        self.set_edge_target(graph, TextureEdge::Image, image);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn image() {
        let mut graph = Graph::default();

        let texture = Texture::new(&mut graph);
        let image = Image::new(&mut graph);

        texture.set_image(&mut graph, Some(image));
        assert_eq!(texture.image(&graph), Some(image));

        texture.set_image(&mut graph, None);
        assert!(texture.image(&graph).is_none());
    }
}
