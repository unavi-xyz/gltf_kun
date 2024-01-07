use anyhow::Result;

use crate::document::Document;

pub mod glb;
pub mod gltf;

/// Import format -> graph.
pub trait ImportFormat {
    fn import(self) -> Result<Document>;
}

/// Export graph -> format.
pub trait ExportFormat {
    fn export(doc: Document) -> Result<Box<Self>>;
}
