use std::{collections::HashMap, fs::File, io::BufWriter, path::Path};

use thiserror::Error;
use tracing::warn;

use crate::{
    extensions::Extensions,
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

pub struct GltfIO<R: Resolver> {
    pub extensions: Extensions<GltfDocument, GltfFormat>,
    pub resolver: Option<R>,
}

impl<R: Resolver> GltfIO<R> {
    pub fn new(resolver: R) -> Self {
        Self {
            resolver: Some(resolver),
            extensions: Extensions {
                map: HashMap::new(),
            },
        }
    }

    pub fn export(
        &self,
        graph: &mut Graph,
        doc: &GltfDocument,
    ) -> Result<GltfFormat, GltfExportError> {
        let mut format = export::export(graph, doc)?;

        self.extensions.map.iter().for_each(|(name, ext)| {
            if let Err(e) = ext.export(graph, doc, &mut format) {
                warn!("Failed to export {}: {}", name, e);
            }
        });

        Ok(format)
    }

    pub async fn import(
        &mut self,
        graph: &mut Graph,
        mut format: GltfFormat,
    ) -> Result<GltfDocument, GltfImportError> {
        let doc = import::import(graph, &mut format, &mut self.resolver).await?;

        self.extensions.map.iter().for_each(|(name, ext)| {
            if let Err(e) = ext.import(graph, &mut format, &doc) {
                warn!("Failed to import {}: {}", name, e);
            }
        });

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

impl GltfIO<FileResolver> {
    /// Import a glTF file from a path.
    /// Ignores the current resolver, creating a new FileResolver for the given file's directory.
    pub async fn import_file(
        &self,
        graph: &mut Graph,
        path: &Path,
    ) -> Result<GltfDocument, ImportFileError> {
        let json = serde_json::from_reader(std::fs::File::open(path)?)?;

        let format = GltfFormat {
            json,
            ..Default::default()
        };

        let dir = std::path::Path::new(path).parent().unwrap();

        let mut io = GltfIO {
            resolver: Some(FileResolver::new(dir)),
            extensions: self.extensions.clone(),
        };

        let doc = io.import(graph, format).await?;

        Ok(doc)
    }
}
