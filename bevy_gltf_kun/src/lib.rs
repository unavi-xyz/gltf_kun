use anyhow::Result;
use bevy::prelude::*;
use gltf_kun::document::gltf::GltfDocument;

pub mod export;

pub struct GltfKunPlugin;

impl Plugin for GltfKunPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Export>()
            .add_event::<ExportResult>()
            .add_systems(Update, export::export_gltf);
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

pub struct BevyFormat;
