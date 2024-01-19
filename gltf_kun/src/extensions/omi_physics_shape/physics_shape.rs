use petgraph::graph::NodeIndex;
use serde::{Deserialize, Serialize};

use crate::{
    extensions::ExtensionProperty,
    graph::{Graph, Weight},
};

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

impl From<&Vec<u8>> for PhysicsShapeWeight {
    fn from(bytes: &Vec<u8>) -> Self {
        bincode::deserialize(bytes).unwrap()
    }
}

impl From<&PhysicsShapeWeight> for Vec<u8> {
    fn from(value: &PhysicsShapeWeight) -> Self {
        bincode::serialize(value).unwrap()
    }
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
    pub fn new(graph: &mut Graph, weight: &PhysicsShapeWeight) -> Self {
        let index = graph.add_node(Weight::Other(weight.into()));
        Self(index)
    }
}

impl ExtensionProperty<PhysicsShapeWeight> for PhysicsShape {}
