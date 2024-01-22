use std::marker::PhantomData;

use bevy::{ecs::system::RunSystemOnce, prelude::*};
use gltf_kun::graph::{
    gltf::{self, document::GltfDocument},
    Graph,
};
use thiserror::Error;

pub mod mesh;
pub mod node;
pub mod scene;
pub mod vertex_to_accessor;

pub struct GltfExportPlugin;

impl Plugin for GltfExportPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GltfExport>()
            .add_event::<GltfExportResult>()
            .add_systems(Update, read_event.pipe(export_gltf));
    }
}

pub type GltfExport = Export<GltfDocument>;
pub type GltfExportResult = ExportResult<GltfDocument>;

#[derive(Default, Event)]
pub struct Export<T> {
    pub scenes: Vec<Handle<Scene>>,
    pub default_scene: Option<Handle<Scene>>,
    pub _doc_type: PhantomData<T>,
}

impl<T> Export<T> {
    pub fn new(scene: Handle<Scene>) -> Self {
        Self {
            scenes: vec![scene.clone()],
            default_scene: Some(scene),
            _doc_type: PhantomData,
        }
    }
}

#[derive(Debug, Error)]
pub enum ExportError {}

#[derive(Event)]
pub struct ExportResult<T> {
    pub graph: Graph,
    pub result: Result<T, ExportError>,
}

pub struct ExportContext {
    pub doc: GltfDocument,
    pub event: GltfExport,
    pub graph: Graph,
    pub meshes: Vec<CachedMesh>,
    pub nodes: Vec<CachedNode>,
    pub scenes: Vec<CachedScene>,
}

impl ExportContext {
    pub fn new(event: GltfExport) -> Self {
        let mut graph = Graph::default();
        let doc = GltfDocument::new(&mut graph);

        Self {
            doc,
            event,
            graph,
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
    pub handle: Handle<Scene>,
    pub entity: Entity,
}

pub fn read_event(mut events: ResMut<Events<GltfExport>>) -> Option<GltfExport> {
    events.drain().next()
}

pub fn export_gltf(In(event): In<Option<GltfExport>>, world: &mut World) {
    let event = match event {
        Some(event) => event,
        None => return,
    };

    world.run_system_once_with(
        ExportContext::new(event),
        scene::export_scenes
            .pipe(node::export_nodes)
            .pipe(mesh::export_meshes)
            .pipe(create_export_result),
    );
}

pub fn create_export_result(
    In(context): In<ExportContext>,
    mut writer: EventWriter<GltfExportResult>,
) {
    writer.send(ExportResult {
        graph: context.graph,
        result: Ok(context.doc),
    });
}
