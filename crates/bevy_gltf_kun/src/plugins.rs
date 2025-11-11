use std::marker::PhantomData;

use bevy::prelude::*;
use gltf_kun::{
    extensions::ExtensionImport, graph::gltf::GltfDocument, io::format::gltf::GltfFormat,
};

use crate::{
    export::{
        extensions::BevyExtensionExport,
        gltf::{GltfExportEvent, GltfExportResult, export_gltf},
    },
    import::{
        extensions::BevyExtensionImport,
        gltf::{
            GltfKun,
            animation::RawGltfAnimation,
            loader::{GlbLoader, GltfLoader},
            mesh::GltfMesh,
            node::GltfNode,
            scene::GltfScene,
        },
    },
};

struct GltfAssetPlugin;

impl Plugin for GltfAssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<GltfKun>()
            .init_asset::<GltfMesh>()
            .init_asset::<GltfNode>()
            .init_asset::<GltfScene>()
            .init_asset::<RawGltfAnimation>();
    }
}

/// Adds the ability to export Bevy scenes to glTF.
pub struct GltfExportPlugin<E: BevyExtensionExport<GltfDocument>> {
    _marker: PhantomData<E>,
}

impl<E: BevyExtensionExport<GltfDocument>> Default for GltfExportPlugin<E> {
    fn default() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<E: BevyExtensionExport<GltfDocument>> Plugin for GltfExportPlugin<E> {
    fn build(&self, app: &mut App) {
        app.add_message::<GltfExportEvent<E>>()
            .add_message::<GltfExportResult>()
            .add_systems(Update, export_gltf::<E>);
    }
}

/// Adds the ability to import glTF files.
pub struct GltfImportPlugin<E: BevyExtensionImport<GltfDocument> + Send + Sync> {
    _marker: PhantomData<E>,
}

impl<E: BevyExtensionImport<GltfDocument> + Send + Sync> Default for GltfImportPlugin<E> {
    fn default() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<E> Plugin for GltfImportPlugin<E>
where
    E: BevyExtensionImport<GltfDocument>
        + ExtensionImport<GltfDocument, GltfFormat>
        + Send
        + Sync
        + 'static,
{
    fn build(&self, app: &mut App) {
        app.add_plugins(GltfAssetPlugin)
            .register_asset_loader::<GltfLoader<E>>(GltfLoader::<E>::default())
            .register_asset_loader::<GlbLoader<E>>(GlbLoader::<E>::default());
    }
}
