use bevy::prelude::*;

pub mod export;

pub struct GltfKunPlugin;

impl Plugin for GltfKunPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(export::GltfExportPlugin);
    }
}
