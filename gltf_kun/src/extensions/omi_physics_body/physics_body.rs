use petgraph::{graph::NodeIndex, visit::EdgeRef};
use serde::{Deserialize, Serialize};

use crate::{
    extensions::{Extension, ExtensionProperty},
    graph::{
        gltf::{node::Node, GltfEdge, GltfWeight},
        Edge, Graph, Weight,
    },
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

impl From<NodeIndex> for PhysicsBody {
    fn from(index: NodeIndex) -> Self {
        Self(index)
    }
}

impl From<PhysicsBody> for NodeIndex {
    fn from(physics_body: PhysicsBody) -> Self {
        physics_body.0
    }
}

impl PhysicsBody {
    pub fn new(graph: &mut Graph) -> Self {
        let weight = PhysicsBodyWeight::default();
        let index = graph.add_node(Weight::Gltf(GltfWeight::Other(
            OMIPhysicsBodyExtension.encode_property(weight),
        )));
        Self(index)
    }

    pub fn node(&self, graph: &Graph) -> Option<Node> {
        graph
            .edges_directed(self.0, petgraph::Direction::Incoming)
            .find_map(|e| match e.weight() {
                Edge::Gltf(GltfEdge::Extension(EXTENSION_NAME)) => Some(Node(e.source())),
                _ => None,
            })
    }
}

impl ExtensionProperty<PhysicsBodyWeight> for PhysicsBody {
    fn extension(&self) -> &dyn Extension<PhysicsBodyWeight> {
        &OMIPhysicsBodyExtension
    }
}

#[cfg(test)]
mod tests {

    use crate::graph::gltf::document::GltfDocument;

    use super::*;

    #[test]
    fn test_physics_body() {
        let mut graph = Graph::default();

        let doc = GltfDocument::new(&mut graph);
        let node = doc.create_node(&mut graph);

        let body = OMIPhysicsBodyExtension::create_body(&mut graph, &node);
        assert_eq!(OMIPhysicsBodyExtension.properties(&graph).len(), 1);
        assert_eq!(body.node(&graph), Some(node));
    }
}
