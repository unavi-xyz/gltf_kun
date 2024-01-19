use crate::graph::{gltf::document::GltfDocument, Edge, Graph};

use self::physics_shape::{PhysicsShape, PhysicsShapeWeight};

pub mod io;
pub mod physics_shape;

const EXTENSION_NAME: &str = "OMI_physics_shape";

#[derive(Clone, Debug)]
pub struct OMIPhysicsShapeExtension;

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
