use std::{collections::HashMap, fs::File, io::BufWriter, path::Path};

use thiserror::Error;
use tracing::warn;

use crate::{
    extensions::ExtensionsIO,
    graph::{gltf::document::GltfDocument, Graph},
    io::resolver::{file_resolver::FileResolver, Resolver},
};

use self::{export::GltfExportError, import::GltfImportError};

pub mod export;
pub mod import;

#[derive(Default)]
pub struct GltfFormat {
    pub json: gltf::json::Root,
    pub resources: HashMap<String, Vec<u8>>,
}

impl GltfFormat {
    /// Write the glTF to a file.
    /// Resources will be written to the same directory.
    pub fn write_file(&self, path: &Path) -> Result<(), WriteFileError> {
        tracing::info!("Writing glTF to file: {:?}", path.as_os_str());

        // Write json file
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, &self.json)?;

        // Write resources
        let dir = Path::new(path)
            .parent()
            .expect("Failed to get parent directory");

        self.resources.iter().for_each(|(k, v)| {
            let path = dir.join(k);
            std::fs::write(path, v).expect("Failed to write resource");
        });

        Ok(())
    }
}

pub struct GltfIO;

impl GltfIO {
    pub fn export(
        &self,
        graph: &mut Graph,
        doc: &GltfDocument,
        extensions: Option<&impl ExtensionsIO<GltfDocument, GltfFormat>>,
    ) -> Result<GltfFormat, GltfExportError> {
        let mut format = export::export(graph, doc)?;

        if let Some(extensions) = extensions {
            if let Err(e) = extensions.export(graph, doc, &mut format) {
                warn!("Failed to export extensions: {}", e);
            }
        }

        Ok(format)
    }

    pub async fn import(
        &self,
        graph: &mut Graph,
        mut format: GltfFormat,
        mut resolver: Option<impl Resolver>,
        extensions: Option<&impl ExtensionsIO<GltfDocument, GltfFormat>>,
    ) -> Result<GltfDocument, GltfImportError> {
        let doc = import::import(graph, &mut format, &mut resolver).await?;

        if let Some(extensions) = extensions {
            if let Err(e) = extensions.import(graph, &mut format, &doc) {
                warn!("Failed to import extensions: {}", e);
            }
        }

        Ok(doc)
    }

    /// Import a glTF file from a path.
    pub async fn import_file(
        &self,
        graph: &mut Graph,
        path: &Path,
        extensions: Option<&impl ExtensionsIO<GltfDocument, GltfFormat>>,
    ) -> Result<GltfDocument, ImportFileError> {
        let format = GltfFormat {
            json: serde_json::from_reader(std::fs::File::open(path)?)?,
            ..Default::default()
        };

        let dir = std::path::Path::new(path).parent().unwrap();
        let resolver = FileResolver::new(dir);

        let doc = self
            .import(graph, format, Some(resolver), extensions)
            .await?;

        Ok(doc)
    }
}

#[derive(Debug, Error)]
pub enum ImportFileError {
    #[error("failed to import gltf: {0}")]
    Import(#[from] import::GltfImportError),
    #[error("failed to load file: {0}")]
    Io(#[from] std::io::Error),
    #[error("failed to parse json: {0}")]
    SerdeJson(#[from] serde_json::Error),
}

#[derive(Debug, Error)]
pub enum WriteFileError {
    #[error("failed to write file: {0}")]
    Io(#[from] std::io::Error),
    #[error("failed to serialize json: {0}")]
    SerdeJson(#[from] serde_json::Error),
}
