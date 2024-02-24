//! Provides a way to resolve URIs during import.

use std::{future::Future, pin::Pin};

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
pub trait Resolver: Send + Sync {
    fn resolve(
        &mut self,
        uri: &str,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<u8>, ResolverError>> + Send + '_>>;
}
