use petgraph::{graph::NodeIndex, visit::EdgeRef};

use crate::graph::{Edge, Graph, Weight};

use super::{primitive::Primitive, GltfEdge, GltfWeight};

#[derive(Debug, PartialEq, Eq)]
pub enum MeshEdge {
    Primitive,
    Material,
}

#[derive(Debug, Default)]
pub struct MeshWeight {
    pub name: Option<String>,
    pub extras: gltf::json::Extras,

    pub weights: Vec<f32>,
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

impl Mesh {
    pub fn new(graph: &mut Graph) -> Self {
        let index = graph.add_node(Weight::Gltf(GltfWeight::Mesh(MeshWeight::default())));
        Self(index)
    }

    pub fn get<'a>(&'a self, graph: &'a Graph) -> &'a MeshWeight {
        match graph.node_weight(self.0).expect("Weight not found") {
            Weight::Gltf(GltfWeight::Mesh(weight)) => weight,
            _ => panic!("Incorrect weight type"),
        }
    }
    pub fn get_mut<'a>(&'a mut self, graph: &'a mut Graph) -> &'a mut MeshWeight {
        match graph.node_weight_mut(self.0).expect("Weight not found") {
            Weight::Gltf(GltfWeight::Mesh(weight)) => weight,
            _ => panic!("Incorrect weight type"),
        }
    }

    pub fn primitives(&self, graph: &Graph) -> Vec<Primitive> {
        let mut vec = graph
            .edges_directed(self.0, petgraph::Direction::Outgoing)
            .filter_map(|edge| {
                if let Edge::Gltf(GltfEdge::Mesh(MeshEdge::Primitive)) = edge.weight() {
                    Some(Primitive(edge.target()))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        vec.sort();

        vec
    }
    pub fn add_primitive(&self, graph: &mut Graph, primitive: &Primitive) {
        graph.add_edge(
            self.0,
            primitive.0,
            Edge::Gltf(GltfEdge::Mesh(MeshEdge::Primitive)),
        );
    }
    pub fn remove_primitive(&self, graph: &mut Graph, primitive: &Primitive) {
        let edge = graph
            .edges_directed(self.0, petgraph::Direction::Outgoing)
            .find(|edge| edge.target() == primitive.0)
            .expect("Primitive not found");

        graph.remove_edge(edge.id());
    }
    pub fn create_primitive(&self, graph: &mut Graph) -> Primitive {
        let primitive = Primitive::new(graph);
        self.add_primitive(graph, &primitive);
        primitive
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mesh() {
        let mut graph = Graph::new();
        let mut mesh = Mesh::new(&mut graph);

        mesh.get_mut(&mut graph).name = Some("Test".to_string());
        assert_eq!(mesh.get(&graph).name, Some("Test".to_string()));

        mesh.get_mut(&mut graph).weights = vec![1.0, 2.0, 3.0];
        assert_eq!(mesh.get(&graph).weights, vec![1.0, 2.0, 3.0]);

        let primitive = Primitive::new(&mut graph);

        mesh.add_primitive(&mut graph, &primitive);
        assert_eq!(mesh.primitives(&graph), vec![primitive]);

        mesh.remove_primitive(&mut graph, &primitive);
        assert_eq!(mesh.primitives(&graph), vec![]);
    }
}
