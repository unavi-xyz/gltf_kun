use bevy::asset::LoadContext;
use gltf_kun::io::resolver::{Resolver, ResolverError};

pub struct BevyAssetResolver<'a, 'b> {
    pub load_context: &'a mut LoadContext<'b>,
}

impl Resolver for BevyAssetResolver<'_, '_> {
    fn resolve(
        &mut self,
        uri: &str,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<Vec<u8>, ResolverError>> + Send + '_>,
    > {
        let uri = uri.to_string();

        Box::pin(async move {
            let buffer_path = self
                .load_context
                .path()
                .parent()
                .ok_or(ResolverError::InvalidUri(uri.clone()))?
                .join(uri);

            self.load_context
                .read_asset_bytes(buffer_path)
                .await
                .map_err(|e| ResolverError::ResolutionError(e.to_string()))
        })
    }
}
