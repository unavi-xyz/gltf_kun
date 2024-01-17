//! Target formats to import from or export to.

use std::error::Error;

pub mod glb;
pub mod gltf;
pub mod glxf;

/// Convert between a document and a format.
pub trait DocumentIO<D, F> {
    type ImportError: Error + Send + Sync;
    type ExportError: Error + Send + Sync;

    fn export(&self, doc: D) -> Result<F, Self::ExportError>;

    #[allow(async_fn_in_trait)]
    async fn import(&mut self, format: F) -> Result<D, Self::ImportError>;
}
