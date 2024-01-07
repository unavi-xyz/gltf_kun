use anyhow::Result;

pub mod glb;
pub mod gltf;

/// Import format -> graph.
pub trait ImportFormat<T> {
    fn import(self) -> Result<T>;
}

/// Export graph -> format.
pub trait ExportFormat<T> {
    fn export(doc: T) -> Result<Box<Self>>;
}
