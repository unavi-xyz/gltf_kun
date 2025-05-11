//! [OMI_physics_body](https://github.com/omigroup/gltf-extensions/tree/main/extensions/2.0/OMI_physics_body)
//! extension.

use std::fmt::Display;

use petgraph::graph::NodeIndex;
use serde::{Deserialize, Serialize};

use crate::graph::{ByteNode, Graph, OtherEdgeHelpers};

use self::weight::OmiPhysicsBodyWeight;

use super::{Extension, omi_physics_shape::physics_shape::PhysicsShape};

pub mod export;
pub mod import;
pub mod json;
pub mod weight;

pub const EXTENSION_NAME: &str = "OMI_physics_body";

#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum PhysicsBodyEdge {
    #[serde(rename = "OMI_physics_body/collider")]
    Collider,
    #[serde(rename = "OMI_physics_body/trigger")]
    Trigger,
}

impl Display for PhysicsBodyEdge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = serde_json::to_string(self).unwrap();
        f.write_str(&res)?;
        Ok(())
    }
}

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
impl OtherEdgeHelpers for OmiPhysicsBody {}

impl Extension for OmiPhysicsBody {
    fn name() -> &'static str {
        EXTENSION_NAME
    }
}

impl OmiPhysicsBody {
    pub fn collider(&self, graph: &Graph) -> Option<PhysicsShape> {
        self.find_property(graph, &PhysicsBodyEdge::Collider.to_string())
    }
    pub fn set_collider(&self, graph: &mut Graph, collider: Option<PhysicsShape>) {
        self.set_property(graph, PhysicsBodyEdge::Collider.to_string(), collider);
    }

    pub fn trigger(&self, graph: &Graph) -> Option<PhysicsShape> {
        self.find_property(graph, &PhysicsBodyEdge::Trigger.to_string())
    }
    pub fn set_trigger(&self, graph: &mut Graph, trigger: Option<PhysicsShape>) {
        self.set_property(graph, PhysicsBodyEdge::Trigger.to_string(), trigger);
    }
}
