use petgraph::{stable_graph::NodeIndex, visit::EdgeRef};

use super::{primitive::Primitive, Edge, GltfGraph, Weight};

#[derive(Debug, Default)]
pub struct MeshWeight {
    pub name: Option<String>,
    pub extras: gltf::json::Extras,

    pub weights: Vec<f32>,
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Mesh(pub NodeIndex);

impl Mesh {
    pub fn new(graph: &mut GltfGraph) -> Self {
        let index = graph.add_node(Weight::Mesh(MeshWeight::default()));
        Self(index)
    }

    pub fn get<'a>(&'a self, graph: &'a GltfGraph) -> &'a MeshWeight {
        match graph.node_weight(self.0).expect("Weight not found") {
            Weight::Mesh(weight) => weight,
            _ => panic!("Incorrect weight type"),
        }
    }
    pub fn get_mut<'a>(&'a mut self, graph: &'a mut GltfGraph) -> &'a mut MeshWeight {
        match graph.node_weight_mut(self.0).expect("Weight not found") {
            Weight::Mesh(weight) => weight,
            _ => panic!("Incorrect weight type"),
        }
    }

    pub fn primitives(&self, graph: &GltfGraph) -> Vec<Primitive> {
        graph
            .edges_directed(self.0, petgraph::Direction::Outgoing)
            .filter_map(|edge| {
                if let Edge::Primitive = edge.weight() {
                    Some(Primitive(edge.target()))
                } else {
                    None
                }
            })
            .collect()
    }
    pub fn add_primitive(&mut self, graph: &mut GltfGraph, primitive: &Primitive) {
        graph.add_edge(self.0, primitive.0, Edge::Primitive);
    }
    pub fn remove_primitive(&mut self, graph: &mut GltfGraph, primitive: &Primitive) {
        let edge = graph
            .edges_directed(self.0, petgraph::Direction::Outgoing)
            .find(|edge| edge.target() == primitive.0)
            .expect("Primitive not found");

        graph.remove_edge(edge.id());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mesh() {
        let mut graph = GltfGraph::new();
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
