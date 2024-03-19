use bevy::asset::LoadContext;
use gltf_kun::io::resolver::{Resolver, ResolverError};

pub struct BevyAssetResolver<'a, 'b> {
    pub load_context: &'a mut LoadContext<'b>,
}

impl Resolver for BevyAssetResolver<'_, '_> {
    async fn resolve(&mut self, uri: &str) -> Result<Vec<u8>, ResolverError> {
        let buffer_path = self
            .load_context
            .path()
            .parent()
            .ok_or(ResolverError::InvalidUri(uri.to_string()))?
            .join(uri);

        self.load_context
            .read_asset_bytes(buffer_path)
            .await
            .map_err(|e| ResolverError::ResolutionError(e.to_string()))
    }
}
