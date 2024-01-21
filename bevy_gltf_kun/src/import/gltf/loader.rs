use bevy::{
    asset::{
        io::Reader, AssetLoadError, AssetLoader, AsyncReadExt, LoadContext, ReadAssetBytesError,
    },
    utils::BoxedFuture,
};
use gltf_kun::{
    extensions::ExtensionsIO,
    graph::{gltf::document::GltfDocument, Graph},
    io::format::{
        glb::{GlbIO, GlbImportError},
        gltf::{import::GltfImportError, GltfFormat, GltfIO},
    },
};
use thiserror::Error;

use crate::{extensions::BevyExtensionIO, import::resolver::BevyAssetResolver};

use super::{
    document::{import_gltf_document, DocumentImportError},
    Gltf,
};

pub struct GltfLoader<E: BevyExtensionIO<GltfDocument>> {
    pub _marker: std::marker::PhantomData<E>,
}

impl<E: BevyExtensionIO<GltfDocument>> Default for GltfLoader<E> {
    fn default() -> Self {
        Self {
            _marker: std::marker::PhantomData,
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
    Import(#[from] GltfImportError),
    #[error("failed to load file: {0}")]
    Io(#[from] std::io::Error),
    #[error("failed to read bytes from an asset path: {0}")]
    ReadAssetBytesError(#[from] ReadAssetBytesError),
    #[error("failed to parse gltf: {0}")]
    SerdeJson(#[from] serde_json::Error),
}

impl<E> AssetLoader for GltfLoader<E>
where
    E: ExtensionsIO<GltfDocument, GltfFormat>
        + BevyExtensionIO<GltfDocument>
        + Send
        + Sync
        + 'static,
{
    type Asset = Gltf;
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

            let format = GltfFormat {
                json: serde_json::from_slice(&bytes)?,
                resources: std::collections::HashMap::new(),
            };
            let resolver = BevyAssetResolver { load_context };
            let mut graph = Graph::default();
            let mut doc = GltfIO::<E>::import(&mut graph, format, Some(resolver)).await?;

            let gltf = import_gltf_document(&mut graph, doc, load_context)?;
            E::import_bevy(&mut graph, &mut doc);

            Ok(gltf)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["gltf"]
    }
}

pub struct GlbLoader<E: BevyExtensionIO<GltfDocument>> {
    pub _marker: std::marker::PhantomData<E>,
}

impl<E: BevyExtensionIO<GltfDocument>> Default for GlbLoader<E> {
    fn default() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}

#[derive(Debug, Error)]
pub enum GlbError {
    #[error("failed to import into bevy: {0}")]
    Bevy(#[from] DocumentImportError),
    #[error("failed to import glb: {0}")]
    Import(#[from] GlbImportError),
    #[error("failed to load file: {0}")]
    Io(#[from] std::io::Error),
}

impl<E> AssetLoader for GlbLoader<E>
where
    E: ExtensionsIO<GltfDocument, GltfFormat>
        + BevyExtensionIO<GltfDocument>
        + Send
        + Sync
        + 'static,
{
    type Asset = Gltf;
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

            let gltf = import_gltf_document(&mut graph, doc, load_context)?;
            E::import_bevy(&mut graph, &mut doc);

            Ok(gltf)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["glb"]
    }
}
