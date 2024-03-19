//! Bevy glTF plugin using [gltf_kun](https://crates.io/crates/gltf_kun).
//!
//! ## Comparison
//!
//! Compared to `bevy_gltf`, this plugin adds support for:
//!
//! - Arbitrary glTF extensions
//! - glTF exporting
//! - [glXF](https://github.com/KhronosGroup/glXF) import and export

#[cfg(feature = "export")]
pub mod export;
pub mod extensions;
#[cfg(feature = "import")]
pub mod import;
