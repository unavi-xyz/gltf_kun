use bevy::{
    asset::{
        io::Reader, AssetLoadError, AssetLoader, AsyncReadExt, LoadContext, ReadAssetBytesError,
    },
    utils::BoxedFuture,
};
use gltf_kun::{
    graph::Graph,
    io::format::{
        glb::{GlbIO, GlbImportError},
        gltf::{import::GltfImportError, GltfFormat, GltfIO},
        DocumentIO,
    },
};
use thiserror::Error;

use crate::import::resolver::BevyAssetResolver;

use super::{
    document::{import_gltf_document, BevyImportError},
    Gltf,
};

#[derive(Default)]
pub struct GltfLoader;

#[derive(Debug, Error)]
pub enum GltfError {
    #[error("failed to load asset from an asset path: {0}")]
    AssetLoadError(#[from] AssetLoadError),
    #[error("failed to import into bevy: {0}")]
    Bevy(#[from] BevyImportError),
    #[error("failed to import gltf: {0}")]
    Import(#[from] GltfImportError),
    #[error("failed to load file: {0}")]
    Io(#[from] std::io::Error),
    #[error("failed to read bytes from an asset path: {0}")]
    ReadAssetBytesError(#[from] ReadAssetBytesError),
    #[error("failed to parse gltf: {0}")]
    SerdeJson(#[from] serde_json::Error),
}

impl AssetLoader for GltfLoader {
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
            let mut io = GltfIO::new(BevyAssetResolver { load_context });
            let mut graph = Graph::default();
            let doc = io.import(&mut graph, format).await?;
            let gltf = import_gltf_document(&mut graph, doc, load_context)?;
            Ok(gltf)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["gltf"]
    }
}

#[derive(Default)]
pub struct GlbLoader;

#[derive(Debug, Error)]
pub enum GlbError {
    #[error("failed to import into bevy: {0}")]
    Bevy(#[from] BevyImportError),
    #[error("failed to import glb: {0}")]
    Import(#[from] GlbImportError),
    #[error("failed to load file: {0}")]
    Io(#[from] std::io::Error),
}

impl AssetLoader for GlbLoader {
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
            let mut io = GlbIO::default();
            let mut graph = Graph::default();
            let doc = io.import_slice(&mut graph, &bytes).await?;
            let gltf = import_gltf_document(&mut graph, doc, load_context)?;
            Ok(gltf)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["glb"]
    }
}
