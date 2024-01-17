//! Provides a way to resolve URIs during import.

use std::error::Error;

pub mod file_resolver;

/// Resolves a URI.
pub trait Resolver {
    type Error: Error + Send + Sync + Sized;

    #[allow(async_fn_in_trait)]
    async fn resolve(&mut self, uri: &str) -> Result<Vec<u8>, Self::Error>;
}
