use std::marker::PhantomData;

use bevy::{
    asset::{AssetLoadError, AssetLoader, LoadContext, ReadAssetBytesError, io::Reader},
    platform::collections::HashMap,
    prelude::*,
};
use gltf_kun::{
    extensions::ExtensionImport,
    graph::{Graph, gltf::GltfDocument},
    io::format::{
        glb::{GlbImport, GlbImportError},
        gltf::{GltfFormat, GltfImport, import::GltfImportError},
    },
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::import::{extensions::BevyExtensionImport, resolver::BevyAssetResolver};

use super::{
    GltfKun,
    document::{DocumentImportError, ImportContext, import_gltf_document},
};

/// Settings for loading GLTF/GLB files.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GltfLoaderSettings {
    /// Whether to expose raw animation curves for retargeting and custom processing.
    /// When true, creates RawGltfAnimation assets alongside the normal AnimationClip assets.
    pub expose_raw_animation_curves: bool,
}

pub struct GltfLoader<E: BevyExtensionImport<GltfDocument>> {
    pub _marker: PhantomData<E>,
}

pub struct GlbLoader<E: BevyExtensionImport<GltfDocument>> {
    pub _marker: PhantomData<E>,
}

impl<E: BevyExtensionImport<GltfDocument>> Default for GltfLoader<E> {
    fn default() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<E: BevyExtensionImport<GltfDocument>> Default for GlbLoader<E> {
    fn default() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

#[derive(Debug, Error)]
pub enum GltfError {
    #[error("failed to load asset from an asset path: {0}")]
    AssetLoadError(#[from] AssetLoadError),
    #[error("failed to import into bevy: {0}")]
    Bevy(#[from] DocumentImportError),
    #[error("failed to import gltf: {0}")]
    GltfImport(#[from] GltfImportError),
    #[error("failed to import glb: {0}")]
    GlbImport(#[from] GlbImportError),
    #[error("failed to load file: {0}")]
    Io(#[from] std::io::Error),
    #[error("failed to read bytes from an asset path: {0}")]
    ReadAssetBytesError(#[from] ReadAssetBytesError),
    #[error("failed to parse gltf: {0}")]
    SerdeJson(#[from] serde_json::Error),
}

impl<E> AssetLoader for GltfLoader<E>
where
    E: ExtensionImport<GltfDocument, GltfFormat>
        + BevyExtensionImport<GltfDocument>
        + Send
        + Sync
        + 'static,
{
    type Asset = GltfKun;
    type Settings = GltfLoaderSettings;
    type Error = GltfError;

    fn load(
        &self,
        reader: &mut dyn Reader,
        settings: &Self::Settings,
        load_context: &mut LoadContext,
    ) -> impl bevy::tasks::ConditionalSendFuture<Output = std::result::Result<Self::Asset, Self::Error>>
    {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;

            let mut graph = Graph::default();

            let format = GltfFormat {
                json: serde_json::from_slice(&bytes)?,
                resources: std::collections::HashMap::new(),
            };
            let resolver = BevyAssetResolver { load_context };

            let mut doc = GltfImport::<E>::import(&mut graph, format, Some(resolver)).await?;

            let mut gltf = GltfKun::new(&mut graph, &mut doc);

            let mut context = ImportContext {
                doc: &mut doc,
                gltf: &mut gltf,
                graph: &mut graph,
                load_context,
                expose_raw_curves: settings.expose_raw_animation_curves,

                materials: HashMap::default(),
                skin_matrices: HashMap::default(),
            };

            import_gltf_document::<E>(&mut context)?;

            gltf.graph = graph;

            Ok(gltf)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["gltf"]
    }
}

impl<E> AssetLoader for GlbLoader<E>
where
    E: ExtensionImport<GltfDocument, GltfFormat>
        + BevyExtensionImport<GltfDocument>
        + Send
        + Sync
        + 'static,
{
    type Asset = GltfKun;
    type Settings = GltfLoaderSettings;
    type Error = GltfError;

    fn load(
        &self,
        reader: &mut dyn Reader,
        settings: &Self::Settings,
        load_context: &mut LoadContext,
    ) -> impl bevy::tasks::ConditionalSendFuture<Output = std::result::Result<Self::Asset, Self::Error>>
    {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;

            let mut graph = Graph::default();

            let mut doc = GlbImport::<E>::import_slice(&mut graph, &bytes).await?;

            let mut gltf = GltfKun::new(&mut graph, &mut doc);

            let mut context = ImportContext {
                doc: &mut doc,
                gltf: &mut gltf,
                graph: &mut graph,
                load_context,
                expose_raw_curves: settings.expose_raw_animation_curves,

                materials: HashMap::default(),
                skin_matrices: HashMap::default(),
            };

            import_gltf_document::<E>(&mut context)?;

            gltf.graph = graph;

            Ok(gltf)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["glb"]
    }
}
