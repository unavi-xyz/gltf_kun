use anyhow::Result;
use gltf_kun::{document::gltf::GltfDocument, io::format::ImportFormat};

pub struct BevyFormat;

impl ImportFormat<GltfDocument> for BevyFormat {
    fn import(self) -> Result<GltfDocument> {
        todo!()
    }
}
