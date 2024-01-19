use petgraph::{graph::NodeIndex, visit::EdgeRef};
use serde::{Deserialize, Serialize};

use crate::graph::{gltf::node::Node, ByteNode, Edge, Graph, Weight};

use super::EXTENSION_NAME;

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PhysicsBody(pub NodeIndex);

#[cfg(test)]
mod tests {

    use crate::graph::gltf::document::GltfDocument;

    use super::*;

    #[test]
    fn test_physics_body() {
        let mut graph = Graph::default();

        let doc = GltfDocument::new(&mut graph);
        let _node = doc.create_node(&mut graph);

        // let body = OMIPhysicsBodyExtension::create_body(&mut graph, &node);
        // assert_eq!(OMIPhysicsBodyExtension.properties(&graph).len(), 1);
        // assert_eq!(body.node(&graph), Some(node));
    }
}
