use std::collections::HashMap;

use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext},
    prelude::*,
    utils::BoxedFuture,
};
use gltf_kun::{
    document::GltfDocument,
    io::{
        format::{glb::GlbFormat, gltf::GltfFormat, ImportFormat},
        resolver::file_resolver::FileResolver,
    },
};

#[derive(Asset, TypePath)]
pub struct GltfDocumentAsset(pub GltfDocument);

#[derive(Default)]
pub struct GltfLoader;

impl AssetLoader for GltfLoader {
    type Asset = GltfDocumentAsset;
    type Settings = ();
    type Error = anyhow::Error;
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
                resources: HashMap::new(),
                resolver: Some(Box::new(FileResolver::new(
                    load_context.path().parent().unwrap(),
                ))),
            };

            Ok(GltfDocumentAsset(format.import()?))
        })
    }

    fn extensions(&self) -> &[&str] {
        &["gltf"]
    }
}

#[derive(Default)]
pub struct GlbLoader;

impl AssetLoader for GlbLoader {
    type Asset = GltfDocumentAsset;
    type Settings = ();
    type Error = anyhow::Error;
    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a (),
        _load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            Ok(GltfDocumentAsset(GlbFormat::import_slice(&bytes)?))
        })
    }

    fn extensions(&self) -> &[&str] {
        &["glb"]
    }
}
