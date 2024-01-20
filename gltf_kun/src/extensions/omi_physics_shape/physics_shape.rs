use petgraph::graph::NodeIndex;
use serde::{Deserialize, Serialize};

use crate::graph::{ByteNode, Graph, Weight};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum PhysicsShapeWeight {
    #[serde(rename = "box")]
    Box(BoxShape),
    #[serde(rename = "sphere")]
    Sphere(SphereShape),
    #[serde(rename = "capsule")]
    Capsule(CapsuleShape),
    #[serde(rename = "cylinder")]
    Cylinder(CylinderShape),
    #[serde(rename = "convex")]
    Convex,
    #[serde(rename = "trimesh")]
    Trimesh,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct BoxShape {
    #[serde(default, skip_serializing_if = "is_default_size")]
    pub size: Size,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SphereShape {
    #[serde(default, skip_serializing_if = "is_default_radius")]
    pub radius: Radius,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CapsuleShape {
    #[serde(default, skip_serializing_if = "is_default_radius")]
    pub radius: Radius,
    #[serde(default, skip_serializing_if = "is_default_height")]
    pub height: Height,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CylinderShape {
    #[serde(default, skip_serializing_if = "is_default_radius")]
    pub radius: Radius,
    #[serde(default, skip_serializing_if = "is_default_height")]
    pub height: Height,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Size(pub [f32; 3]);

impl Default for Size {
    fn default() -> Self {
        Self([1.0, 1.0, 1.0])
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Radius(pub f32);

impl Default for Radius {
    fn default() -> Self {
        Self(0.5)
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Height(pub f32);

impl Default for Height {
    fn default() -> Self {
        Self(2.0)
    }
}

impl From<&Vec<u8>> for PhysicsShapeWeight {
    fn from(bytes: &Vec<u8>) -> Self {
        serde_json::from_slice(bytes).expect("Failed to deserialize physics shape weight")
    }
}

impl From<&PhysicsShapeWeight> for Vec<u8> {
    fn from(value: &PhysicsShapeWeight) -> Self {
        serde_json::to_vec(value).expect("Failed to serialize physics shape weight")
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

impl ByteNode<PhysicsShapeWeight> for PhysicsShape {}

impl PhysicsShape {
    pub fn new(graph: &mut Graph, weight: &PhysicsShapeWeight) -> Self {
        let index = graph.add_node(Weight::Bytes(weight.into()));
        Self(index)
    }
}

fn is_default_size(size: &Size) -> bool {
    size.0 == [1.0, 1.0, 1.0]
}

fn is_default_radius(radius: &Radius) -> bool {
    radius.0 == 0.5
}

fn is_default_height(height: &Height) -> bool {
    height.0 == 2.0
}
