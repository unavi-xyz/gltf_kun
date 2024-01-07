use anyhow::Result;
use bevy::prelude::*;
use gltf_kun::document::gltf::GltfDocument;

pub mod format;

pub struct GltfKunPlugin;

impl Plugin for GltfKunPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Export>().add_event::<ExportResult>();
    }
}

#[derive(Event)]
pub struct Export {
    pub scenes: Vec<Entity>,
    pub default_scene: Option<Entity>,
}

#[derive(Event)]
pub struct ExportResult {
    pub result: Result<Box<GltfDocument>>,
}
