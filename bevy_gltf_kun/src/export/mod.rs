use std::marker::PhantomData;

use bevy::{ecs::system::RunSystemOnce, prelude::*};
use gltf_kun::{document::GltfDocument, graph::gltf};
use thiserror::Error;

pub mod mesh;
pub mod node;
pub mod scene;
pub mod vertex_to_accessor;

pub struct GltfExportPlugin;

impl Plugin for GltfExportPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Export<GltfDocument>>()
            .add_event::<ExportResult<GltfDocument>>()
            .add_systems(Update, read_event.pipe(export_gltf));
    }
}

#[derive(Default, Event)]
pub struct Export<T> {
    pub scenes: Vec<Entity>,
    pub default_scene: Option<Entity>,
    pub _doc_type: PhantomData<T>,
}

#[derive(Debug, Error)]
pub enum ExportError {}

#[derive(Event)]
pub struct ExportResult<T> {
    pub result: Result<T, ExportError>,
}

pub struct ExportContext {
    pub doc: GltfDocument,
    pub event: Export<GltfDocument>,
    pub meshes: Vec<CachedMesh>,
    pub nodes: Vec<CachedNode>,
    pub scenes: Vec<CachedScene>,
}

impl ExportContext {
    pub fn new(event: Export<GltfDocument>) -> Self {
        Self {
            doc: GltfDocument::default(),
            event,
            meshes: Vec::new(),
            nodes: Vec::new(),
            scenes: Vec::new(),
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

pub struct CachedScene {
    pub scene: gltf::scene::Scene,
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

    let system = scene::export_scenes
        .pipe(node::export_nodes.pipe(mesh::export_meshes.pipe(create_export_result)));
    world.run_system_once_with(ExportContext::new(event), system);
}

pub fn create_export_result(
    In(context): In<ExportContext>,
    mut writer: EventWriter<ExportResult<GltfDocument>>,
) {
    writer.send(ExportResult {
        result: Ok(context.doc),
    });
}
