use bevy::prelude::*;

use gltf::{GlbLoader, Gltf, GltfLoader};

pub mod gltf;
pub mod resolver;

pub struct GltfImportPlugin;

impl Plugin for GltfImportPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<Gltf>()
            .register_asset_loader::<GltfLoader>(GltfLoader)
            .register_asset_loader::<GlbLoader>(GlbLoader);
    }
}
