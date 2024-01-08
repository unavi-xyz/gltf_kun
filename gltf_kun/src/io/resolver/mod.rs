//! Provides a way to resolve URIs to buffers during import.

use anyhow::Result;

pub mod file_resolver;

/// Resolves a URI to a buffer.
pub trait Resolver {
    fn resolve(&self, uri: &str) -> Result<Vec<u8>>;
}
