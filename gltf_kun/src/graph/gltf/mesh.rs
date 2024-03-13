use petgraph::graph::NodeIndex;

use crate::graph::{Edge, Extensions, Graph, GraphNodeEdges, GraphNodeWeight, Weight};

use super::{node::NodeEdge, primitive::Primitive, GltfEdge, GltfWeight, Node};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MeshEdge {
    Primitive,
    Material,
}

impl<'a> TryFrom<&'a Edge> for &'a MeshEdge {
    type Error = ();
    fn try_from(value: &'a Edge) -> Result<Self, Self::Error> {
        match value {
            Edge::Gltf(GltfEdge::Mesh(edge)) => Ok(edge),
            _ => Err(()),
        }
    }
}

impl From<MeshEdge> for Edge {
    fn from(edge: MeshEdge) -> Self {
        Self::Gltf(GltfEdge::Mesh(edge))
    }
}

#[derive(Clone, Debug, Default)]
pub struct MeshWeight {
    pub name: Option<String>,
    pub extras: gltf::json::Extras,

    pub weights: Vec<f32>,
}

impl From<MeshWeight> for Weight {
    fn from(weight: MeshWeight) -> Self {
        Self::Gltf(GltfWeight::Mesh(weight))
    }
}

impl<'a> TryFrom<&'a Weight> for &'a MeshWeight {
    type Error = ();
    fn try_from(value: &'a Weight) -> Result<Self, Self::Error> {
        match value {
            Weight::Gltf(GltfWeight::Mesh(weight)) => Ok(weight),
            _ => Err(()),
        }
    }
}

impl<'a> TryFrom<&'a mut Weight> for &'a mut MeshWeight {
    type Error = ();
    fn try_from(value: &'a mut Weight) -> Result<Self, Self::Error> {
        match value {
            Weight::Gltf(GltfWeight::Mesh(weight)) => Ok(weight),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Mesh(pub NodeIndex);

impl From<NodeIndex> for Mesh {
    fn from(index: NodeIndex) -> Self {
        Self(index)
    }
}

impl From<Mesh> for NodeIndex {
    fn from(mesh: Mesh) -> Self {
        mesh.0
    }
}

impl GraphNodeWeight<MeshWeight> for Mesh {}
impl GraphNodeEdges for Mesh {}
impl Extensions for Mesh {}

impl Mesh {
    /// Returns any Nodes using this Mesh.
    pub fn nodes(&self, graph: &Graph) -> Vec<Node> {
        self.edge_sources(graph, &NodeEdge::Mesh)
    }

    pub fn primitives(&self, graph: &Graph) -> Vec<Primitive> {
        self.edge_targets(graph, &MeshEdge::Primitive)
    }
    pub fn add_primitive(&self, graph: &mut Graph, primitive: &Primitive) {
        self.add_edge_target(graph, MeshEdge::Primitive, *primitive);
    }
    pub fn remove_primitive(&self, graph: &mut Graph, primitive: &Primitive) {
        self.remove_edge_target(graph, MeshEdge::Primitive, *primitive);
    }
    pub fn create_primitive(&self, graph: &mut Graph) -> Primitive {
        self.create_edge_target(graph, MeshEdge::Primitive)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nodes() {
        let mut graph = Graph::default();

        let mesh = Mesh::new(&mut graph);
        let node_1 = Node::new(&mut graph);
        let node_2 = Node::new(&mut graph);

        node_1.set_mesh(&mut graph, Some(mesh));
        assert_eq!(mesh.nodes(&graph), vec![node_1]);

        node_2.set_mesh(&mut graph, Some(mesh));
        assert_eq!(mesh.nodes(&graph), vec![node_1, node_2]);

        node_1.set_mesh(&mut graph, None);
        assert_eq!(mesh.nodes(&graph), vec![node_2]);

        node_2.set_mesh(&mut graph, None);
        assert!(mesh.nodes(&graph).is_empty());
    }

    #[test]
    fn primitives() {
        let mut graph = Graph::default();

        let mesh = Mesh::new(&mut graph);
        let primitive_1 = Primitive::new(&mut graph);
        let primitive_2 = Primitive::new(&mut graph);

        mesh.add_primitive(&mut graph, &primitive_1);
        assert_eq!(mesh.primitives(&graph), vec![primitive_1]);

        mesh.add_primitive(&mut graph, &primitive_2);
        assert_eq!(mesh.primitives(&graph), vec![primitive_1, primitive_2]);

        mesh.remove_primitive(&mut graph, &primitive_1);
        assert_eq!(mesh.primitives(&graph), vec![primitive_2]);

        mesh.remove_primitive(&mut graph, &primitive_2);
        assert!(mesh.primitives(&graph).is_empty());
    }
}
