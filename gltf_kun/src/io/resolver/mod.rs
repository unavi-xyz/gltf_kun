//! Provides a way to resolve URIs during import.

pub mod file_resolver;

/// Resolves a URI.
pub trait Resolver {
    type Error: std::error::Error;
    fn resolve(
        &mut self,
        uri: &str,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, Self::Error>>;
}

pub const NO_RESOLVER: Option<&mut EmptyResolver> = None;

/// A resolver that always returns an empty buffer.
pub struct EmptyResolver;

impl Resolver for EmptyResolver {
    type Error = std::convert::Infallible;

    async fn resolve(
        &mut self,
        _uri: &str,
    ) -> Result<Vec<u8>, Self::Error> { Ok(Vec::new()) }
}
