use petgraph::graph::NodeIndex;

use crate::graph::{Edge, GraphNodeEdges, GraphNodeWeight, Property, Weight};

use super::{GltfEdge, GltfWeight};

#[derive(Debug, PartialEq, Eq)]
pub enum MaterialEdge {
    Texture,
}

impl<'a> TryFrom<&'a Edge> for &'a MaterialEdge {
    type Error = ();
    fn try_from(value: &'a Edge) -> Result<Self, Self::Error> {
        match value {
            Edge::Gltf(GltfEdge::Material(edge)) => Ok(edge),
            _ => Err(()),
        }
    }
}

impl From<MaterialEdge> for Edge {
    fn from(edge: MaterialEdge) -> Self {
        Self::Gltf(GltfEdge::Material(edge))
    }
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

impl From<MaterialWeight> for Weight {
    fn from(weight: MaterialWeight) -> Self {
        Self::Gltf(GltfWeight::Material(weight))
    }
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

impl GraphNodeWeight<MaterialWeight> for Material {}
impl GraphNodeEdges<MaterialEdge> for Material {}
impl Property for Material {}
