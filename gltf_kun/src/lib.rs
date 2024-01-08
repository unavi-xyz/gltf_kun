//! An extensible library for building [glTF](https://github.com/KhronosGroup/glTF) toolchains.
//! Builds upon the [petgraph](https://crates.io/crates/petgraph) crate to create a traversable graph of the glTF document.
//!
//! ## Basic Usage
//!
//! ```
//! use gltf_kun::document::GltfDocument;
//!
//! // Create a new glTF document.
//! // This stores the graph, which will be passed in to methods using "doc.0".
//! let mut doc = GltfDocument::default();
//!
//! // Create a new scene.
//! // This "scene" variable is just a wrapper around a u32 index into the graph,
//! // making it cheap to copy and pass around.
//! let mut scene = doc.create_scene();
//!
//! // To read or write data, we need to get its weight.
//! let weight = scene.get_mut(&mut doc.0);
//! weight.name = Some("My Scene".to_string());
//!
//! // Create a glTF node and add it to the scene.
//! let mut node = doc.create_node();
//! scene.add_node(&mut doc.0, &node);
//!
//! // Iterate over all scenes in the document, printing their names.
//! doc.scenes().iter().for_each(|scene| {
//!     let weight = scene.get(&doc.0);
//!     println!("Scene name: {:?}", weight.name);
//! });
//!
//! use gltf_kun::io::format::{ExportFormat, glb::GlbFormat};
//!
//! // Export the document to a GLB byte array.
//! let glb = GlbFormat::export(doc).ok();
//! ```

pub mod document;
pub mod extension;
pub mod graph;
pub mod io;
