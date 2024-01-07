use anyhow::Result;
use gltf_kun::{document::Document, io::format::ImportFormat};

pub struct BevyFormat;

impl ImportFormat for BevyFormat {
    fn import(self) -> Result<Document> {
        todo!()
    }
}
