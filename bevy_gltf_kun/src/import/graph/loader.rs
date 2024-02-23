use std::marker::PhantomData;

use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext},
    utils::BoxedFuture,
};
use gltf_kun::{
    extensions::ExtensionsIO,
    graph::{gltf::GltfDocument, Graph},
    io::format::{
        glb::GlbIO,
        gltf::{GltfFormat, GltfIO},
    },
};

use crate::import::{
    extensions::BevyImportExtensions,
    gltf::loader::{GlbError, GltfError},
    resolver::BevyAssetResolver,
};

use super::GltfGraph;

pub struct GltfGraphLoader<E: BevyImportExtensions<GltfDocument>> {
    pub _marker: PhantomData<E>,
}

impl<E: BevyImportExtensions<GltfDocument>> Default for GltfGraphLoader<E> {
    fn default() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<E> AssetLoader for GltfGraphLoader<E>
where
    E: ExtensionsIO<GltfDocument, GltfFormat>
        + BevyImportExtensions<GltfDocument>
        + Send
        + Sync
        + 'static,
{
    type Asset = GltfGraph;
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
            GltfIO::<E>::import(&mut graph, format, Some(resolver)).await?;

            let graph = GltfGraph(graph);

            Ok(graph)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["gltf", "glb"]
    }
}

pub struct GlbGraphLoader<E: BevyImportExtensions<GltfDocument>> {
    pub _marker: PhantomData<E>,
}

impl<E: BevyImportExtensions<GltfDocument>> Default for GlbGraphLoader<E> {
    fn default() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<E> AssetLoader for GlbGraphLoader<E>
where
    E: ExtensionsIO<GltfDocument, GltfFormat>
        + BevyImportExtensions<GltfDocument>
        + Send
        + Sync
        + 'static,
{
    type Asset = GltfGraph;
    type Settings = ();
    type Error = GlbError;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a (),
        _load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;

            let mut graph = Graph::default();
            GlbIO::<E>::import_slice(&mut graph, &bytes).await?;

            let graph = GltfGraph(graph);

            Ok(graph)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["glb"]
    }
}
