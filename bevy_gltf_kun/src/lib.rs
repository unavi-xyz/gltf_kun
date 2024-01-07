use anyhow::Result;
use bevy::prelude::*;
use gltf_kun::io::format::{glb::GlbFormat, gltf::GltfFormat, ExportFormat};

pub mod format;

pub struct GltfKunPlugin;

impl Plugin for GltfKunPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Export<GltfFormat>>()
            .add_event::<ExportResult<GltfFormat>>()
            .add_event::<Export<GlbFormat>>()
            .add_event::<ExportResult<GlbFormat>>();
    }
}

#[derive(Event)]
pub struct Export<T: ExportFormat> {
    pub scenes: Vec<Scene>,
    pub default_scene: Option<Scene>,
    pub format: T,
}

#[derive(Event)]
pub struct ExportResult<T: ExportFormat> {
    pub result: Result<T>,
}
