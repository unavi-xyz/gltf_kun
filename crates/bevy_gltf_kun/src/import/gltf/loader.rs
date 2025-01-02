use std::marker::PhantomData;

use bevy::{
    asset::{io::Reader, AssetLoadError, AssetLoader, LoadContext, ReadAssetBytesError},
    prelude::*,
    utils::HashMap,
};
use gltf_kun::{
    extensions::ExtensionImport,
    graph::{gltf::GltfDocument, Graph},
    io::format::{
        glb::{GlbImport, GlbImportError},
        gltf::{import::GltfImportError, GltfFormat, GltfImport},
    },
};
use thiserror::Error;

use crate::import::{extensions::BevyExtensionImport, resolver::BevyAssetResolver};

use super::{
    document::{import_gltf_document, DocumentImportError, ImportContext},
    GltfKun,
};

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
    #[error("Failed to load asset from an asset path: {0}")]
    AssetLoadError(#[from] AssetLoadError),
    #[error("Failed to import into bevy: {0}")]
    Bevy(#[from] DocumentImportError),
    #[error("Failed to import gltf: {0}")]
    GltfImport(#[from] GltfImportError),
    #[error("Failed to import glb: {0}")]
    GlbImport(#[from] GlbImportError),
    #[error("Failed to load file: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to read bytes from an asset path: {0}")]
    ReadAssetBytesError(#[from] ReadAssetBytesError),
    #[error("Failed to parse gltf: {0}")]
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
    type Settings = ();
    type Error = GltfError;

    fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        load_context: &mut LoadContext,
    ) -> impl bevy::utils::ConditionalSendFuture<Output = Result<Self::Asset, Self::Error>> {
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
    type Settings = ();
    type Error = GltfError;

    fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        load_context: &mut LoadContext,
    ) -> impl bevy::utils::ConditionalSendFuture<Output = Result<Self::Asset, Self::Error>> {
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
