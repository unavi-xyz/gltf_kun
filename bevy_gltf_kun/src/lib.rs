use bevy::prelude::*;

mod exporter;

pub struct GltfExportPlugin;

impl Plugin for GltfExportPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ExportScene>()
            .add_event::<ExportResult>()
            .add_systems(PreUpdate, exporter::export_gltf);
    }
}

#[derive(Default)]
pub enum ExportFormat {
    Standard,
    #[default]
    Binary,
}

#[derive(Event)]
pub struct ExportScene {
    pub scenes: Vec<Entity>,
    pub format: ExportFormat,
}

#[derive(Event)]
pub enum ExportResult {
    Standard {
        root: gltf::json::Root,
        binary: Vec<u8>,
    },
    Binary(Vec<u8>),
}
