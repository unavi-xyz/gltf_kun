use std::{fs::File, io::Read, path::PathBuf};

use thiserror::Error;
use tracing::debug;

use super::Resolver;

pub struct FileResolver {
    root: PathBuf,
}

impl FileResolver {
    pub fn new<P: Into<PathBuf>>(root: P) -> Self {
        Self { root: root.into() }
    }
}

#[derive(Debug, Error)]
pub enum FileResolverError {
    #[error("failed to load file: {0}")]
    Io(#[from] std::io::Error),
}

impl Resolver for FileResolver {
    type Error = FileResolverError;

    async fn resolve(&mut self, uri: &str) -> Result<Vec<u8>, Self::Error> {
        let path = self.root.join(uri);
        debug!("Resolving: {}", path.display());

        let mut buf = Vec::new();
        File::open(path)?.read_to_end(&mut buf)?;

        Ok(buf)
    }
}
