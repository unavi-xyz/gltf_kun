//! [OMI_physics_shape](https://github.com/omigroup/gltf-extensions/tree/main/extensions/2.0/OMI_physics_shape) extension.

use petgraph::{graph::NodeIndex, visit::EdgeRef};

use crate::graph::{Edge, Graph};

use self::physics_shape::{PhysicsShape, PhysicsShapeWeight};

use super::Extension;

pub mod io;
pub mod physics_shape;

pub const EXTENSION_NAME: &str = "OMI_physics_shape";
pub const SHAPE_EDGE: &str = "OMI_physics_shape/shape";

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct OMIPhysicsShape(pub NodeIndex);

impl From<NodeIndex> for OMIPhysicsShape {
    fn from(index: NodeIndex) -> Self {
        Self(index)
    }
}

impl From<OMIPhysicsShape> for NodeIndex {
    fn from(physics_shape: OMIPhysicsShape) -> Self {
        physics_shape.0
    }
}

impl Extension for OMIPhysicsShape {
    fn name() -> &'static str {
        EXTENSION_NAME
    }
}

impl OMIPhysicsShape {
    pub fn shapes<'a>(&self, graph: &'a Graph) -> impl Iterator<Item = PhysicsShape> + 'a {
        graph
            .edges_directed(self.0, petgraph::Direction::Outgoing)
            .filter(|e| matches!(e.weight(), Edge::Other(SHAPE_EDGE)))
            .map(|e| PhysicsShape(e.target()))
    }
    pub fn add_shape(&self, graph: &mut Graph, shape: &PhysicsShape) {
        graph.add_edge(self.0, shape.0, Edge::Other(SHAPE_EDGE));
    }
    pub fn remove_shape(&self, graph: &mut Graph, shape: &PhysicsShape) {
        let edge = graph
            .edges_directed(self.0, petgraph::Direction::Outgoing)
            .find(|e| match e.weight() {
                Edge::Other(SHAPE_EDGE) => e.target() == shape.0,
                _ => false,
            })
            .map(|e| e.id());

        if let Some(edge) = edge {
            graph.remove_edge(edge);
        }
    }
    pub fn create_shape(&self, graph: &mut Graph, weight: &PhysicsShapeWeight) -> PhysicsShape {
        let shape = PhysicsShape::new(graph, weight);
        self.add_shape(graph, &shape);
        shape
    }
}
