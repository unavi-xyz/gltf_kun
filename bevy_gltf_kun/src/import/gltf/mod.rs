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
pub mod image;
pub mod loader;
pub mod material;
pub mod mesh;
pub mod node;
pub mod primitive;
pub mod scene;

pub struct GltfImportPlugin;

impl Plugin for GltfImportPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<GltfKun>()
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
pub struct GltfKun {
    pub default_scene: Option<Handle<Scene>>,
    pub images: Vec<Handle<Image>>,
    pub materials: Vec<Handle<StandardMaterial>>,
    pub meshes: Vec<Handle<GltfMesh>>,
    pub nodes: Vec<Handle<GltfNode>>,
    pub scenes: Vec<Handle<Scene>>,

    pub named_materials: HashMap<String, Handle<StandardMaterial>>,
    pub named_meshes: HashMap<String, Handle<GltfMesh>>,
    pub named_nodes: HashMap<String, Handle<GltfNode>>,
    pub named_scenes: HashMap<String, Handle<Scene>>,

    pub node_entities: HashMap<Handle<GltfNode>, Entity>,
}

impl GltfKun {
    pub fn new(graph: &mut Graph, doc: &mut GltfDocument) -> Self {
        GltfKun {
            images: vec![Handle::default(); doc.images(graph).len()],
            materials: vec![Handle::default(); doc.materials(graph).len()],
            meshes: vec![Handle::default(); doc.meshes(graph).len()],
            nodes: vec![Handle::default(); doc.nodes(graph).len()],
            scenes: vec![Handle::default(); doc.scenes(graph).len()],
            ..default()
        }
    }
}
