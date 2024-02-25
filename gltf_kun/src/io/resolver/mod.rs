//! Provides a way to resolve URIs during import.

use thiserror::Error;

mod data;
mod file;

pub use data::*;
pub use file::*;

#[derive(Debug, Error)]
pub enum ResolverError {
    #[error("Invalid URI: {0}")]
    InvalidUri(String),
    #[error("Failed to resolve URI: {0}")]
    ResolutionError(String),
}

/// Resolves a URI.
pub trait Resolver {
    #[allow(async_fn_in_trait)]
    async fn resolve(&mut self, uri: &str) -> Result<Vec<u8>, ResolverError>;
}
