//! [OMI_physics_body](https://github.com/omigroup/gltf-extensions/tree/main/extensions/2.0/OMI_physics_body)
//! extension.

use petgraph::{graph::NodeIndex, visit::EdgeRef};

use crate::graph::{ByteNode, Edge, Graph, Weight};

use super::{omi_physics_shape::physics_shape::PhysicsShape, Extension};

pub mod io;
mod weight;

pub use weight::*;

pub const EXTENSION_NAME: &str = "OMI_physics_body";
pub const COLLIDER_EDGE: &str = "OMI_physics_body/collider";
pub const TRIGGER_EDGE: &str = "OMI_physics_body/trigger";

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct OmiPhysicsBody(pub NodeIndex);

impl From<NodeIndex> for OmiPhysicsBody {
    fn from(index: NodeIndex) -> Self {
        Self(index)
    }
}

impl From<OmiPhysicsBody> for NodeIndex {
    fn from(physics_body: OmiPhysicsBody) -> Self {
        physics_body.0
    }
}

impl ByteNode<OmiPhysicsBodyWeight> for OmiPhysicsBody {}

impl Extension for OmiPhysicsBody {
    fn name() -> &'static str {
        EXTENSION_NAME
    }
}

impl OmiPhysicsBody {
    pub fn collider_edge_name() -> &'static str {
        COLLIDER_EDGE
    }
    pub fn trigger_edge_name() -> &'static str {
        TRIGGER_EDGE
    }

    pub fn new(graph: &mut Graph) -> Self {
        let weight = &OmiPhysicsBodyWeight::default();
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
