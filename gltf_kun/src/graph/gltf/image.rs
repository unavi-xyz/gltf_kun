use petgraph::{graph::NodeIndex, visit::EdgeRef};

use crate::graph::{gltf::GltfEdge, Edge, Graph, GraphNode, Property, Weight};

use super::{buffer::Buffer, GltfWeight};

#[derive(Debug, PartialEq, Eq)]
pub enum ImageEdge {
    Buffer,
}

#[derive(Debug, Default)]
pub struct ImageWeight {
    pub name: Option<String>,
    pub extras: gltf::json::Extras,

    pub mime_type: Option<String>,
    pub uri: Option<String>,

    pub data: Vec<u8>,
}

impl<'a> TryFrom<&'a Weight> for &'a ImageWeight {
    type Error = ();
    fn try_from(value: &'a Weight) -> Result<Self, Self::Error> {
        match value {
            Weight::Gltf(GltfWeight::Image(weight)) => Ok(weight),
            _ => Err(()),
        }
    }
}

impl<'a> TryFrom<&'a mut Weight> for &'a mut ImageWeight {
    type Error = ();
    fn try_from(value: &'a mut Weight) -> Result<Self, Self::Error> {
        match value {
            Weight::Gltf(GltfWeight::Image(weight)) => Ok(weight),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Image(pub NodeIndex);

impl From<NodeIndex> for Image {
    fn from(index: NodeIndex) -> Self {
        Self(index)
    }
}

impl From<Image> for NodeIndex {
    fn from(image: Image) -> Self {
        image.0
    }
}

impl GraphNode<ImageWeight> for Image {}
impl Property for Image {}

impl Image {
    pub fn new(graph: &mut Graph) -> Self {
        let index = graph.add_node(Weight::Gltf(GltfWeight::Image(Default::default())));
        Self(index)
    }

    pub fn buffer(&self, graph: &Graph) -> Option<Buffer> {
        graph
            .edges_directed(self.0, petgraph::Direction::Outgoing)
            .find(|edge| {
                matches!(
                    edge.weight(),
                    Edge::Gltf(GltfEdge::Image(ImageEdge::Buffer))
                )
            })
            .map(|edge| Buffer(edge.target()))
    }
    pub fn set_buffer(&self, graph: &mut Graph, source: Option<&Buffer>) {
        let edge = graph
            .edges_directed(self.0, petgraph::Direction::Outgoing)
            .find(|edge| {
                matches!(
                    edge.weight(),
                    Edge::Gltf(GltfEdge::Image(ImageEdge::Buffer))
                )
            })
            .map(|edge| edge.id());

        if let Some(edge) = edge {
            graph.remove_edge(edge);
        }

        if let Some(buffer) = source {
            graph.add_edge(
                self.0,
                buffer.0,
                Edge::Gltf(GltfEdge::Image(ImageEdge::Buffer)),
            );
        }
    }
}
