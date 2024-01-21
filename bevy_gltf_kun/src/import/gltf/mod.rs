use bevy::{prelude::*, utils::HashMap};
use gltf_kun::graph::{gltf::document::GltfDocument, Graph};

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

    pub default_scene: Option<Handle<Scene>>,

    pub named_meshes: HashMap<String, Handle<GltfMesh>>,
    pub named_nodes: HashMap<String, Handle<GltfNode>>,
    pub named_scenes: HashMap<String, Handle<Scene>>,

    pub node_entities: HashMap<Handle<GltfNode>, Entity>,
}

impl Gltf {
    pub fn new(graph: &mut Graph, doc: &mut GltfDocument) -> Self {
        Gltf {
            nodes: vec![Handle::default(); doc.nodes(graph).len()],
            scenes: vec![Handle::default(); doc.scenes(graph).len()],
            meshes: vec![Handle::default(); doc.meshes(graph).len()],
            ..default()
        }
    }
}
