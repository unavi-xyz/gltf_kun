//! Graph-based [glTF](https://github.com/KhronosGroup/glTF) processing library.
//! Uses [petgraph](https://crates.io/crates/petgraph) to create a traversable graph of the glTF document.
//!
//! ## Usage
//!
//! ```
//! use gltf_kun::graph::{Graph, GraphNodeWeight, gltf::document::GltfDocument};
//!
//! let mut graph = Graph::default();
//!
//! // Create a new glTF document within the graph.
//! let doc = GltfDocument::new(&mut graph);
//!
//! // Create a new scene.
//! // This `scene` variable is just a wrapper around a u32 index into the graph,
//! // making it cheap to copy and pass around.
//! let mut scene = doc.create_scene(&mut graph);
//!
//! // To read or write data, we need to get the weight.
//! let weight = scene.get_mut(&mut graph);
//! weight.name = Some("My Scene".to_string());
//!
//! // Create a glTF node and add it to the scene.
//! let mut node = doc.create_node(&mut graph);
//! scene.add_node(&mut graph, node);
//!
//! // Iterate over all scenes in the document, printing their names.
//! doc.scenes(&graph).iter().for_each(|scene| {
//!     let weight = scene.get(&graph);
//!     println!("Scene name: {:?}", weight.name);
//! });
//!
//! use gltf_kun::{extensions::DefaultExtensions, io::format::glb::GlbExport};
//!
//! // Export the document to a GLB byte array.
//! let glb = GlbExport::<DefaultExtensions>::export(&mut graph, &doc).ok();
//! ```

pub mod extensions;
pub mod graph;
pub mod io;
