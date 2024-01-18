use crate::graph::gltf::GltfGraph;

use self::physics_shape::{PhysicsShape, PhysicsShapeWeight};

use super::Extension;

pub mod io;
pub mod physics_shape;

const EXTENSION_NAME: &str = "OMI_physics_shape";

#[derive(Clone, Debug)]
pub struct OMIPhysicsShapeExtension;

impl OMIPhysicsShapeExtension {
    pub fn create_shape(graph: &mut GltfGraph, weight: PhysicsShapeWeight) -> PhysicsShape {
        let body = PhysicsShape::new(graph, weight);
        body
    }
}

impl Extension<PhysicsShapeWeight> for OMIPhysicsShapeExtension {
    fn name(&self) -> &'static str {
        EXTENSION_NAME
    }
}
