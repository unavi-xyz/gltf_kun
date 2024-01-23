use petgraph::graph::NodeIndex;

use crate::graph::{Graph, GraphNode, Property, Weight};

use super::GltfWeight;

#[derive(Debug, PartialEq, Eq)]
pub enum ImageEdge {
    BufferView,
}

#[derive(Debug, Default)]
pub struct ImageWeight {
    pub name: Option<String>,
    pub extras: gltf::json::Extras,

    pub uri: Option<String>,
    pub mime_type: Option<String>,
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
}
