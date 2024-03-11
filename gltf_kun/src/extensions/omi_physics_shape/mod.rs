//! [OMI_physics_shape](https://github.com/omigroup/gltf-extensions/tree/main/extensions/2.0/OMI_physics_shape) extension.

use petgraph::graph::NodeIndex;

use crate::graph::{Graph, OtherEdgeHelpers};

use self::physics_shape::{PhysicsShape, PhysicsShapeWeight};

use super::Extension;

pub mod export;
pub mod import;
pub mod json;
pub mod physics_shape;

pub const EXTENSION_NAME: &str = "OMI_physics_shape";
pub const SHAPE_EDGE: &str = "OMI_physics_shape/shape";

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct OmiPhysicsShape(pub NodeIndex);

impl From<NodeIndex> for OmiPhysicsShape {
    fn from(index: NodeIndex) -> Self {
        Self(index)
    }
}

impl From<OmiPhysicsShape> for NodeIndex {
    fn from(physics_shape: OmiPhysicsShape) -> Self {
        physics_shape.0
    }
}

impl OtherEdgeHelpers for OmiPhysicsShape {}

impl Extension for OmiPhysicsShape {
    fn name() -> &'static str {
        EXTENSION_NAME
    }
}

impl OmiPhysicsShape {
    pub fn shapes<'a>(&self, graph: &'a Graph) -> impl Iterator<Item = PhysicsShape> + 'a {
        self.find_properties(graph, SHAPE_EDGE)
    }
    pub fn add_shape(&self, graph: &mut Graph, shape: PhysicsShape) {
        self.add_property(graph, SHAPE_EDGE.to_string(), shape);
    }
    pub fn remove_shape(&self, graph: &mut Graph, shape: PhysicsShape) {
        self.remove_property(graph, SHAPE_EDGE, shape);
    }
    pub fn create_shape(&self, graph: &mut Graph, weight: &PhysicsShapeWeight) -> PhysicsShape {
        let shape = PhysicsShape::new(graph, weight);
        self.add_shape(graph, shape);
        shape
    }
}
