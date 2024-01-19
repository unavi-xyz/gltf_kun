use petgraph::graph::NodeIndex;

use crate::graph::{gltf::document::GltfDocument, Edge, Graph};

use self::physics_shape::{PhysicsShape, PhysicsShapeWeight};

use super::Extension;

pub mod io;
pub mod physics_shape;

const EXTENSION_NAME: &str = "OMI_physics_shape";

#[derive(Debug)]
pub struct OMIPhysicsShapeExtension(pub NodeIndex);

impl From<NodeIndex> for OMIPhysicsShapeExtension {
    fn from(index: NodeIndex) -> Self {
        Self(index)
    }
}

impl From<OMIPhysicsShapeExtension> for NodeIndex {
    fn from(physics_shape: OMIPhysicsShapeExtension) -> Self {
        physics_shape.0
    }
}

impl Extension for OMIPhysicsShapeExtension {
    fn name() -> &'static str {
        EXTENSION_NAME
    }
}

impl OMIPhysicsShapeExtension {
    pub fn create_shape(
        graph: &mut Graph,
        doc: &GltfDocument,
        weight: PhysicsShapeWeight,
    ) -> PhysicsShape {
        let shape = PhysicsShape::new(graph, &weight);
        graph.add_edge(doc.0, shape.0, Edge::Extension(EXTENSION_NAME));
        shape
    }
}
