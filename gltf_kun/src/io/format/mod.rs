use anyhow::Result;

use crate::document::Document;

pub mod glb;
pub mod gltf;

/// Import format -> graph.
pub trait ImportFormat: Sized {
    fn import(self) -> Result<Document>;
}

/// Export graph -> format.
pub trait ExportFormat: Sized {
    fn export(doc: Document) -> Result<Self>;
}
