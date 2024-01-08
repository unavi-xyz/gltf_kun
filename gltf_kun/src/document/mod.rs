//! A document provides a high-level API for interacting with the internal graph.
//!
//! There are two provided document types:
//! [`GltfDocument`] for [glTF](https://github.com/KhronosGroup/glTF) files,
//! and [`GlxfDocument`] for [glXF](https://github.com/KhronosGroup/glXF) files.

mod gltf;
mod glxf;

pub use gltf::*;
pub use glxf::*;
