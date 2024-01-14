use bevy::prelude::*;

use self::gltf::{
    loader::{GlbLoader, GltfLoader},
    node::GltfNode,
    Gltf,
};

pub mod gltf;
pub mod resolver;

pub struct GltfImportPlugin;

impl Plugin for GltfImportPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<Gltf>()
            .init_asset::<GltfNode>()
            .register_asset_loader::<GltfLoader>(GltfLoader)
            .register_asset_loader::<GlbLoader>(GlbLoader);
    }
}
