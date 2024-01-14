use bevy::{prelude::*, utils::HashMap};

use self::{node::GltfNode, scene::GltfScene};

pub mod document;
pub mod loader;
pub mod mesh;
pub mod node;
pub mod scene;

#[derive(Asset, Debug, Default, TypePath)]
pub struct Gltf {
    pub named_nodes: HashMap<String, Handle<GltfNode>>,
    pub named_scenes: HashMap<String, Handle<GltfScene>>,
    pub nodes: Vec<Handle<GltfNode>>,
    pub scenes: Vec<Handle<GltfScene>>,
}
