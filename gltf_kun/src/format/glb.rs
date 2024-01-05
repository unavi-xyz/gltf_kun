use anyhow::Result;

use crate::document::Document;

use super::{gltf::GltfFormat, IoFormat};

pub struct GlbFormat<'a>(pub gltf::Glb<'a>);

impl<'a> IoFormat for GlbFormat<'a> {
    fn import(mut self) -> Result<Document> {
        let json = serde_json::from_slice(&self.0.json)?;
        let blob = self.0.bin.take().map(|blob| blob.into_owned());

        GltfFormat { json, blob }.import()
    }

    fn export(graph: Document) -> Result<Self> {
        todo!()
    }
}
