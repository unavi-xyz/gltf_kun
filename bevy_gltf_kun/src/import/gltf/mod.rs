use bevy::{prelude::*, utils::HashMap};
use gltf_kun::{
    extensions::DefaultExtensions,
    graph::{gltf::document::GltfDocument, Graph},
};

use self::{
    loader::{GlbLoader, GltfLoader},
    mesh::GltfMesh,
    node::GltfNode,
};

pub mod document;
pub mod loader;
pub mod mesh;
pub mod node;
pub mod primitive;
pub mod scene;

pub struct GltfImportPlugin;

impl Plugin for GltfImportPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<Gltf>()
            .init_asset::<GltfNode>()
            .init_asset::<GltfMesh>()
            .register_asset_loader::<GltfLoader<DefaultExtensions>>(
                GltfLoader::<DefaultExtensions>::default(),
            )
            .register_asset_loader::<GlbLoader<DefaultExtensions>>(
                GlbLoader::<DefaultExtensions>::default(),
            );
    }
}

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
