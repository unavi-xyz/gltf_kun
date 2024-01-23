use petgraph::graph::NodeIndex;

use crate::graph::{Graph, GraphNode, Property, Weight};

use super::GltfWeight;

#[derive(Debug, PartialEq, Eq)]
pub enum MaterialEdge {
    Texture,
}

#[derive(Debug, Default)]
pub struct MaterialWeight {
    pub name: Option<String>,
    pub extras: gltf::json::Extras,

    pub alpha_cutoff: AlphaCutoff,
    pub alpha_mode: AlphaMode,
    pub double_sided: bool,
    pub emissive_factor: [f32; 3],
}

#[derive(Debug)]
pub struct AlphaCutoff(pub f32);

impl Default for AlphaCutoff {
    fn default() -> Self {
        Self(0.5)
    }
}

#[derive(Debug, Default)]
pub enum AlphaMode {
    #[default]
    Opaque,
    Mask,
    Blend,
    Other(String),
}

impl<'a> TryFrom<&'a Weight> for &'a MaterialWeight {
    type Error = ();
    fn try_from(value: &'a Weight) -> Result<Self, Self::Error> {
        match value {
            Weight::Gltf(GltfWeight::Material(weight)) => Ok(weight),
            _ => Err(()),
        }
    }
}

impl<'a> TryFrom<&'a mut Weight> for &'a mut MaterialWeight {
    type Error = ();
    fn try_from(value: &'a mut Weight) -> Result<Self, Self::Error> {
        match value {
            Weight::Gltf(GltfWeight::Material(weight)) => Ok(weight),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Material(pub NodeIndex);

impl From<NodeIndex> for Material {
    fn from(index: NodeIndex) -> Self {
        Self(index)
    }
}

impl From<Material> for NodeIndex {
    fn from(material: Material) -> Self {
        material.0
    }
}

impl GraphNode<MaterialWeight> for Material {}
impl Property for Material {}

impl Material {
    pub fn new(graph: &mut Graph) -> Self {
        let index = graph.add_node(Weight::Gltf(
            GltfWeight::Material(MaterialWeight::default()),
        ));
        Self(index)
    }
}
