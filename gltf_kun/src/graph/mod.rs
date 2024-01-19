//! The internal directed graph.
//!
//! There are provided graph types for both [glTF](https://github.com/KhronosGroup/glTF) and
//! [glXF](https://github.com/KhronosGroup/glXF) files.

use petgraph::graph::DiGraph;

pub mod gltf;
pub mod glxf;
mod graph_node;
mod property;

pub use graph_node::GraphNode;
pub use petgraph::graph::NodeIndex;
pub use property::Property;

#[derive(Debug)]
pub enum Weight {
    Gltf(gltf::GltfWeight),
    Glxf(glxf::Weight),
    Other(Vec<u8>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Edge {
    Extension(&'static str),
    Gltf(gltf::GltfEdge),
    Glxf(glxf::Edge),
}

pub type Graph = DiGraph<Weight, Edge>;
