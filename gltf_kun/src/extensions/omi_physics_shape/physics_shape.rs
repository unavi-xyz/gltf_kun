use petgraph::graph::NodeIndex;
use serde::{Deserialize, Serialize};

use crate::{
    extensions::{Extension, ExtensionProperty},
    graph::{gltf::GltfWeight, Graph, Weight},
};

use super::OMIPhysicsShapeExtension;

#[derive(Debug, Deserialize, Serialize)]
pub enum PhysicsShapeWeight {
    Box(BoxShape),
    Sphere(SphereShape),
    Capsule(CapsuleShape),
    Cylinder(CylinderShape),
    Convex,
    Trimesh,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct BoxShape {
    pub size: [f32; 3],
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SphereShape {
    pub radius: f32,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct CapsuleShape {
    pub radius: f32,
    pub height: f32,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct CylinderShape {
    pub radius: f32,
    pub height: f32,
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PhysicsShape(pub NodeIndex);

impl From<NodeIndex> for PhysicsShape {
    fn from(index: NodeIndex) -> Self {
        Self(index)
    }
}

impl From<PhysicsShape> for NodeIndex {
    fn from(physics_shape: PhysicsShape) -> Self {
        physics_shape.0
    }
}

impl PhysicsShape {
    pub fn new(graph: &mut Graph, weight: PhysicsShapeWeight) -> Self {
        let index = graph.add_node(Weight::Gltf(GltfWeight::Other(
            OMIPhysicsShapeExtension.encode_property(weight),
        )));
        Self(index)
    }
}

impl ExtensionProperty<PhysicsShapeWeight> for PhysicsShape {
    fn extension(&self) -> &dyn Extension<PhysicsShapeWeight> {
        &OMIPhysicsShapeExtension
    }
}
