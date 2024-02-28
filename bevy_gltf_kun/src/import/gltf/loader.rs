use std::marker::PhantomData;

use bevy::{
    asset::{
        io::Reader, AssetLoadError, AssetLoader, AsyncReadExt, LoadContext, ReadAssetBytesError,
    },
    prelude::*,
    utils::{BoxedFuture, HashMap},
};
use gltf_kun::{
    extensions::ExtensionsIO,
    graph::{gltf::GltfDocument, Graph},
    io::format::{
        glb::{GlbIO, GlbImportError},
        gltf::{import::GltfImportError, GltfFormat, GltfIO},
    },
};
use thiserror::Error;

use crate::import::{extensions::BevyImportExtensions, resolver::BevyAssetResolver};

use super::{
    document::{import_gltf_document, DocumentImportError, ImportContext},
    GltfKun,
};

pub struct GltfLoader<E: BevyImportExtensions<GltfDocument>> {
    pub _marker: PhantomData<E>,
}

impl<E: BevyImportExtensions<GltfDocument>> Default for GltfLoader<E> {
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
    Import(#[from] GltfImportError),
    #[error("Failed to load file: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to read bytes from an asset path: {0}")]
    ReadAssetBytesError(#[from] ReadAssetBytesError),
    #[error("Failed to parse gltf: {0}")]
    SerdeJson(#[from] serde_json::Error),
}

impl<E> AssetLoader for GltfLoader<E>
where
    E: ExtensionsIO<GltfDocument, GltfFormat>
        + BevyImportExtensions<GltfDocument>
        + Send
        + Sync
        + 'static,
{
    type Asset = GltfKun;
    type Settings = ();
    type Error = GltfError;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a (),
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;

            let mut graph = Graph::default();
            let format = GltfFormat {
                json: serde_json::from_slice(&bytes)?,
                resources: std::collections::HashMap::new(),
            };
            let resolver = BevyAssetResolver { load_context };

            let mut doc = GltfIO::<E>::import(&mut graph, format, Some(resolver)).await?;
            let mut gltf = GltfKun::new(&mut graph, &mut doc);

            let mut context = ImportContext {
                doc: &mut doc,
                gltf: &mut gltf,
                graph: &mut graph,
                load_context,
                node_entities: HashMap::default(),
                node_primitive_entities: HashMap::default(),
                nodes_handles: HashMap::default(),
                skin_matrices: HashMap::default(),
            };

            import_gltf_document::<E>(&mut context)?;

            Ok(gltf)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["gltf"]
    }
}

pub struct GlbLoader<E: BevyImportExtensions<GltfDocument>> {
    pub _marker: PhantomData<E>,
}

impl<E: BevyImportExtensions<GltfDocument>> Default for GlbLoader<E> {
    fn default() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

#[derive(Debug, Error)]
pub enum GlbError {
    #[error("Failed to import into bevy: {0}")]
    Bevy(#[from] DocumentImportError),
    #[error("Failed to import glb: {0}")]
    Import(#[from] GlbImportError),
    #[error("Failed to load file: {0}")]
    Io(#[from] std::io::Error),
}

impl<E> AssetLoader for GlbLoader<E>
where
    E: ExtensionsIO<GltfDocument, GltfFormat>
        + BevyImportExtensions<GltfDocument>
        + Send
        + Sync
        + 'static,
{
    type Asset = GltfKun;
    type Settings = ();
    type Error = GlbError;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a (),
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;

            let mut graph = Graph::default();
            let mut doc = GlbIO::<E>::import_slice(&mut graph, &bytes).await?;

            let mut gltf = GltfKun::new(&mut graph, &mut doc);

            let mut context = ImportContext {
                doc: &mut doc,
                gltf: &mut gltf,
                graph: &mut graph,
                load_context,
                node_entities: HashMap::default(),
                node_primitive_entities: HashMap::default(),
                nodes_handles: HashMap::default(),
                skin_matrices: HashMap::default(),
            };

            import_gltf_document::<E>(&mut context)?;

            Ok(gltf)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["glb"]
    }
}
