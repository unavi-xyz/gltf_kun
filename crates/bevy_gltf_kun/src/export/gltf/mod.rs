use std::marker::PhantomData;

use bevy::{ecs::system::RunSystemOnce, prelude::*};
use gltf_kun::graph::{
    gltf::{self, document::GltfDocument},
    Graph,
};
use thiserror::Error;

use super::extensions::BevyExportExtensions;

pub mod animation;
pub mod material;
pub mod mesh;
pub mod node;
pub mod scene;
pub mod skin;

/// Adds the ability to export Bevy scenes to glTF.
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
            .add_systems(Update, export_gltf::<E>);
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

    pub materials: Vec<CachedMaterial>,
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

            materials: Vec::new(),
            meshes: Vec::new(),
            nodes: Vec::new(),
            scenes: Vec::new(),
        }
    }
}

pub struct CachedMaterial {
    pub material: gltf::Material,
    pub entity: Entity,
    pub bevy_material: Handle<StandardMaterial>,
}

pub struct CachedMesh {
    pub mesh: gltf::Mesh,
    pub primitives: Vec<(Entity, gltf::Primitive)>,
    pub bevy_meshes: Vec<Handle<Mesh>>,
}

pub struct CachedNode {
    pub node: gltf::Node,
    pub entity: Entity,
}

pub struct CachedScene {
    pub scene: gltf::scene::Scene,
    pub handle: Handle<Scene>,
    pub entity: Entity,
}

pub fn export_gltf<E: BevyExportExtensions<GltfDocument>>(world: &mut World) {
    let events = match world.get_resource_mut::<Events<GltfExport<E>>>() {
        Some(mut events) => events.drain().collect::<Vec<_>>(),
        None => return,
    };

    for event in events {
        world.run_system_once_with(
            ExportContext::new(event),
            scene::export_scenes
                .pipe(node::export_nodes)
                .pipe(mesh::export_meshes)
                .pipe(material::export_materials)
                .pipe(animation::export_animations)
                .pipe(skin::export_skins)
                .pipe(E::bevy_export)
                .pipe(create_export_result),
        );
    }
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
