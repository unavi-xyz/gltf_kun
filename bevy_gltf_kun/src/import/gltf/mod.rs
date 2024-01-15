use bevy::{prelude::*, utils::HashMap};

use self::{mesh::GltfMesh, node::GltfNode};

pub mod document;
pub mod loader;
pub mod mesh;
pub mod node;
pub mod primitive;
pub mod scene;

#[derive(Asset, Debug, Default, TypePath)]
pub struct Gltf {
    pub meshes: Vec<Handle<GltfMesh>>,
    pub nodes: Vec<Handle<GltfNode>>,
    pub scenes: Vec<Handle<Scene>>,
    pub named_meshes: HashMap<String, Handle<GltfMesh>>,
    pub named_nodes: HashMap<String, Handle<GltfNode>>,
    pub named_scenes: HashMap<String, Handle<Scene>>,
}
