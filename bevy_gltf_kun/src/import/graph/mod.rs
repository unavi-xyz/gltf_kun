use bevy::prelude::*;
use gltf_kun::{extensions::DefaultExtensions, graph::Graph};

use self::loader::{GlbGraphLoader, GltfGraphLoader};

pub mod loader;

pub struct GraphImportPlugin;

impl Plugin for GraphImportPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<GltfGraph>()
            .register_asset_loader::<GlbGraphLoader<DefaultExtensions>>(GlbGraphLoader::<
                DefaultExtensions,
            >::default())
            .register_asset_loader::<GltfGraphLoader<DefaultExtensions>>(GltfGraphLoader::<
                DefaultExtensions,
            >::default());
    }
}

#[derive(Asset, Debug, Default, TypePath)]
pub struct GltfGraph(pub Graph);
