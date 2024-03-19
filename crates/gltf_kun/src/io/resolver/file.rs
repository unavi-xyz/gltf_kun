use std::{fs::File, io::Read, path::PathBuf};

use super::{Resolver, ResolverError};

pub struct FileResolver {
    root: PathBuf,
}

impl FileResolver {
    pub fn new<P: Into<PathBuf>>(root: P) -> Self {
        Self { root: root.into() }
    }
}

impl Resolver for FileResolver {
    async fn resolve(&mut self, uri: &str) -> Result<Vec<u8>, ResolverError> {
        let path = self.root.join(uri);

        let mut buf = Vec::new();
        let mut file =
            File::open(path).map_err(|e| ResolverError::ResolutionError(e.to_string()))?;
        file.read_to_end(&mut buf)
            .map_err(|e| ResolverError::ResolutionError(e.to_string()))?;

        Ok(buf)
    }
}
