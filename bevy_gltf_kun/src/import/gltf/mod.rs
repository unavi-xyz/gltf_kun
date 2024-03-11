use std::marker::PhantomData;

use bevy::{prelude::*, utils::HashMap};
use gltf_kun::{
    extensions::ExtensionImport,
    graph::{gltf::document::GltfDocument, Graph},
    io::format::gltf::GltfFormat,
};

use self::{
    loader::{GlbLoader, GltfLoader},
    mesh::GltfMesh,
    node::GltfNode,
};

use super::extensions::BevyImportExtensions;

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

/// Adds the ability to import glTF files.
pub struct GltfImportPlugin<E: BevyImportExtensions<GltfDocument> + Send + Sync> {
    _marker: PhantomData<E>,
}

impl<E: BevyImportExtensions<GltfDocument> + Send + Sync> Default for GltfImportPlugin<E> {
    fn default() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<E> Plugin for GltfImportPlugin<E>
where
    E: BevyImportExtensions<GltfDocument>
        + ExtensionImport<GltfDocument, GltfFormat>
        + Send
        + Sync
        + 'static,
{
    fn build(&self, app: &mut App) {
        app.init_asset::<GltfKun>()
            .init_asset::<GltfNode>()
            .init_asset::<GltfMesh>()
            .register_asset_loader::<GltfLoader<E>>(GltfLoader::<E>::default())
            .register_asset_loader::<GlbLoader<E>>(GlbLoader::<E>::default());
    }
}

#[derive(Asset, Debug, Default, TypePath)]
pub struct GltfKun {
    pub animations: Vec<Handle<AnimationClip>>,
    pub default_scene: Option<Handle<Scene>>,
    pub images: Vec<Handle<Image>>,
    pub materials: Vec<Handle<StandardMaterial>>,
    pub meshes: Vec<Handle<GltfMesh>>,
    pub nodes: Vec<Handle<GltfNode>>,
    pub scenes: Vec<Handle<Scene>>,

    pub named_animations: HashMap<String, Handle<AnimationClip>>,
    pub named_materials: HashMap<String, Handle<StandardMaterial>>,
    pub named_meshes: HashMap<String, Handle<GltfMesh>>,
    pub named_nodes: HashMap<String, Handle<GltfNode>>,
    pub named_scenes: HashMap<String, Handle<Scene>>,
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
