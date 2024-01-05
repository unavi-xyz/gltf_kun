use anyhow::Result;

use crate::document::Document;

pub mod glb;
pub mod gltf;

/// A format for importing and exporting glTF graphs.
pub trait IoFormat: Sized {
    /// Import format -> graph.
    fn import(self) -> Result<Document>;
    /// Export graph -> format.
    fn export(doc: Document) -> Result<Self>;
}
