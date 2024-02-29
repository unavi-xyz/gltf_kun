use std::{collections::HashMap, fs::File, io::BufWriter, path::Path};

use thiserror::Error;
use tracing::{debug, warn};

use crate::{
    extensions::ExtensionsIO,
    graph::{gltf::document::GltfDocument, Graph},
    io::resolver::{FileResolver, Resolver},
};

use self::{export::GltfExportError, import::GltfImportError};

pub mod export;
pub mod import;

#[derive(Debug, Default)]
pub struct GltfFormat {
    pub json: gltf::json::Root,
    pub resources: HashMap<String, Vec<u8>>,
}

#[derive(Debug, Error)]
pub enum WriteFileError {
    #[error("Failed to write file: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to serialize json: {0}")]
    SerdeJson(#[from] serde_json::Error),
}

impl GltfFormat {
    /// Write the glTF to a file.
    /// Resources will be written to the same directory.
    pub fn write_file(&self, path: &Path) -> Result<(), WriteFileError> {
        debug!("Writing glTF to file: {:?}", path.as_os_str());

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

pub struct GltfIO<E: ExtensionsIO<GltfDocument, GltfFormat>> {
    pub _marker: std::marker::PhantomData<E>,
}

#[derive(Debug, Error)]
pub enum ImportFileError {
    #[error("Failed to import gltf: {0}")]
    Import(#[from] import::GltfImportError),
    #[error("Failed to load file: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to parse json: {0}")]
    SerdeJson(#[from] serde_json::Error),
}

impl<E: ExtensionsIO<GltfDocument, GltfFormat>> GltfIO<E> {
    pub fn export(graph: &mut Graph, doc: &GltfDocument) -> Result<GltfFormat, GltfExportError> {
        let mut format = export::export(graph, doc)?;

        if let Err(e) = E::export(graph, doc, &mut format) {
            warn!("Failed to export extensions: {}", e);
        }

        Ok(format)
    }

    pub async fn import(
        graph: &mut Graph,
        mut format: GltfFormat,
        resolver: Option<impl Resolver>,
    ) -> Result<GltfDocument, GltfImportError> {
        let doc = import::import(graph, &mut format, resolver).await?;

        if let Err(e) = E::import(graph, &mut format, &doc) {
            warn!("Failed to import extensions: {}", e);
        }

        Ok(doc)
    }

    /// Import a glTF file from a path.
    pub async fn import_file(
        graph: &mut Graph,
        path: &Path,
    ) -> Result<GltfDocument, ImportFileError> {
        let format = GltfFormat {
            json: serde_json::from_reader(std::fs::File::open(path)?)?,
            ..Default::default()
        };

        let dir = std::path::Path::new(path).parent().unwrap();
        let resolver = FileResolver::new(dir);

        let doc = Self::import(graph, format, Some(resolver)).await?;

        Ok(doc)
    }
}
