use std::borrow::Cow;

use anyhow::Result;

use crate::document::GltfDocument;

use super::{gltf::GltfFormat, ExportFormat, ImportFormat};

#[derive(Default)]
pub struct GlbFormat(pub Vec<u8>);

impl GlbFormat {
    pub fn import_slice(bytes: &[u8]) -> Result<GltfDocument> {
        GlbFormat(bytes.to_vec()).import()
    }

    pub fn import_file(path: &str) -> Result<GltfDocument> {
        let bytes = std::fs::read(path)?;
        GlbFormat::import_slice(&bytes)
    }
}

impl ImportFormat<GltfDocument> for GlbFormat {
    fn import(self) -> Result<GltfDocument> {
        let mut glb = gltf::Glb::from_slice(&self.0)?;

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

impl ExportFormat<GltfDocument> for GlbFormat {
    fn export(doc: GltfDocument) -> Result<Box<Self>> {
        let gltf = GltfFormat::export(doc)?;

        let json_bin = serde_json::to_vec(&gltf.json)?;

        let glb = gltf::Glb {
            header: gltf::binary::Header {
                magic: *b"glTF",
                version: 2,
                length: 0,
            },
            json: Cow::Owned(json_bin),
            bin: gltf.blob.map(|blob| blob.into()),
        };

        let bytes = glb.to_vec()?;

        Ok(Box::new(GlbFormat(bytes)))
    }
}
