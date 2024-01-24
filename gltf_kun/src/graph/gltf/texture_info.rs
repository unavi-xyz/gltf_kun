use petgraph::{graph::NodeIndex, visit::EdgeRef, Direction};

use crate::graph::{Edge, Graph, GraphNode, Property, Weight};

use super::{texture::Texture, GltfEdge, GltfWeight};

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

    pub fn texture(&self, graph: &Graph) -> Option<Texture> {
        graph
            .edges_directed(self.0, petgraph::Direction::Outgoing)
            .find_map(|edge| {
                if let Edge::Gltf(GltfEdge::TextureInfo(TextureInfoEdge::Texture)) = edge.weight() {
                    Some(Texture(edge.target()))
                } else {
                    None
                }
            })
    }
    pub fn set_texture(&self, graph: &mut Graph, texture: Option<&Texture>) {
        let edge = graph
            .edges_directed(self.0, Direction::Outgoing)
            .find(|edge| {
                matches!(
                    edge.weight(),
                    Edge::Gltf(GltfEdge::TextureInfo(TextureInfoEdge::Texture))
                )
            })
            .map(|edge| edge.id());

        if let Some(edge) = edge {
            graph.remove_edge(edge);
        }

        if let Some(texture) = texture {
            graph.add_edge(
                self.0,
                texture.0,
                Edge::Gltf(GltfEdge::TextureInfo(TextureInfoEdge::Texture)),
            );
        }
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

        texture_info.set_texture(&mut graph, Some(&texture));
        assert_eq!(texture_info.texture(&graph), Some(texture));

        texture_info.set_texture(&mut graph, None);
        assert!(texture_info.texture(&graph).is_none());
    }
}
