use petgraph::graph::NodeIndex;

use crate::graph::{
    Edge, Extensions, Graph, GraphNodeEdges, GraphNodeWeight, Weight, gltf::GltfEdge,
};

use super::{GltfWeight, buffer::Buffer};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ImageEdge {
    Buffer,
}

impl<'a> TryFrom<&'a Edge> for &'a ImageEdge {
    type Error = ();
    fn try_from(value: &'a Edge) -> Result<Self, Self::Error> {
        match value {
            Edge::Gltf(GltfEdge::Image(edge)) => Ok(edge),
            _ => Err(()),
        }
    }
}

impl From<ImageEdge> for Edge {
    fn from(edge: ImageEdge) -> Self {
        Self::Gltf(GltfEdge::Image(edge))
    }
}

#[derive(Clone, Debug, Default)]
pub struct ImageWeight {
    pub name: Option<String>,
    pub extras: gltf::json::Extras,

    pub mime_type: Option<String>,
    pub uri: Option<String>,

    pub data: Vec<u8>,
}

impl From<ImageWeight> for Weight {
    fn from(weight: ImageWeight) -> Self {
        Self::Gltf(GltfWeight::Image(weight))
    }
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

impl GraphNodeWeight<ImageWeight> for Image {}
impl GraphNodeEdges for Image {}
impl Extensions for Image {}

impl Image {
    pub fn buffer(&self, graph: &Graph) -> Option<Buffer> {
        self.find_edge_target(graph, &ImageEdge::Buffer)
    }
    pub fn set_buffer(&self, graph: &mut Graph, buffer: Option<Buffer>) {
        self.set_edge_target(graph, ImageEdge::Buffer, buffer);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buffer() {
        let graph = &mut Graph::default();

        let image = Image::new(graph);
        let buffer = Buffer::new(graph);

        image.set_buffer(graph, Some(buffer));
        assert_eq!(image.buffer(graph), Some(buffer));

        image.set_buffer(graph, None);
        assert_eq!(image.buffer(graph), None);
    }
}
