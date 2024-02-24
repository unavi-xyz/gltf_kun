use base64::engine::{general_purpose::STANDARD, Engine};

use super::{Resolver, ResolverError};

pub struct DataUriResolver;

impl Resolver for DataUriResolver {
    fn resolve(
        &mut self,
        uri: &str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Vec<u8>, ResolverError>> + Send>>
    {
        let uri = uri.to_string();

        Box::pin(async move {
            let uri = uri
                .strip_prefix("data:")
                .ok_or_else(|| ResolverError::InvalidUri(uri.to_string()))?;

            let (mime_type, data) = uri
                .split_once(',')
                .ok_or_else(|| ResolverError::InvalidUri(uri.to_string()))?;

            let (_mime_type, base64) = match mime_type.strip_suffix(";base64") {
                Some(mime_type) => (mime_type, true),
                None => (mime_type, false),
            };

            let data = if base64 {
                STANDARD
                    .decode(data.as_bytes())
                    .map_err(|e| ResolverError::ResolutionError(e.to_string()))?
            } else {
                data.as_bytes().to_vec()
            };

            Ok(data)
        })
    }
}
