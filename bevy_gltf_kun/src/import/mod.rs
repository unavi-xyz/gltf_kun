use anyhow::Result;
use bevy::{ecs::system::RunSystemOnce, gltf::Gltf, prelude::*, utils::HashMap};
use gltf_kun::document::GltfDocument;

pub mod scene;

pub struct GltfImportPlugin;

impl Plugin for GltfImportPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Import<GltfDocument>>()
            .add_event::<ImportResult<GltfDocument>>()
            .add_systems(Update, read_event.pipe(import_gltf));
    }
}

#[derive(Default, Event)]
pub struct Import<T> {
    pub doc: T,
}

#[derive(Event)]
pub struct ImportResult<T> {
    pub result: Result<T>,
}

pub struct ImportContext {
    pub doc: GltfDocument,
    pub gltf: Gltf,
}

impl ImportContext {
    pub fn new(event: Import<GltfDocument>) -> Self {
        Self {
            doc: event.doc,
            gltf: Gltf {
                animations: Vec::new(),
                default_scene: None,
                materials: Vec::new(),
                meshes: Vec::new(),
                named_animations: HashMap::new(),
                named_materials: HashMap::new(),
                named_meshes: HashMap::new(),
                named_nodes: HashMap::new(),
                named_scenes: HashMap::new(),
                nodes: Vec::new(),
                scenes: Vec::new(),
            },
        }
    }
}

pub fn read_event(
    mut events: ResMut<Events<Import<GltfDocument>>>,
) -> Option<Import<GltfDocument>> {
    events.drain().next()
}

pub fn import_gltf(In(event): In<Option<Import<GltfDocument>>>, world: &mut World) {
    let event = match event {
        Some(event) => event,
        None => return,
    };

    let system = scene::import_scenes.pipe(create_import_result);
    world.run_system_once_with(ImportContext::new(event), system);
}

pub fn create_import_result(
    In(context): In<ImportContext>,
    mut writer: EventWriter<ImportResult<Gltf>>,
) {
    writer.send(ImportResult {
        result: Ok(context.gltf),
    });
}
