//! Bevy glTF plugin using [gltf_kun](https://crates.io/crates/gltf_kun).
//!
//! This plugin adds support for:
//!
//! - glTF extension processing via hooks
//! - [glTF](https://github.com/KhronosGroup/glTF) import / export
//! - [glXF](https://github.com/KhronosGroup/glXF) import / export

use bevy::prelude::*;
use extensions::ExtensionsPlugin;
use gltf_kun::extensions::DefaultExtensions;
use plugins::{GltfExportPlugin, GltfImportPlugin};

pub mod export;
pub mod extensions;
pub mod import;
mod plugins;

pub struct GltfKunPlugin {
    gltf_export: bool,
    gltf_import: bool,
}

impl Default for GltfKunPlugin {
    fn default() -> Self {
        Self {
            gltf_export: true,
            gltf_import: true,
        }
    }
}

impl Plugin for GltfKunPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ExtensionsPlugin);

        if self.gltf_export {
            app.add_plugins(GltfExportPlugin::<DefaultExtensions>::default());
        }

        if self.gltf_import {
            app.add_plugins(GltfImportPlugin::<DefaultExtensions>::default());
        }
    }
}
