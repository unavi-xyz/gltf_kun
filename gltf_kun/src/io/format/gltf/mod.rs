use std::{collections::HashMap, fs::File, io::BufWriter, path::Path};

use thiserror::Error;

use crate::{document::GltfDocument, io::resolver::file_resolver::FileResolver};

pub mod export;
pub mod import;

pub struct GltfFormat {
    pub json: gltf::json::Root,
    pub resources: HashMap<String, Vec<u8>>,
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

impl GltfFormat {
    pub async fn import_file(path: &Path) -> Result<GltfDocument, ImportFileError> {
        let json = serde_json::from_reader(std::fs::File::open(path)?)?;

        let format = GltfFormat {
            json,
            resources: HashMap::new(),
        };

        let dir = std::path::Path::new(path).parent().unwrap();
        let mut resolver = FileResolver::new(dir);
        let doc = format.import(Some(&mut resolver)).await?;

        Ok(doc)
    }

    /// Write the glTF to a file.
    /// Path should include the file name and extension.
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
