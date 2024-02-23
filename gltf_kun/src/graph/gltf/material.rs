use petgraph::graph::NodeIndex;

use crate::graph::{Edge, Graph, GraphNodeEdges, GraphNodeWeight, Property, Weight};

use super::{texture_info::TextureInfo, GltfEdge, GltfWeight};

pub use gltf::json::material::{AlphaCutoff, AlphaMode};

#[derive(Debug, PartialEq, Eq)]
pub enum MaterialEdge {
    BaseColorTextureInfo,
    EmissiveTextureInfo,
    MetallicRoughnessTextureInfo,
    NormalTextureInfo,
    OcclusionTextureInfo,
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

#[derive(Debug)]
pub struct MaterialWeight {
    pub name: Option<String>,
    pub extras: gltf::json::Extras,

    pub alpha_cutoff: AlphaCutoff,
    pub alpha_mode: AlphaMode,
    pub double_sided: bool,
    pub emissive_factor: [f32; 3],

    pub base_color_factor: [f32; 4],
    pub metallic_factor: f32,
    pub roughness_factor: f32,

    pub normal_scale: f32,
    pub occlusion_strength: f32,
}

impl Default for MaterialWeight {
    fn default() -> Self {
        Self {
            name: None,
            extras: Default::default(),

            alpha_cutoff: AlphaCutoff::default(),
            alpha_mode: AlphaMode::default(),
            double_sided: false,
            emissive_factor: [0.0, 0.0, 0.0],

            base_color_factor: [1.0, 1.0, 1.0, 1.0],
            metallic_factor: 1.0,
            roughness_factor: 1.0,

            normal_scale: 1.0,
            occlusion_strength: 1.0,
        }
    }
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

impl Material {
    pub fn base_color_texture_info(&self, graph: &Graph) -> Option<TextureInfo> {
        self.find_edge_target(graph, &MaterialEdge::BaseColorTextureInfo)
    }
    pub fn set_base_color_texture_info(
        &self,
        graph: &mut Graph,
        texture_info: Option<TextureInfo>,
    ) {
        self.set_edge_target(graph, MaterialEdge::BaseColorTextureInfo, texture_info);
    }

    pub fn emissive_texture_info(&self, graph: &Graph) -> Option<TextureInfo> {
        self.find_edge_target(graph, &MaterialEdge::EmissiveTextureInfo)
    }
    pub fn set_emissive_texture_info(&self, graph: &mut Graph, texture_info: Option<TextureInfo>) {
        self.set_edge_target(graph, MaterialEdge::EmissiveTextureInfo, texture_info);
    }

    pub fn metallic_roughness_texture_info(&self, graph: &Graph) -> Option<TextureInfo> {
        self.find_edge_target(graph, &MaterialEdge::MetallicRoughnessTextureInfo)
    }
    pub fn set_metallic_roughness_texture_info(
        &self,
        graph: &mut Graph,
        texture_info: Option<TextureInfo>,
    ) {
        self.set_edge_target(
            graph,
            MaterialEdge::MetallicRoughnessTextureInfo,
            texture_info,
        );
    }

    pub fn normal_texture_info(&self, graph: &Graph) -> Option<TextureInfo> {
        self.find_edge_target(graph, &MaterialEdge::NormalTextureInfo)
    }
    pub fn set_normal_texture_info(&self, graph: &mut Graph, texture_info: Option<TextureInfo>) {
        self.set_edge_target(graph, MaterialEdge::NormalTextureInfo, texture_info);
    }

    pub fn occlusion_texture_info(&self, graph: &Graph) -> Option<TextureInfo> {
        self.find_edge_target(graph, &MaterialEdge::OcclusionTextureInfo)
    }
    pub fn set_occlusion_texture_info(&self, graph: &mut Graph, texture_info: Option<TextureInfo>) {
        self.set_edge_target(graph, MaterialEdge::OcclusionTextureInfo, texture_info);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_color_texture_info() {
        let graph = &mut Graph::new();

        let material = Material::new(graph);
        let texture_info = TextureInfo::new(graph);

        material.set_base_color_texture_info(graph, Some(texture_info));
        assert_eq!(material.base_color_texture_info(graph), Some(texture_info));

        material.set_base_color_texture_info(graph, None);
        assert!(material.base_color_texture_info(graph).is_none());
    }

    #[test]
    fn emmissive_texture_info() {
        let graph = &mut Graph::new();

        let material = Material::new(graph);
        let texture_info = TextureInfo::new(graph);

        material.set_emissive_texture_info(graph, Some(texture_info));
        assert_eq!(material.emissive_texture_info(graph), Some(texture_info));

        material.set_emissive_texture_info(graph, None);
        assert!(material.emissive_texture_info(graph).is_none());
    }

    #[test]
    fn metallic_roughness_texture_info() {
        let graph = &mut Graph::new();

        let material = Material::new(graph);
        let texture_info = TextureInfo::new(graph);

        material.set_metallic_roughness_texture_info(graph, Some(texture_info));
        assert_eq!(
            material.metallic_roughness_texture_info(graph),
            Some(texture_info)
        );

        material.set_metallic_roughness_texture_info(graph, None);
        assert!(material.metallic_roughness_texture_info(graph).is_none());
    }

    #[test]
    fn normal_texture_info() {
        let graph = &mut Graph::new();

        let material = Material::new(graph);
        let texture_info = TextureInfo::new(graph);

        material.set_normal_texture_info(graph, Some(texture_info));
        assert_eq!(material.normal_texture_info(graph), Some(texture_info));

        material.set_normal_texture_info(graph, None);
        assert!(material.normal_texture_info(graph).is_none());
    }

    #[test]
    fn occlusion_texture_info() {
        let graph = &mut Graph::new();

        let material = Material::new(graph);
        let texture_info = TextureInfo::new(graph);

        material.set_occlusion_texture_info(graph, Some(texture_info));
        assert_eq!(material.occlusion_texture_info(graph), Some(texture_info));

        material.set_occlusion_texture_info(graph, None);
        assert!(material.occlusion_texture_info(graph).is_none());
    }
}
