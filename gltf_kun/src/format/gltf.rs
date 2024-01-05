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
    fn import(self) -> Result<Document> {
        todo!()
    }

    fn export(graph: Document) -> Result<Self> {
        todo!()
    }
}
