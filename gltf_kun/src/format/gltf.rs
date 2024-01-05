use anyhow::Result;

use crate::document::Document;

use super::IoFormat;

pub struct GltfFormat(pub gltf::Gltf);

impl IoFormat for GltfFormat {
    fn import(self) -> Result<Document> {
        todo!()
    }

    fn export(graph: Document) -> Result<Self> {
        todo!()
    }
}

pub struct GlbFormat<'a>(pub gltf::Glb<'a>);

impl<'a> IoFormat for GlbFormat<'a> {
    fn import(mut self) -> Result<Document> {
        let json = serde_json::from_slice(&self.0.json)?;
        let document = gltf::Document::from_json(json)?;

        let blob = self.0.bin.take().map(|blob| blob.into_owned());
        let gltf = gltf::Gltf { document, blob };

        GltfFormat(gltf).import()
    }

    fn export(graph: Document) -> Result<Self> {
        todo!()
    }
}
