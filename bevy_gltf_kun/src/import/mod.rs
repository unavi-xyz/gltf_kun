use bevy::prelude::*;
use gltf_kun::extensions::DefaultExtensions;

use self::gltf::{
    loader::{GlbLoader, GltfLoader},
    mesh::GltfMesh,
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
            .init_asset::<GltfMesh>()
            .register_asset_loader::<GltfLoader<DefaultExtensions>>(
                GltfLoader::<DefaultExtensions>::default(),
            )
            .register_asset_loader::<GlbLoader<DefaultExtensions>>(
                GlbLoader::<DefaultExtensions>::default(),
            );
    }
}
