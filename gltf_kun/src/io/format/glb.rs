use std::{borrow::Cow, collections::HashMap, path::Path};

use thiserror::Error;

use crate::{
    extensions::Extensions,
    graph::{gltf::document::GltfDocument, Graph},
    io::resolver::file_resolver::FileResolver,
};

use super::gltf::{export::GltfExportError, import::GltfImportError, GltfFormat, GltfIO};

#[derive(Default)]
pub struct GlbFormat(pub Vec<u8>);

#[derive(Debug, Error)]
pub enum ImportFileError {
    #[error("failed to import gltf: {0}")]
    Import(#[from] GlbImportError),
    #[error("failed to load file: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Default)]
pub struct GlbIO {
    pub extensions: Extensions<GltfDocument, GltfFormat>,
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

impl GlbIO {
    pub fn export(
        &self,
        graph: &mut Graph,
        doc: &GltfDocument,
    ) -> Result<GlbFormat, GlbExportError> {
        if doc.buffers(graph).len() > 1 {
            // TODO: Merge multiple buffers into one (maybe using a transform function)
            return Err(GlbExportError::MultipleBuffers);
        }

        let io = GltfIO::<FileResolver> {
            extensions: self.extensions.clone(),
            resolver: None,
        };
        let gltf = io.export(graph, doc)?;
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

        Ok(GlbFormat(bytes))
    }

    pub async fn import_slice(
        &mut self,
        graph: &mut Graph,
        bytes: &[u8],
    ) -> Result<GltfDocument, GlbImportError> {
        let format = GlbFormat(bytes.to_vec());
        self.import(graph, format).await
    }

    pub async fn import_file(
        &mut self,
        graph: &mut Graph,
        path: &Path,
    ) -> Result<GltfDocument, ImportFileError> {
        let bytes = std::fs::read(path)?;
        let doc = self.import_slice(graph, &bytes).await?;
        Ok(doc)
    }

    pub async fn import(
        &mut self,
        graph: &mut Graph,
        format: GlbFormat,
    ) -> Result<GltfDocument, GlbImportError> {
        let mut glb = gltf::Glb::from_slice(&format.0)?;

        let json = serde_json::from_slice(&glb.json)?;
        let blob = glb.bin.take().map(|blob| blob.into_owned());

        let mut resources = HashMap::new();

        if let Some(blob) = blob {
            resources.insert("bin".to_string(), blob);
        }

        let format = GltfFormat { json, resources };
        let extensions = self.extensions.clone();
        let mut io = GltfIO::<FileResolver> {
            extensions,
            resolver: None,
        };
        let doc = io.import(graph, format).await?;

        Ok(doc)
    }
}
