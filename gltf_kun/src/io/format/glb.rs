use std::{borrow::Cow, collections::HashMap, path::Path};

use thiserror::Error;

use crate::{document::GltfDocument, io::resolver::NO_RESOLVER};

use super::gltf::{export::GltfExportError, import::GltfImportError, GltfFormat};

#[derive(Default)]
pub struct GlbFormat(pub Vec<u8>);

#[derive(Debug, Error)]
pub enum ImportFileError {
    #[error("failed to import gltf: {0}")]
    Import(#[from] GlbImportError),
    #[error("failed to load file: {0}")]
    Io(#[from] std::io::Error),
}

impl GlbFormat {
    pub async fn import_slice(bytes: &[u8]) -> Result<GltfDocument, GlbImportError> {
        GlbFormat(bytes.to_vec()).import().await
    }

    pub async fn import_file(path: &Path) -> Result<GltfDocument, ImportFileError> {
        let bytes = std::fs::read(path)?;
        let doc = GlbFormat::import_slice(&bytes).await?;
        Ok(doc)
    }
}

#[derive(Debug, Error)]
pub enum GlbExportError {
    #[error("failed to export gltf: {0}")]
    Export(#[from] GltfExportError),
    #[error("failed to export glb: {0}")]
    Gltf(#[from] gltf::Error),
    #[error("glb only supports one buffer")]
    MultipleBuffers,
    #[error("failed to serialize json: {0}")]
    SerdeJson(#[from] serde_json::Error),
}

#[derive(Debug, Error)]
pub enum GlbImportError {
    #[error("failed to parse glb: {0}")]
    Gltf(#[from] gltf::Error),
    #[error("failed to import gltf: {0}")]
    Import(#[from] GltfImportError),
    #[error("failed to parse json: {0}")]
    SerdeJson(#[from] serde_json::Error),
}

impl GlbFormat {
    pub fn export(doc: GltfDocument) -> Result<Box<Self>, GlbExportError> {
        if doc.buffers().len() > 1 {
            // TODO: Merge multiple buffers into one (maybe using a transform function)
            return Err(GlbExportError::MultipleBuffers);
        }

        let gltf = GltfFormat::export(doc)?;
        let json_bin = serde_json::to_vec(&gltf.json)?;
        let resource = gltf.resources.iter().find(|_| true);

        let glb = gltf::Glb {
            header: gltf::binary::Header {
                magic: *b"glTF",
                version: 2,
                length: 0,
            },
            json: Cow::Owned(json_bin),
            bin: resource.map(|(_, blob)| blob.into()),
        };

        let bytes = glb.to_vec()?;

        Ok(Box::new(GlbFormat(bytes)))
    }

    pub async fn import(self) -> Result<GltfDocument, GlbImportError> {
        let mut glb = gltf::Glb::from_slice(&self.0)?;

        let json = serde_json::from_slice(&glb.json)?;
        let blob = glb.bin.take().map(|blob| blob.into_owned());

        let mut resources = HashMap::new();

        if let Some(blob) = blob {
            resources.insert("bin".to_string(), blob);
        }

        let format = GltfFormat { json, resources };
        let doc = format.import(NO_RESOLVER).await?;

        Ok(doc)
    }
}
