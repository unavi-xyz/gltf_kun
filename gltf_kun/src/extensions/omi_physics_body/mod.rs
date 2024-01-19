use petgraph::{graph::NodeIndex, visit::EdgeRef};
use serde::{Deserialize, Serialize};

use crate::graph::{gltf::node::Node, ByteNode, Edge, Graph, Weight};

use super::{omi_physics_shape::physics_shape::PhysicsShape, Extension, ExtensionIO};

pub mod io;

pub const EXTENSION_NAME: &str = "OMI_physics_body";
pub const COLLIDER_EDGE: &str = "OMI_physics_body/collider";
pub const TRIGGER_EDGE: &str = "OMI_physics_body/trigger";

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct PhysicsBodyWeight {
    pub motion: Option<Motion>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Motion {
    /// The type of the physics body.
    pub typ: BodyType,
    /// The mass of the physics body in kilograms.
    pub mass: f32,
    /// The initial linear velocity of the body in meters per second.
    pub linear_velocity: [f32; 3],
    /// The initial angular velocity of the body in radians per second.
    pub angular_velocity: [f32; 3],
    /// The center of mass offset from the origin in meters.
    pub center_of_mass: [f32; 3],
    /// The inertia around principle axes in kilogram meter squared (kg⋅m²).
    pub intertial_diagonal: [f32; 3],
    /// The inertia orientation as a Quaternion.
    pub inertia_orientation: [f32; 4],
}

impl Motion {
    pub fn new(typ: BodyType) -> Self {
        Self {
            typ,
            mass: 0.0,
            linear_velocity: [0.0; 3],
            angular_velocity: [0.0; 3],
            center_of_mass: [0.0; 3],
            intertial_diagonal: [0.0; 3],
            inertia_orientation: [0.0, 0.0, 0.0, 1.0],
        }
    }
}

impl From<&Vec<u8>> for PhysicsBodyWeight {
    fn from(bytes: &Vec<u8>) -> Self {
        bincode::deserialize(bytes).unwrap()
    }
}

impl From<&PhysicsBodyWeight> for Vec<u8> {
    fn from(value: &PhysicsBodyWeight) -> Self {
        bincode::serialize(value).unwrap()
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub enum BodyType {
    Static,
    Dynamic,
    Kinematic,
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct OMIPhysicsBodyExtension(pub NodeIndex);

impl From<NodeIndex> for OMIPhysicsBodyExtension {
    fn from(index: NodeIndex) -> Self {
        Self(index)
    }
}

impl From<OMIPhysicsBodyExtension> for NodeIndex {
    fn from(physics_body: OMIPhysicsBodyExtension) -> Self {
        physics_body.0
    }
}

impl ByteNode<PhysicsBodyWeight> for OMIPhysicsBodyExtension {}

impl Extension<Node> for OMIPhysicsBodyExtension {
    fn name() -> &'static str {
        EXTENSION_NAME
    }
}

impl OMIPhysicsBodyExtension {
    pub fn new(graph: &mut Graph) -> Self {
        let weight = &PhysicsBodyWeight::default();
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
