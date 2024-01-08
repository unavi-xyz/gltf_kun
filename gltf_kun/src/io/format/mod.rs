//! A target format to import from or export to.

use anyhow::Result;

pub mod glb;
pub mod gltf;
pub mod glxf;

/// Format -> Graph.
pub trait ImportFormat<T> {
    fn import(self) -> Result<T>;
}

/// Graph -> Format.
pub trait ExportFormat<T> {
    fn export(doc: T) -> Result<Box<Self>>;
}
