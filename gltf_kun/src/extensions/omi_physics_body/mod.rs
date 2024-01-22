//! [OMI_physics_body](https://github.com/omigroup/gltf-extensions/tree/main/extensions/2.0/OMI_physics_body)
//! extension.

use petgraph::{graph::NodeIndex, visit::EdgeRef};
use serde::{Deserialize, Serialize};

use crate::graph::{ByteNode, Edge, Graph, Weight};

use super::{omi_physics_shape::physics_shape::PhysicsShape, Extension};

pub mod io;

pub const EXTENSION_NAME: &str = "OMI_physics_body";
pub const COLLIDER_EDGE: &str = "OMI_physics_body/collider";
pub const TRIGGER_EDGE: &str = "OMI_physics_body/trigger";

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct OMIPhysicsBodyWeight {
    pub motion: Option<Motion>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Motion {
    /// The type of the physics body.
    #[serde(rename = "type")]
    pub typ: BodyType,
    /// The mass of the physics body in kilograms.
    #[serde(default, skip_serializing_if = "is_default_mass")]
    pub mass: Mass,
    /// The initial linear velocity of the body in meters per second.
    #[serde(
        default,
        rename = "linearVelocity",
        skip_serializing_if = "slice_is_zero"
    )]
    pub linear_velocity: [f32; 3],
    /// The initial angular velocity of the body in radians per second.
    #[serde(
        default,
        rename = "angularVelocity",
        skip_serializing_if = "slice_is_zero"
    )]
    pub angular_velocity: [f32; 3],
    /// The center of mass offset from the origin in meters.
    #[serde(
        default,
        rename = "centerOfMass",
        skip_serializing_if = "slice_is_zero"
    )]
    pub center_of_mass: [f32; 3],
    /// The inertia around principle axes in kilogram meter squared (kg⋅m²).
    #[serde(
        default,
        rename = "inertialDiagonal",
        skip_serializing_if = "slice_is_zero"
    )]
    pub intertial_diagonal: [f32; 3],
    /// The inertia orientation as a Quaternion.
    #[serde(
        default,
        rename = "inertiaOrientation",
        skip_serializing_if = "is_default_quat"
    )]
    pub inertia_orientation: Quat,
}

#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Mass(pub f32);

impl Default for Mass {
    fn default() -> Self {
        Self(1.0)
    }
}

impl From<f32> for Mass {
    fn from(value: f32) -> Self {
        Self(value)
    }
}

pub fn is_default_mass(mass: &Mass) -> bool {
    mass.0 == 1.0
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Quat(pub [f32; 4]);

fn is_default_quat(quat: &Quat) -> bool {
    quat.0 == [0.0, 0.0, 0.0, 1.0]
}

impl Default for Quat {
    fn default() -> Self {
        Self([0.0, 0.0, 0.0, 1.0])
    }
}

impl Motion {
    pub fn new(typ: BodyType) -> Self {
        Self {
            typ,
            angular_velocity: Default::default(),
            center_of_mass: Default::default(),
            inertia_orientation: Default::default(),
            intertial_diagonal: Default::default(),
            linear_velocity: Default::default(),
            mass: Default::default(),
        }
    }
}

impl From<&Vec<u8>> for OMIPhysicsBodyWeight {
    fn from(bytes: &Vec<u8>) -> Self {
        if bytes.is_empty() {
            return Self::default();
        }
        serde_json::from_slice(bytes).expect("Failed to deserialize physics body weight")
    }
}

impl From<&OMIPhysicsBodyWeight> for Vec<u8> {
    fn from(value: &OMIPhysicsBodyWeight) -> Self {
        serde_json::to_vec(value).expect("Failed to serialize physics body weight")
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub enum BodyType {
    #[serde(rename = "static")]
    Static,
    #[serde(rename = "dynamic")]
    Dynamic,
    #[serde(rename = "kinematic")]
    Kinematic,
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct OMIPhysicsBody(pub NodeIndex);

impl From<NodeIndex> for OMIPhysicsBody {
    fn from(index: NodeIndex) -> Self {
        Self(index)
    }
}

impl From<OMIPhysicsBody> for NodeIndex {
    fn from(physics_body: OMIPhysicsBody) -> Self {
        physics_body.0
    }
}

impl ByteNode<OMIPhysicsBodyWeight> for OMIPhysicsBody {}

impl Extension for OMIPhysicsBody {
    fn name() -> &'static str {
        EXTENSION_NAME
    }
}

impl OMIPhysicsBody {
    pub fn new(graph: &mut Graph) -> Self {
        let weight = &OMIPhysicsBodyWeight::default();
        let index = graph.add_node(Weight::Bytes(weight.into()));
        Self(index)
    }

    pub fn collider(&self, graph: &Graph) -> Option<PhysicsShape> {
        graph
            .edges_directed(self.0, petgraph::Direction::Outgoing)
            .find(|e| matches!(e.weight(), Edge::Other(COLLIDER_EDGE)))
            .map(|e| PhysicsShape(e.target()))
    }
    pub fn set_collider(&self, graph: &mut Graph, collider: Option<&PhysicsShape>) {
        let edge = graph
            .edges_directed(self.0, petgraph::Direction::Outgoing)
            .find(|e| matches!(e.weight(), Edge::Other(COLLIDER_EDGE)))
            .map(|e| e.id());

        if let Some(edge) = edge {
            graph.remove_edge(edge);
        }

        if let Some(collider) = collider {
            graph.add_edge(self.0, collider.0, Edge::Other(COLLIDER_EDGE));
        }
    }

    pub fn trigger(&self, graph: &Graph) -> Option<PhysicsShape> {
        graph
            .edges_directed(self.0, petgraph::Direction::Outgoing)
            .find(|e| matches!(e.weight(), Edge::Other(TRIGGER_EDGE)))
            .map(|e| PhysicsShape(e.target()))
    }
    pub fn set_trigger(&self, graph: &mut Graph, trigger: Option<&PhysicsShape>) {
        let edge = graph
            .edges_directed(self.0, petgraph::Direction::Outgoing)
            .find(|e| matches!(e.weight(), Edge::Other(TRIGGER_EDGE)))
            .map(|e| e.id());

        if let Some(edge) = edge {
            graph.remove_edge(edge);
        }

        if let Some(trigger) = trigger {
            graph.add_edge(self.0, trigger.0, Edge::Other(TRIGGER_EDGE));
        }
    }
}

fn float_is_zero(num: &f32) -> bool {
    *num == 0.0
}

fn slice_is_zero(slice: &[f32]) -> bool {
    slice.iter().all(float_is_zero)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_mass() {
        assert!(Mass::default().0 == 1.0);
        assert!(is_default_mass(&Mass::default()));
    }

    #[test]
    fn test_default_quat() {
        assert!(Quat::default().0 == [0.0, 0.0, 0.0, 1.0]);
        assert!(is_default_quat(&Quat::default()));
    }
}
