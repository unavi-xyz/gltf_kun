use bevy::prelude::*;
use export::extensions::BevyExportExtensions;
use gltf_kun::graph::gltf::document::GltfDocument;

#[cfg(feature = "export")]
pub mod export;
pub mod extensions;
#[cfg(feature = "import")]
pub mod import;

pub struct GltfKunPlugin<E: BevyExportExtensions<GltfDocument>> {
    _marker: std::marker::PhantomData<E>,
}

impl<E: BevyExportExtensions<GltfDocument>> Default for GltfKunPlugin<E> {
    fn default() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}

impl<E: BevyExportExtensions<GltfDocument>> Plugin for GltfKunPlugin<E> {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "export")]
        app.add_plugins(export::gltf::GltfExportPlugin::<E>::default());
        #[cfg(feature = "import")]
        app.add_plugins(import::gltf::GltfImportPlugin);
        app.add_plugins(extensions::ExtensionsPlugin);
    }
}
