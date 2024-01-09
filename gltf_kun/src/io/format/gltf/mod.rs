use std::collections::HashMap;

use anyhow::Result;

use crate::{
    document::GltfDocument,
    io::resolver::{file_resolver::FileResolver, Resolver},
};

use super::ImportFormat;

pub mod export;
pub mod import;

pub struct GltfFormat {
    pub json: gltf::json::Root,
    pub resources: HashMap<String, Vec<u8>>,
    pub resolver: Option<Box<dyn Resolver>>,
}

impl GltfFormat {
    pub fn import_file(path: &str) -> Result<GltfDocument> {
        let json = serde_json::from_reader(std::fs::File::open(path)?)?;

        let dir = std::path::Path::new(path)
            .parent()
            .expect("Failed to get parent directory");
        let resolver = FileResolver::new(dir);

        GltfFormat {
            json,
            resolver: Some(Box::new(resolver)),
            resources: HashMap::new(),
        }
        .import()
    }
}
