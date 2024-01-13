//! Provides a way to resolve URIs to buffers during import.

pub mod file_resolver;

/// Resolves a URI to a buffer.
pub trait Resolver {
    type Error: std::error::Error;
    fn resolve(&self, uri: &str) -> Result<Vec<u8>, Self::Error>;
}
