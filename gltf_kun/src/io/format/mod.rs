//! A target format to import from or export to.

pub mod glb;
pub mod gltf;
pub mod glxf;

/// Format -> Graph.
pub trait ImportFormat<T> {
    type Error: std::error::Error;
    fn import(self) -> Result<T, Self::Error>;
}

/// Graph -> Format.
pub trait ExportFormat<T> {
    type Error: std::error::Error;
    fn export(doc: T) -> Result<Box<Self>, Self::Error>;
}
