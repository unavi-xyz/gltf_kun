use petgraph::{graph::NodeIndex, visit::EdgeRef, Direction};

use crate::graph::{Edge, Graph, GraphNodeEdges, GraphNodeWeight, Property, Weight};

use super::{image::Image, sampler::Sampler, GltfEdge, GltfWeight};

#[derive(Debug, PartialEq, Eq)]
pub enum TextureEdge {
    Sampler,
    Source,
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

impl GraphNodeWeight<TextureWeight> for Texture {}
impl GraphNodeEdges<TextureEdge> for Texture {}
impl Property for Texture {}

impl Texture {
    pub fn new(graph: &mut Graph) -> Self {
        let index = graph.add_node(Weight::Gltf(GltfWeight::Texture(Default::default())));
        Self(index)
    }

    pub fn sampler(&self, graph: &Graph) -> Option<Sampler> {
        self.find_edge_target::<Sampler>(graph, &TextureEdge::Sampler)
    }
    pub fn set_sampler(&self, graph: &mut Graph, sampler: Option<Sampler>) {
        self.set_edge_target(graph, TextureEdge::Sampler, sampler);
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
    fn sampler() {
        let mut graph = Graph::new();

        let texture = Texture::new(&mut graph);
        let sampler = Sampler::new(&mut graph);

        texture.set_sampler(&mut graph, Some(sampler));
        assert_eq!(texture.sampler(&graph), Some(sampler));

        texture.set_sampler(&mut graph, None);
        assert!(texture.sampler(&graph).is_none());
    }

    #[test]
    fn source() {
        let mut graph = Graph::new();

        let texture = Texture::new(&mut graph);
        let image = Image::new(&mut graph);

        texture.set_source(&mut graph, Some(&image));
        assert_eq!(texture.source(&graph), Some(image));

        texture.set_source(&mut graph, None);
        assert!(texture.source(&graph).is_none());
    }
}
