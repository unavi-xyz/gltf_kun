use crate::graph::{
    gltf::{node::Node, GltfEdge},
    Edge, Graph,
};

use self::physics_body::{PhysicsBody, PhysicsBodyWeight};

use super::{Extension, ExtensionIO};

pub mod io;
pub mod physics_body;

pub const EXTENSION_NAME: &str = "OMI_physics_body";

#[derive(Clone, Debug)]
pub struct OMIPhysicsBodyExtension;

impl OMIPhysicsBodyExtension {
    pub fn create_body(graph: &mut Graph, node: &Node) -> PhysicsBody {
        let body = PhysicsBody::new(graph);
        graph.add_edge(
            node.0,
            body.0,
            Edge::Gltf(GltfEdge::Extension(EXTENSION_NAME)),
        );
        body
    }
}

impl Extension<PhysicsBodyWeight> for OMIPhysicsBodyExtension {
    fn name(&self) -> &'static str {
        EXTENSION_NAME
    }
}
