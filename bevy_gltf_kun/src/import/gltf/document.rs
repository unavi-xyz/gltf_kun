use bevy::{gltf::Gltf, utils::hashbrown::HashMap};
use gltf_kun::document::GltfDocument;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BevyImportError {}

pub fn import_gltf(_doc: GltfDocument) -> Result<Gltf, BevyImportError> {
    let gltf = Gltf {
        animations: Vec::new(),
        default_scene: None,
        materials: Vec::new(),
        meshes: Vec::new(),
        named_animations: HashMap::new(),
        named_materials: HashMap::new(),
        named_meshes: HashMap::new(),
        named_nodes: HashMap::new(),
        named_scenes: HashMap::new(),
        nodes: Vec::new(),
        scenes: Vec::new(),
    };

    Ok(gltf)
}
