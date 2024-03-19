use petgraph::graph::NodeIndex;

use crate::graph::{Edge, Extensions, Graph, GraphNodeEdges, GraphNodeWeight, Weight};

use super::{GltfEdge, GltfWeight, Texture};

pub use gltf::json::material::{AlphaCutoff, AlphaMode};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MaterialEdge {
    BaseColorTexture,
    EmissiveTexture,
    MetallicRoughnessTexture,
    NormalTexture,
    OcclusionTexture,
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

#[derive(Clone, Debug)]
pub struct MaterialWeight {
    pub name: Option<String>,
    pub extras: gltf::json::Extras,

    pub alpha_cutoff: AlphaCutoff,
    pub alpha_mode: AlphaMode,
    pub base_color_factor: [f32; 4],
    pub double_sided: bool,
    pub emissive_factor: [f32; 3],
    pub metallic_factor: f32,
    pub normal_scale: f32,
    pub occlusion_strength: f32,
    pub roughness_factor: f32,

    pub base_color_tex_coord: usize,
    pub emissive_tex_coord: usize,
    pub metallic_roughness_tex_coord: usize,
    pub normal_tex_coord: usize,
    pub occlusion_tex_coord: usize,
}

impl Default for MaterialWeight {
    fn default() -> Self {
        Self {
            name: None,
            extras: Default::default(),

            alpha_cutoff: AlphaCutoff::default(),
            alpha_mode: AlphaMode::default(),
            base_color_factor: [1.0, 1.0, 1.0, 1.0],
            double_sided: false,
            emissive_factor: [0.0, 0.0, 0.0],
            metallic_factor: 1.0,
            normal_scale: 1.0,
            occlusion_strength: 1.0,
            roughness_factor: 1.0,

            base_color_tex_coord: 0,
            emissive_tex_coord: 0,
            metallic_roughness_tex_coord: 0,
            normal_tex_coord: 0,
            occlusion_tex_coord: 0,
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
impl GraphNodeEdges for Material {}
impl Extensions for Material {}

impl Material {
    pub fn base_color_texture(&self, graph: &Graph) -> Option<Texture> {
        self.find_edge_target(graph, &MaterialEdge::BaseColorTexture)
    }
    pub fn set_base_color_texture(&self, graph: &mut Graph, texture: Option<Texture>) {
        self.set_edge_target(graph, MaterialEdge::BaseColorTexture, texture);
    }

    pub fn emissive_texture(&self, graph: &Graph) -> Option<Texture> {
        self.find_edge_target(graph, &MaterialEdge::EmissiveTexture)
    }
    pub fn set_emissive_texture(&self, graph: &mut Graph, texture: Option<Texture>) {
        self.set_edge_target(graph, MaterialEdge::EmissiveTexture, texture);
    }

    pub fn metallic_roughness_texture(&self, graph: &Graph) -> Option<Texture> {
        self.find_edge_target(graph, &MaterialEdge::MetallicRoughnessTexture)
    }
    pub fn set_metallic_roughness_texture(&self, graph: &mut Graph, texture: Option<Texture>) {
        self.set_edge_target(graph, MaterialEdge::MetallicRoughnessTexture, texture);
    }

    pub fn normal_texture(&self, graph: &Graph) -> Option<Texture> {
        self.find_edge_target(graph, &MaterialEdge::NormalTexture)
    }
    pub fn set_normal_texture(&self, graph: &mut Graph, texture: Option<Texture>) {
        self.set_edge_target(graph, MaterialEdge::NormalTexture, texture);
    }

    pub fn occlusion_texture(&self, graph: &Graph) -> Option<Texture> {
        self.find_edge_target(graph, &MaterialEdge::OcclusionTexture)
    }
    pub fn set_occlusion_texture(&self, graph: &mut Graph, texture: Option<Texture>) {
        self.set_edge_target(graph, MaterialEdge::OcclusionTexture, texture);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_color_texture() {
        let graph = &mut Graph::default();

        let material = Material::new(graph);
        let texture = Texture::new(graph);

        material.set_base_color_texture(graph, Some(texture));
        assert_eq!(material.base_color_texture(graph), Some(texture));

        material.set_base_color_texture(graph, None);
        assert!(material.base_color_texture(graph).is_none());
    }

    #[test]
    fn emmissive_texture() {
        let graph = &mut Graph::default();

        let material = Material::new(graph);
        let texture = Texture::new(graph);

        material.set_emissive_texture(graph, Some(texture));
        assert_eq!(material.emissive_texture(graph), Some(texture));

        material.set_emissive_texture(graph, None);
        assert!(material.emissive_texture(graph).is_none());
    }

    #[test]
    fn metallic_roughness_texture() {
        let graph = &mut Graph::default();

        let material = Material::new(graph);
        let texture = Texture::new(graph);

        material.set_metallic_roughness_texture(graph, Some(texture));
        assert_eq!(material.metallic_roughness_texture(graph), Some(texture));

        material.set_metallic_roughness_texture(graph, None);
        assert!(material.metallic_roughness_texture(graph).is_none());
    }

    #[test]
    fn normal_texture() {
        let graph = &mut Graph::default();

        let material = Material::new(graph);
        let texture = Texture::new(graph);

        material.set_normal_texture(graph, Some(texture));
        assert_eq!(material.normal_texture(graph), Some(texture));

        material.set_normal_texture(graph, None);
        assert!(material.normal_texture(graph).is_none());
    }

    #[test]
    fn occlusion_texture() {
        let graph = &mut Graph::default();

        let material = Material::new(graph);
        let texture = Texture::new(graph);

        material.set_occlusion_texture(graph, Some(texture));
        assert_eq!(material.occlusion_texture(graph), Some(texture));

        material.set_occlusion_texture(graph, None);
        assert!(material.occlusion_texture(graph).is_none());
    }
}
