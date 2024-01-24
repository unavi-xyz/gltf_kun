use petgraph::{graph::NodeIndex, visit::EdgeRef, Direction};

use crate::graph::{Edge, Graph, GraphNode, Property, Weight};

use super::{image::Image, sampler::Sampler, GltfEdge, GltfWeight};

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

    pub fn sampler(&self, graph: &Graph) -> Option<Sampler> {
        graph
            .edges_directed(self.0, Direction::Outgoing)
            .find_map(|edge| {
                if let Edge::Gltf(GltfEdge::Texture(TextureEdge::Sampler)) = edge.weight() {
                    Some(Sampler(edge.target()))
                } else {
                    None
                }
            })
    }
    pub fn set_sampler(&self, graph: &mut Graph, sampler: Option<&Sampler>) {
        let edge = graph
            .edges_directed(self.0, Direction::Outgoing)
            .find(|edge| {
                matches!(
                    edge.weight(),
                    Edge::Gltf(GltfEdge::Texture(TextureEdge::Sampler))
                )
            })
            .map(|edge| edge.id());

        if let Some(edge) = edge {
            graph.remove_edge(edge);
        }

        if let Some(sampler) = sampler {
            graph.add_edge(
                self.0,
                sampler.0,
                Edge::Gltf(GltfEdge::Texture(TextureEdge::Sampler)),
            );
        }
    }

    pub fn source(&self, graph: &Graph) -> Option<Image> {
        graph
            .edges_directed(self.0, Direction::Outgoing)
            .find_map(|edge| {
                if let Edge::Gltf(GltfEdge::Texture(TextureEdge::Source)) = edge.weight() {
                    Some(Image(edge.target()))
                } else {
                    None
                }
            })
    }
    pub fn set_source(&self, graph: &mut Graph, source: Option<&Image>) {
        let edge = graph
            .edges_directed(self.0, Direction::Outgoing)
            .find(|edge| {
                matches!(
                    edge.weight(),
                    Edge::Gltf(GltfEdge::Texture(TextureEdge::Source))
                )
            })
            .map(|edge| edge.id());

        if let Some(edge) = edge {
            graph.remove_edge(edge);
        }

        if let Some(source) = source {
            graph.add_edge(
                self.0,
                source.0,
                Edge::Gltf(GltfEdge::Texture(TextureEdge::Source)),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sampler() {
        let mut graph = Graph::new();

        let texture = Texture::new(&mut graph);
        let sampler = Sampler::new(&mut graph);

        texture.set_sampler(&mut graph, Some(&sampler));
        assert_eq!(texture.sampler(&graph), Some(sampler));
    }

    #[test]
    fn test_source() {
        let mut graph = Graph::new();

        let texture = Texture::new(&mut graph);
        let image = Image::new(&mut graph);

        texture.set_source(&mut graph, Some(&image));
        assert_eq!(texture.source(&graph), Some(image));
    }
}
