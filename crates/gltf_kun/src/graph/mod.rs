//! The internal directed graph.
//!
//! There are provided graph types for both [glTF](https://github.com/KhronosGroup/glTF) and
//! [glXF](https://github.com/KhronosGroup/glXF) files.

use petgraph::stable_graph::StableDiGraph;

mod byte_node;
pub mod gltf;
pub mod glxf;
mod graph_node;
mod other_edge;
mod property;

pub use byte_node::ByteNode;
pub use graph_node::{GraphNodeEdges, GraphNodeWeight};
pub use other_edge::OtherEdgeHelpers;
pub use petgraph::stable_graph::NodeIndex;
pub use property::Extensions;

use self::{
    gltf::{GltfEdge, GltfWeight},
    glxf::{GlxfEdge, GlxfWeight},
};

#[derive(Clone, Debug)]
pub enum Weight {
    Bytes(Vec<u8>),
    Gltf(GltfWeight),
    Glxf(GlxfWeight),
}

#[derive(Clone, Debug)]
pub enum Edge {
    Extension(&'static str),
    Gltf(GltfEdge),
    Glxf(GlxfEdge),
    Other(String),
}

pub type Graph = StableDiGraph<Weight, Edge>;
