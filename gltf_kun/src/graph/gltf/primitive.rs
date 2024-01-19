use petgraph::{graph::NodeIndex, visit::EdgeRef};

use crate::graph::{Edge, Graph, GraphNode, Property, Weight};

use super::{accessor::Accessor, GltfEdge, GltfWeight};

pub use gltf::json::mesh::{Mode, Semantic};

#[derive(Debug, PartialEq, Eq)]
pub enum PrimitiveEdge {
    Indices,
    Attribute(Semantic),
}

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

impl<'a> TryFrom<&'a Weight> for &'a PrimitiveWeight {
    type Error = ();
    fn try_from(value: &'a Weight) -> Result<Self, Self::Error> {
        match value {
            Weight::Gltf(GltfWeight::Primitive(weight)) => Ok(weight),
            _ => Err(()),
        }
    }
}

impl<'a> TryFrom<&'a mut Weight> for &'a mut PrimitiveWeight {
    type Error = ();
    fn try_from(value: &'a mut Weight) -> Result<Self, Self::Error> {
        match value {
            Weight::Gltf(GltfWeight::Primitive(weight)) => Ok(weight),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Primitive(pub NodeIndex);

impl From<NodeIndex> for Primitive {
    fn from(index: NodeIndex) -> Self {
        Self(index)
    }
}

impl From<Primitive> for NodeIndex {
    fn from(primitive: Primitive) -> Self {
        primitive.0
    }
}

impl GraphNode<PrimitiveWeight> for Primitive {}
impl Property for Primitive {}

impl Primitive {
    pub fn new(graph: &mut Graph) -> Self {
        let index = graph.add_node(Weight::Gltf(GltfWeight::Primitive(
            PrimitiveWeight::default(),
        )));
        Self(index)
    }

    pub fn indices(&self, graph: &Graph) -> Option<Accessor> {
        graph
            .edges_directed(self.0, petgraph::Direction::Outgoing)
            .find_map(|edge| {
                if let Edge::Gltf(GltfEdge::Primitive(PrimitiveEdge::Indices)) = edge.weight() {
                    Some(Accessor(edge.target()))
                } else {
                    None
                }
            })
    }
    pub fn set_indices(&self, graph: &mut Graph, indices: Option<&Accessor>) {
        let edge = graph
            .edges_directed(self.0, petgraph::Direction::Outgoing)
            .find(|edge| {
                matches!(
                    edge.weight(),
                    Edge::Gltf(GltfEdge::Primitive(PrimitiveEdge::Indices))
                )
            })
            .map(|edge| edge.id());

        if let Some(edge) = edge {
            graph.remove_edge(edge);
        }

        if let Some(indices) = indices {
            graph.add_edge(
                self.0,
                indices.0,
                Edge::Gltf(GltfEdge::Primitive(PrimitiveEdge::Indices)),
            );
        }
    }

    pub fn attributes(&self, graph: &Graph) -> Vec<(Semantic, Accessor)> {
        graph
            .edges_directed(self.0, petgraph::Direction::Outgoing)
            .filter_map(|edge| {
                if let Edge::Gltf(GltfEdge::Primitive(PrimitiveEdge::Attribute(semantic))) =
                    edge.weight()
                {
                    Some((semantic.clone(), Accessor(edge.target())))
                } else {
                    None
                }
            })
            .collect()
    }
    pub fn attribute(&self, graph: &Graph, semantic: &Semantic) -> Option<Accessor> {
        graph
            .edges_directed(self.0, petgraph::Direction::Outgoing)
            .find_map(|edge| {
                if let Edge::Gltf(GltfEdge::Primitive(PrimitiveEdge::Attribute(edge_semantic))) =
                    edge.weight()
                {
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
        &self,
        graph: &mut Graph,
        semantic: &Semantic,
        accessor: Option<&Accessor>,
    ) {
        if let Some(accessor) = accessor {
            graph.add_edge(
                self.0,
                accessor.0,
                Edge::Gltf(GltfEdge::Primitive(PrimitiveEdge::Attribute(
                    semantic.clone(),
                ))),
            );
        } else if let Some(edge) = graph
            .edges_directed(self.0, petgraph::Direction::Outgoing)
            .find(|edge| {
                if let Edge::Gltf(GltfEdge::Primitive(PrimitiveEdge::Attribute(edge_semantic))) =
                    edge.weight()
                {
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
        let mut graph = Graph::new();
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
