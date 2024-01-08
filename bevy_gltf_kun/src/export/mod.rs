use std::marker::PhantomData;

use anyhow::Result;
use bevy::{ecs::system::RunSystemOnce, prelude::*};
use gltf_kun::{document::GltfDocument, graph::gltf};

pub mod mesh;
pub mod node;
pub mod scene;
pub mod vertex_to_accessor;

pub struct GltfExportPlugin;

impl Plugin for GltfExportPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Export<GltfDocument>>()
            .add_event::<ExportResult<GltfDocument>>()
            .add_state::<ExportState>()
            .add_systems(Update, read_event.pipe(export_gltf));
    }
}

#[derive(Default, Event)]
pub struct Export<T> {
    pub scenes: Vec<Entity>,
    pub default_scene: Option<Entity>,
    pub doc_type: PhantomData<T>,
}

#[derive(Event)]
pub struct ExportResult<T> {
    pub result: Result<T>,
}

/// State used to control the export systems.
/// We don't want to run the systems every frame if there's nothing to export.
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, States)]
pub enum ExportState {
    Active,
    #[default]
    Inactive,
}

pub struct ExportContext {
    pub event: Export<GltfDocument>,
    pub doc: GltfDocument,
    pub meshes: Vec<CachedMesh>,
    pub nodes: Vec<CachedNode>,
}

impl ExportContext {
    pub fn new(event: Export<GltfDocument>) -> Self {
        Self {
            event,
            doc: GltfDocument::default(),
            meshes: Vec::new(),
            nodes: Vec::new(),
        }
    }
}

pub struct CachedMesh {
    pub mesh: gltf::mesh::Mesh,
    /// Corresponding Bevy mesh handles used to create this mesh.
    pub bevy_meshes: Vec<Handle<Mesh>>,
}

pub struct CachedNode {
    pub node: gltf::node::Node,
    pub entity: Entity,
}

pub fn read_event(
    mut events: ResMut<Events<Export<GltfDocument>>>,
) -> Option<Export<GltfDocument>> {
    events.drain().next()
}

pub fn export_gltf(In(event): In<Option<Export<GltfDocument>>>, world: &mut World) {
    let event = match event {
        Some(event) => event,
        None => return,
    };

    let system = scene::export_scenes.pipe(node::export_nodes.pipe(mesh::export_meshes));
    world.run_system_once_with(ExportContext::new(event), system);
}
