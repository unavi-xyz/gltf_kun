use anyhow::Result;

pub mod file_resolver;

pub trait Resolver {
    fn resolve(&self, uri: &str) -> Result<Vec<u8>>;
}
