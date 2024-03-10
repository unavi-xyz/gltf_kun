use petgraph::graph::NodeIndex;

use super::Extension;

pub const EXTENSION_NAME: &str = "VRMC_vrm";

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct VrmcVrm(pub NodeIndex);

impl From<NodeIndex> for VrmcVrm {
    fn from(index: NodeIndex) -> Self {
        Self(index)
    }
}

impl From<VrmcVrm> for NodeIndex {
    fn from(physics_shape: VrmcVrm) -> Self {
        physics_shape.0
    }
}

impl Extension for VrmcVrm {
    fn name() -> &'static str {
        EXTENSION_NAME
    }
}
