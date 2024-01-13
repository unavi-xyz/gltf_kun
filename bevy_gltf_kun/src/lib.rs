use bevy::prelude::*;

#[cfg(feature = "import")]
mod default_plugins;
#[cfg(feature = "export")]
pub mod export;
#[cfg(feature = "import")]
pub mod import;

#[cfg(feature = "import")]
pub use default_plugins::DefaultPlugins;

pub struct GltfKunPlugin;

impl Plugin for GltfKunPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "export")]
        app.add_plugins(export::GltfExportPlugin);
        #[cfg(feature = "import")]
        app.add_plugins(import::GltfImportPlugin);
    }
}
