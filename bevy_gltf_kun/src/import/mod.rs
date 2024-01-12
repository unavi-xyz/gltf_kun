use bevy::prelude::*;

pub mod gltf;

pub struct GltfImportPlugin;

impl Plugin for GltfImportPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<gltf::GltfDocumentAsset>()
            .register_asset_loader::<gltf::GltfLoader>(gltf::GltfLoader::default());
    }
}
