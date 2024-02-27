use std::marker::PhantomData;

use bevy::{ecs::system::RunSystemOnce, prelude::*};
use gltf_kun::graph::{
    gltf::{self, document::GltfDocument},
    Graph,
};
use thiserror::Error;

use super::extensions::BevyExportExtensions;

pub mod material;
pub mod mesh;
pub mod node;
pub mod scene;

pub struct GltfExportPlugin<E: BevyExportExtensions<GltfDocument>> {
    _marker: PhantomData<E>,
}

impl<E: BevyExportExtensions<GltfDocument>> Default for GltfExportPlugin<E> {
    fn default() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<E: BevyExportExtensions<GltfDocument>> Plugin for GltfExportPlugin<E> {
    fn build(&self, app: &mut App) {
        app.add_event::<GltfExport<E>>()
            .add_event::<GltfExportResult>()
            .add_systems(Update, read_event::<E>.pipe(export_gltf));
    }
}

#[derive(Default, Event)]
pub struct GltfExport<E: BevyExportExtensions<GltfDocument>> {
    pub scenes: Vec<Handle<Scene>>,
    pub default_scene: Option<Handle<Scene>>,
    _marker: PhantomData<E>,
}

impl<E: BevyExportExtensions<GltfDocument>> GltfExport<E> {
    pub fn new(scene: Handle<Scene>) -> Self {
        Self {
            scenes: vec![scene.clone()],
            default_scene: Some(scene),
            _marker: PhantomData,
        }
    }
}

#[derive(Debug, Error)]
pub enum ExportError {}

#[derive(Event)]
pub struct GltfExportResult {
    pub graph: Graph,
    pub result: Result<GltfDocument, ExportError>,
}

pub struct ExportContext {
    pub doc: GltfDocument,
    pub graph: Graph,

    pub target_scenes: Vec<Handle<Scene>>,
    pub target_default_scene: Option<Handle<Scene>>,

    pub meshes: Vec<CachedMesh>,
    pub nodes: Vec<CachedNode>,
    pub scenes: Vec<CachedScene>,
}

impl ExportContext {
    pub fn new<E: BevyExportExtensions<GltfDocument>>(event: GltfExport<E>) -> Self {
        let mut graph = Graph::default();
        let doc = GltfDocument::new(&mut graph);

        Self {
            doc,
            graph,

            target_scenes: event.scenes,
            target_default_scene: event.default_scene,

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

pub fn read_event<E: BevyExportExtensions<GltfDocument>>(
    mut events: ResMut<Events<GltfExport<E>>>,
) -> Option<GltfExport<E>> {
    events.drain().next()
}

pub fn export_gltf<E: BevyExportExtensions<GltfDocument>>(
    In(event): In<Option<GltfExport<E>>>,
    world: &mut World,
) {
    let event = match event {
        Some(event) => event,
        None => return,
    };

    world.run_system_once_with(
        ExportContext::new(event),
        scene::export_scenes
            .pipe(node::export_nodes)
            .pipe(mesh::export_meshes)
            .pipe(E::bevy_export)
            .pipe(create_export_result),
    );
}

pub fn create_export_result(
    In(context): In<ExportContext>,
    mut writer: EventWriter<GltfExportResult>,
) {
    writer.send(GltfExportResult {
        graph: context.graph,
        result: Ok(context.doc),
    });
}
