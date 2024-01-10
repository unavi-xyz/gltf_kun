use std::{collections::HashMap, fs::File, io::BufWriter, path::Path};

use anyhow::Result;

use crate::{
    document::GltfDocument,
    io::resolver::{file_resolver::FileResolver, Resolver},
};

use super::ImportFormat;

pub mod export;
pub mod import;

pub struct GltfFormat {
    pub json: gltf::json::Root,
    pub resources: HashMap<String, Vec<u8>>,
    pub resolver: Option<Box<dyn Resolver>>,
}

impl GltfFormat {
    pub fn import_file(path: &Path) -> Result<GltfDocument> {
        let json = serde_json::from_reader(std::fs::File::open(path)?)?;

        let dir = std::path::Path::new(path)
            .parent()
            .expect("Failed to get parent directory");
        let resolver = FileResolver::new(dir);

        GltfFormat {
            json,
            resolver: Some(Box::new(resolver)),
            resources: HashMap::new(),
        }
        .import()
    }

    /// Write the glTF to a file.
    /// Path should include the file name and extension.
    /// Resources will be written to the same directory.
    pub fn write_file(&self, path: &Path) -> Result<()> {
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
