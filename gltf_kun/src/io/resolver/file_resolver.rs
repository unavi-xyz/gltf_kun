use std::{fs::File, io::Read, path::PathBuf};

use anyhow::Result;
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

impl Resolver for FileResolver {
    fn resolve(&self, uri: &str) -> Result<Vec<u8>> {
        let path = self.root.join(uri);
        debug!("Resolving: {}", path.display());

        let mut file = File::open(path)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;

        Ok(buf)
    }
}
