use bevy::prelude::*;

mod exporter;

pub struct GltfExportPlugin;

impl Plugin for GltfExportPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ExportScene>()
            .add_systems(PreUpdate, exporter::export_gltf);
    }
}

#[derive(Default)]
pub enum ExportFormat {
    #[default]
    Standard,
    Binary,
}



#[derive(Event)]
pub struct ExportScene {
    pub scenes: Vec<Entity>,
    pub format: ExportFormat,
}
