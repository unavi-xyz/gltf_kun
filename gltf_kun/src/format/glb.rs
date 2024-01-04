use anyhow::Result;

use crate::graph::GltfGraph;

use super::IoFormat;

/// glTF binary format.
pub struct GlbFormat(pub Vec<u8>);

impl IoFormat for GlbFormat {
    fn to_graph(self) -> Result<GltfGraph> {
        todo!()
    }

    fn from_graph(graph: GltfGraph) -> Result<Self> {
        todo!()
    }
}
