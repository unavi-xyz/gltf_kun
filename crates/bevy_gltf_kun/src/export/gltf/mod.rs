use std::marker::PhantomData;

use bevy::{ecs::system::RunSystemOnce, prelude::*};
use gltf_kun::graph::{
    Graph,
    gltf::{self, document::GltfDocument},
};
use thiserror::Error;

use super::extensions::BevyExtensionExport;

pub mod animation;
pub mod material;
pub mod mesh;
pub mod node;
pub mod scene;
pub mod skin;

#[derive(Default, Message)]
pub struct GltfExportEvent<E: BevyExtensionExport<GltfDocument>> {
    pub scenes: Vec<Handle<Scene>>,
    pub default_scene: Option<Handle<Scene>>,
    _marker: PhantomData<E>,
}

impl<E: BevyExtensionExport<GltfDocument>> GltfExportEvent<E> {
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

#[derive(Message)]
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
    pub fn new<E: BevyExtensionExport<GltfDocument>>(event: GltfExportEvent<E>) -> Self {
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
    pub bevy_material: MeshMaterial3d<StandardMaterial>,
}

pub struct CachedMesh {
    pub mesh: gltf::Mesh,
    pub primitives: Vec<(Entity, gltf::Primitive)>,
    pub bevy_meshes: Vec<Mesh3d>,
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

pub fn export_gltf<E: BevyExtensionExport<GltfDocument>>(world: &mut World) {
    let events = match world.get_resource_mut::<Messages<GltfExportEvent<E>>>() {
        Some(mut events) => events.drain().collect::<Vec<_>>(),
        None => return,
    };

    for event in events {
        world
            .run_system_once_with(
                scene::export_scenes
                    .pipe(node::export_nodes)
                    .pipe(mesh::export_meshes)
                    .pipe(material::export_materials)
                    .pipe(animation::export_animations)
                    .pipe(skin::export_skins)
                    .pipe(E::bevy_export)
                    .pipe(create_export_result),
                ExportContext::new(event),
            )
            .expect("export");
    }
}

pub fn create_export_result(
    In(ctx): In<ExportContext>,
    mut writer: MessageWriter<GltfExportResult>,
) {
    writer.write(GltfExportResult {
        graph: ctx.graph,
        result: Ok(ctx.doc),
    });
}
