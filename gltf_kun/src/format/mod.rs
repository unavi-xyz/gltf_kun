use anyhow::Result;

use crate::graph::GltfGraph;

pub mod glb;

/// A format for importing and exporting glTF graphs.
pub trait IoFormat: Sized {
    fn to_graph(self) -> Result<GltfGraph>;
    fn from_graph(graph: GltfGraph) -> Result<Self>;
}
