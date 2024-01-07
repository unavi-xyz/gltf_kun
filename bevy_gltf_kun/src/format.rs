use anyhow::Result;
use gltf_kun::{document::Document, io::format::ExportFormat};

pub struct BevyFormat;

impl ExportFormat for BevyFormat {
    fn export(_doc: Document) -> Result<Box<Self>> {
        todo!()
    }
}
