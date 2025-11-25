use bevy::{platform::collections::HashMap, prelude::*};
use gltf_kun::graph::{
    Graph,
    gltf::{Node, document::GltfDocument},
};

use self::{animation::RawGltfAnimation, mesh::GltfMesh, node::GltfNode, scene::GltfScene};

pub mod animation;
pub mod document;
pub mod loader;
pub mod material;
pub mod mesh;
pub mod node;
pub mod primitive;
pub mod scene;
pub mod skin;
pub mod texture;

#[derive(Asset, Debug, Default, TypePath)]
pub struct GltfKun {
    pub graph: Graph,
    pub node_handles: HashMap<Node, Handle<GltfNode>>,

    pub animations: Vec<Handle<AnimationClip>>,
    pub raw_animations: Vec<Handle<RawGltfAnimation>>,
    pub default_scene: Option<Handle<GltfScene>>,
    pub images: Vec<Handle<Image>>,
    pub materials: Vec<Handle<StandardMaterial>>,
    pub meshes: Vec<Handle<GltfMesh>>,
    pub nodes: Vec<Handle<GltfNode>>,
    pub scenes: Vec<Handle<GltfScene>>,

    pub named_animations: HashMap<String, Handle<AnimationClip>>,
    pub named_raw_animations: HashMap<String, Handle<RawGltfAnimation>>,
    pub named_materials: HashMap<String, Handle<StandardMaterial>>,
    pub named_meshes: HashMap<String, Handle<GltfMesh>>,
    pub named_nodes: HashMap<String, Handle<GltfNode>>,
    pub named_scenes: HashMap<String, Handle<GltfScene>>,
}

impl GltfKun {
    pub fn new(graph: &mut Graph, doc: &mut GltfDocument) -> Self {
        Self {
            images: vec![Handle::default(); doc.images(graph).len()],
            materials: vec![Handle::default(); doc.materials(graph).len()],
            meshes: vec![Handle::default(); doc.meshes(graph).len()],
            nodes: vec![Handle::default(); doc.nodes(graph).len()],
            scenes: vec![Handle::default(); doc.scenes(graph).len()],
            ..default()
        }
    }
}
