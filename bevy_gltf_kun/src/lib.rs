use anyhow::Result;
use bevy::prelude::*;
use gltf_kun::document::GltfDocument;

pub mod export;

pub struct GltfKunPlugin;

impl Plugin for GltfKunPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Export<GltfDocument>>()
            .add_event::<ExportResult<GltfDocument>>()
            .add_systems(Update, export::export_gltf);
    }
}

#[derive(Default, Event)]
pub struct Export<T> {
    pub scenes: Vec<Entity>,
    pub default_scene: Option<Entity>,
    pub document: T,
}

#[derive(Event)]
pub struct ExportResult<T> {
    pub result: Result<T>,
}
