use bevy::asset::{LoadContext, ReadAssetBytesError};
use gltf_kun::io::resolver::Resolver;
use thiserror::Error;

pub struct BevyAssetResolver<'a, 'b> {
    pub load_context: &'a mut LoadContext<'b>,
}

#[derive(Debug, Error)]
pub enum BevyAssetResolverError {
    #[error("failed to read asset: {0}")]
    ReadAssetBytesError(#[from] ReadAssetBytesError),
}

impl Resolver for BevyAssetResolver<'_, '_> {
    type Error = BevyAssetResolverError;

    async fn resolve(&mut self, uri: &str) -> Result<Vec<u8>, Self::Error> {
        let buffer_path = self.load_context.path().parent().unwrap().join(uri);
        let bytes = self.load_context.read_asset_bytes(buffer_path).await?;
        Ok(bytes)
    }
}
