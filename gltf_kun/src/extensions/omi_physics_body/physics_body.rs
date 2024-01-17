use petgraph::{stable_graph::NodeIndex, visit::EdgeRef};
use serde::{Deserialize, Serialize};

use crate::{
    extensions::{Extension, ExtensionProperty},
    graph::gltf::{node::Node, Edge, GltfGraph, Weight},
};

use super::{OMIPhysicsBodyExtension, EXTENSION_NAME};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct PhysicsBodyWeight {
    pub motion: Option<Motion>,
}

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub enum BodyType {
    Static,
    Dynamic,
    Kinematic,
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PhysicsBody(pub NodeIndex);

impl PhysicsBody {
    pub fn new(graph: &mut GltfGraph) -> Self {
        let weight = PhysicsBodyWeight::default();
        let index = graph.add_node(Weight::Other(
            OMIPhysicsBodyExtension.encode_property(weight),
        ));
        Self(index)
    }

    pub fn node(&self, graph: &GltfGraph) -> Option<Node> {
        graph
            .edges_directed(self.0, petgraph::Direction::Incoming)
            .find_map(|e| match e.weight() {
                Edge::Extension(EXTENSION_NAME) => Some(Node(e.source())),
                _ => None,
            })
    }
}

impl ExtensionProperty<PhysicsBodyWeight> for PhysicsBody {
    fn index(&self) -> NodeIndex {
        self.0
    }

    fn extension(&self) -> &dyn Extension<PhysicsBodyWeight> {
        &OMIPhysicsBodyExtension
    }
}

#[cfg(test)]
mod tests {
    use crate::{document::GltfDocument, extensions::omi_physics_body::OMIPhysicsBodyExtension};

    use super::*;

    #[test]
    fn test_physics_body() {
        let mut doc = GltfDocument::default();
        let node = doc.create_node();

        let body = OMIPhysicsBodyExtension::create_body(&mut doc.0, &node);
        assert_eq!(OMIPhysicsBodyExtension.properties(&doc.0).len(), 1);
        assert_eq!(body.node(&doc.0), Some(node));
    }
}
