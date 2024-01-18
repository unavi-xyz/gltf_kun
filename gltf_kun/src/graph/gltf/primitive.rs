use petgraph::{graph::NodeIndex, visit::EdgeRef};

use super::{accessor::Accessor, Edge, GltfGraph, Weight};

pub use gltf::json::mesh::{Mode, Semantic};

#[derive(Debug)]
pub struct PrimitiveWeight {
    pub extras: gltf::json::Extras,
    pub mode: Mode,
}

impl Default for PrimitiveWeight {
    fn default() -> Self {
        Self {
            extras: None,
            mode: Mode::Triangles,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Primitive(pub NodeIndex);

impl Primitive {
    pub fn new(graph: &mut GltfGraph) -> Self {
        let index = graph.add_node(Weight::Primitive(PrimitiveWeight::default()));
        Self(index)
    }

    pub fn get<'a>(&'a self, graph: &'a GltfGraph) -> &'a PrimitiveWeight {
        match graph.node_weight(self.0).expect("Weight not found") {
            Weight::Primitive(weight) => weight,
            _ => panic!("Incorrect weight type"),
        }
    }
    pub fn get_mut<'a>(&'a mut self, graph: &'a mut GltfGraph) -> &'a mut PrimitiveWeight {
        match graph.node_weight_mut(self.0).expect("Weight not found") {
            Weight::Primitive(weight) => weight,
            _ => panic!("Incorrect weight type"),
        }
    }

    pub fn indices(&self, graph: &GltfGraph) -> Option<Accessor> {
        graph
            .edges_directed(self.0, petgraph::Direction::Outgoing)
            .find_map(|edge| {
                if let Edge::Indices = edge.weight() {
                    Some(Accessor(edge.target()))
                } else {
                    None
                }
            })
    }
    pub fn set_indices(&mut self, graph: &mut GltfGraph, indices: Option<&Accessor>) {
        let edge = graph
            .edges_directed(self.0, petgraph::Direction::Outgoing)
            .find(|edge| matches!(edge.weight(), Edge::Indices))
            .map(|edge| edge.id());

        if let Some(edge) = edge {
            graph.remove_edge(edge);
        }

        if let Some(indices) = indices {
            graph.add_edge(self.0, indices.0, Edge::Indices);
        }
    }

    pub fn attributes(&self, graph: &GltfGraph) -> Vec<(Semantic, Accessor)> {
        graph
            .edges_directed(self.0, petgraph::Direction::Outgoing)
            .filter_map(|edge| {
                if let Edge::Attribute(semantic) = edge.weight() {
                    Some((semantic.clone(), Accessor(edge.target())))
                } else {
                    None
                }
            })
            .collect()
    }
    pub fn attribute(&self, graph: &GltfGraph, semantic: &Semantic) -> Option<Accessor> {
        graph
            .edges_directed(self.0, petgraph::Direction::Outgoing)
            .find_map(|edge| {
                if let Edge::Attribute(edge_semantic) = edge.weight() {
                    if edge_semantic == semantic {
                        Some(Accessor(edge.target()))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
    }
    pub fn set_attribute(
        &mut self,
        graph: &mut GltfGraph,
        semantic: &Semantic,
        accessor: Option<&Accessor>,
    ) {
        if let Some(accessor) = accessor {
            graph.add_edge(self.0, accessor.0, Edge::Attribute(semantic.clone()));
        } else if let Some(edge) = graph
            .edges_directed(self.0, petgraph::Direction::Outgoing)
            .find(|edge| {
                if let Edge::Attribute(edge_semantic) = edge.weight() {
                    edge_semantic == semantic
                } else {
                    false
                }
            })
        {
            graph.remove_edge(edge.id());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primitive() {
        let mut graph = GltfGraph::new();
        let mut primitive = Primitive::new(&mut graph);

        primitive.get_mut(&mut graph).mode = Mode::Lines;
        assert_eq!(primitive.get(&graph).mode, Mode::Lines);

        let indices = Accessor::new(&mut graph);
        primitive.set_indices(&mut graph, Some(&indices));
        assert_eq!(primitive.indices(&graph), Some(indices));

        let position = Accessor::new(&mut graph);
        primitive.set_attribute(&mut graph, &Semantic::Positions, Some(&position));
        assert_eq!(
            primitive.attribute(&graph, &Semantic::Positions),
            Some(position)
        );

        let normal = Accessor::new(&mut graph);
        primitive.set_attribute(&mut graph, &Semantic::Normals, Some(&normal));
        assert_eq!(
            primitive.attribute(&graph, &Semantic::Normals),
            Some(normal)
        );
        assert_eq!(primitive.attributes(&graph).len(), 2);

        primitive.set_attribute(&mut graph, &Semantic::Normals, None);
        assert_eq!(primitive.attribute(&graph, &Semantic::Normals), None);
        assert_eq!(primitive.attributes(&graph).len(), 1);
    }
}
