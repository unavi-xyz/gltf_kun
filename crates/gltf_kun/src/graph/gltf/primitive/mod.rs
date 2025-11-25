use petgraph::{Direction, graph::NodeIndex, visit::EdgeRef};

use crate::graph::{Edge, Extensions, Graph, GraphNodeEdges, GraphNodeWeight, Weight};

use super::{GltfEdge, GltfWeight, Mesh, accessor::Accessor, material::Material, mesh::MeshEdge};

pub use gltf::json::mesh::{Mode, Semantic};

mod morph_target;

pub use morph_target::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PrimitiveEdge {
    Attribute(Semantic),
    Indices,
    Material,
    MorphTarget(usize),
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
impl GraphNodeEdges for Primitive {}
impl Extensions for Primitive {}

impl Primitive {
    pub fn attributes(&self, graph: &Graph) -> Vec<(Semantic, Accessor)> {
        graph
            .edges_directed(self.0, Direction::Outgoing)
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
    pub fn attribute(&self, graph: &Graph, semantic: Semantic) -> Option<Accessor> {
        self.find_edge_target(graph, &PrimitiveEdge::Attribute(semantic))
    }
    pub fn set_attribute(&self, graph: &mut Graph, semantic: Semantic, accessor: Option<Accessor>) {
        self.set_edge_target(graph, PrimitiveEdge::Attribute(semantic), accessor);
    }

    pub fn indices(&self, graph: &Graph) -> Option<Accessor> {
        self.find_edge_target(graph, &PrimitiveEdge::Indices)
    }
    pub fn set_indices(&self, graph: &mut Graph, indices: Option<Accessor>) {
        self.set_edge_target(graph, PrimitiveEdge::Indices, indices);
    }

    pub fn material(&self, graph: &Graph) -> Option<Material> {
        self.find_edge_target(graph, &PrimitiveEdge::Material)
    }
    pub fn set_material(&self, graph: &mut Graph, material: Option<Material>) {
        self.set_edge_target(graph, PrimitiveEdge::Material, material);
    }

    pub fn morph_targets(&self, graph: &Graph) -> Vec<MorphTarget> {
        let mut morph_targets = graph
            .edges_directed(self.0, Direction::Outgoing)
            .filter_map(|edge_ref| {
                let edge: &PrimitiveEdge = match edge_ref.weight().try_into() {
                    Ok(edge) => edge,
                    Err(()) => return None,
                };

                match edge {
                    PrimitiveEdge::MorphTarget(i) => {
                        Some((i, MorphTarget::from(edge_ref.target())))
                    }
                    _ => None,
                }
            })
            .collect::<Vec<_>>();

        morph_targets.sort_by_key(|(i, _)| *i);

        morph_targets.into_iter().map(|(_, node)| node).collect()
    }
    pub fn add_morph_target(&self, graph: &mut Graph, node: &MorphTarget, index: usize) {
        self.add_edge_target(graph, PrimitiveEdge::MorphTarget(index), *node);
    }
    pub fn remove_morph_target(&self, graph: &mut Graph, node: &MorphTarget) {
        let target_idx: NodeIndex = (*node).into();

        let found_edge = graph
            .edges_directed(self.0, Direction::Outgoing)
            .filter(|edge_ref| {
                let edge: &PrimitiveEdge = match edge_ref.weight().try_into() {
                    Ok(edge) => edge,
                    Err(()) => return false,
                };

                matches!(edge, PrimitiveEdge::MorphTarget(_))
            })
            .find(|edge_ref| edge_ref.target() == target_idx);

        if let Some(found_edge) = found_edge {
            graph.remove_edge(found_edge.id());
        }
    }
    pub fn create_morph_target(&self, graph: &mut Graph, index: usize) -> MorphTarget {
        let idx = graph.add_node(Weight::Gltf(GltfWeight::MorphTarget));
        self.add_edge_target(graph, PrimitiveEdge::MorphTarget(index), idx);
        MorphTarget(idx)
    }

    pub fn mesh(&self, graph: &Graph) -> Option<Mesh> {
        self.find_edge_source(graph, &MeshEdge::Primitive)
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

        primitive.set_attribute(&mut graph, Semantic::Positions, Some(position));
        assert_eq!(
            primitive.attribute(&graph, Semantic::Positions),
            Some(position)
        );

        primitive.set_attribute(&mut graph, Semantic::Normals, Some(normal));
        assert_eq!(primitive.attribute(&graph, Semantic::Normals), Some(normal));
        assert_eq!(primitive.attributes(&graph).len(), 2);

        primitive.set_attribute(&mut graph, Semantic::Normals, None);
        assert_eq!(primitive.attribute(&graph, Semantic::Normals), None);
        assert_eq!(primitive.attributes(&graph).len(), 1);

        primitive.set_attribute(&mut graph, Semantic::Positions, None);
        assert!(primitive.attribute(&graph, Semantic::Positions).is_none());
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

    #[test]
    fn morph_targets() {
        let mut graph = Graph::default();

        let primitive = Primitive::new(&mut graph);
        let target1 = MorphTarget::new(&mut graph);
        let target2 = MorphTarget::new(&mut graph);

        primitive.add_morph_target(&mut graph, &target1, 0);
        primitive.add_morph_target(&mut graph, &target2, 1);

        assert_eq!(primitive.morph_targets(&graph), vec![target1, target2]);

        primitive.remove_morph_target(&mut graph, &target1);
        assert_eq!(primitive.morph_targets(&graph), vec![target2]);

        let target3 = primitive.create_morph_target(&mut graph, 2);
        assert_eq!(primitive.morph_targets(&graph), vec![target2, target3]);
    }

    #[test]
    fn mesh() {
        let mut graph = Graph::default();

        let mesh = Mesh::new(&mut graph);
        let primitive = mesh.create_primitive(&mut graph);
        assert_eq!(primitive.mesh(&graph), Some(mesh));

        mesh.remove_primitive(&mut graph, &primitive);
        assert!(primitive.mesh(&graph).is_none());
    }
}
