use petgraph::{graph::NodeIndex, visit::EdgeRef};

use crate::graph::{Edge, Graph, GraphNodeEdges, GraphNodeWeight, Property, Weight};

use super::{accessor::Accessor, material::Material, GltfEdge, GltfWeight};

pub use gltf::json::mesh::{Mode, Semantic};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PrimitiveEdge {
    Attribute(Semantic),
    Indices,
    Material,
}

impl<'a> TryFrom<&'a Edge> for &'a PrimitiveEdge {
    type Error = ();
    fn try_from(value: &'a Edge) -> Result<Self, Self::Error> {
        match value {
            Edge::Gltf(GltfEdge::Primitive(edge)) => Ok(edge),
            _ => Err(()),
        }
    }
}

impl From<PrimitiveEdge> for Edge {
    fn from(edge: PrimitiveEdge) -> Self {
        Self::Gltf(GltfEdge::Primitive(edge))
    }
}

#[derive(Clone, Debug)]
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

impl From<PrimitiveWeight> for Weight {
    fn from(weight: PrimitiveWeight) -> Self {
        Self::Gltf(GltfWeight::Primitive(weight))
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

impl GraphNodeWeight<PrimitiveWeight> for Primitive {}
impl GraphNodeEdges<PrimitiveEdge> for Primitive {}
impl Property for Primitive {}

impl Primitive {
    pub fn material(&self, graph: &Graph) -> Option<Material> {
        self.find_edge_target(graph, &PrimitiveEdge::Material)
    }
    pub fn set_material(&self, graph: &mut Graph, material: Option<Material>) {
        self.set_edge_target(graph, PrimitiveEdge::Material, material);
    }

    pub fn indices(&self, graph: &Graph) -> Option<Accessor> {
        self.find_edge_target(graph, &PrimitiveEdge::Indices)
    }
    pub fn set_indices(&self, graph: &mut Graph, indices: Option<Accessor>) {
        self.set_edge_target(graph, PrimitiveEdge::Indices, indices);
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
        self.find_edge_target(graph, &PrimitiveEdge::Attribute(semantic.clone()))
    }
    pub fn set_attribute(
        &self,
        graph: &mut Graph,
        semantic: &Semantic,
        accessor: Option<Accessor>,
    ) {
        self.set_edge_target(graph, PrimitiveEdge::Attribute(semantic.clone()), accessor);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn indices() {
        let mut graph = Graph::default();

        let primitive = Primitive::new(&mut graph);
        let indices = Accessor::new(&mut graph);

        primitive.set_indices(&mut graph, Some(indices));
        assert_eq!(primitive.indices(&graph), Some(indices));

        primitive.set_indices(&mut graph, None);
        assert_eq!(primitive.indices(&graph), None);
    }

    #[test]
    fn attributes() {
        let mut graph = Graph::default();

        let primitive = Primitive::new(&mut graph);
        let position = Accessor::new(&mut graph);
        let normal = Accessor::new(&mut graph);

        primitive.set_attribute(&mut graph, &Semantic::Positions, Some(position));
        assert_eq!(
            primitive.attribute(&graph, &Semantic::Positions),
            Some(position)
        );

        primitive.set_attribute(&mut graph, &Semantic::Normals, Some(normal));
        assert_eq!(
            primitive.attribute(&graph, &Semantic::Normals),
            Some(normal)
        );
        assert_eq!(primitive.attributes(&graph).len(), 2);

        primitive.set_attribute(&mut graph, &Semantic::Normals, None);
        assert_eq!(primitive.attribute(&graph, &Semantic::Normals), None);
        assert_eq!(primitive.attributes(&graph).len(), 1);

        primitive.set_attribute(&mut graph, &Semantic::Positions, None);
        assert!(primitive.attribute(&graph, &Semantic::Positions).is_none());
    }

    #[test]
    fn material() {
        let mut graph = Graph::default();

        let primitive = Primitive::new(&mut graph);
        let material = Material::new(&mut graph);

        primitive.set_material(&mut graph, Some(material));
        assert_eq!(primitive.material(&graph), Some(material));

        primitive.set_material(&mut graph, None);
        assert!(primitive.material(&graph).is_none());
    }
}
