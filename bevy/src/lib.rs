use bevy::prelude::*;

mod exporter;

pub struct GltfExportPlugin;

impl Plugin for GltfExportPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ExportScene>()
            .add_systems(PreUpdate, exporter::export_gltf);
    }
}

pub enum ExportFormat {
    Standard,
    Binary,
}

impl Default for ExportFormat {
    fn default() -> Self {
        ExportFormat::Standard
    }
}

#[derive(Event)]
pub struct ExportScene {
    pub scenes: Vec<Entity>,
    pub format: ExportFormat,
}
