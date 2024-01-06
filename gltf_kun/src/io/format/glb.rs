use anyhow::Result;

use crate::document::Document;

use super::{gltf::GltfFormat, ImportFormat};

pub struct GlbFormat<'a>(pub &'a [u8]);

impl<'a> GlbFormat<'a> {
    pub fn import_slice(bytes: &'a [u8]) -> Result<Document> {
        GlbFormat(bytes).import()
    }

    pub fn import_file(path: &str) -> Result<Document> {
        let bytes = std::fs::read(path)?;
        GlbFormat::import_slice(&bytes)
    }
}

impl<'a> ImportFormat for GlbFormat<'a> {
    fn import(self) -> Result<Document> {
        let mut glb = gltf::Glb::from_slice(self.0)?;

        let json = serde_json::from_slice(&glb.json)?;
        let blob = glb.bin.take().map(|blob| blob.into_owned());

        GltfFormat {
            json,
            blob,
            resolver: None,
        }
        .import()
    }
}
