use petgraph::graph::NodeIndex;

use super::Extension;

pub const EXTENSION_NAME: &str = "VRMC_materials_mtoon";

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct VrmcMaterialsMtoon(pub NodeIndex);

impl From<NodeIndex> for VrmcMaterialsMtoon {
    fn from(index: NodeIndex) -> Self {
        Self(index)
    }
}

impl From<VrmcMaterialsMtoon> for NodeIndex {
    fn from(physics_shape: VrmcMaterialsMtoon) -> Self {
        physics_shape.0
    }
}

impl Extension for VrmcMaterialsMtoon {
    fn name() -> &'static str {
        EXTENSION_NAME
    }
}
