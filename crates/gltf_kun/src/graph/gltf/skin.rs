use petgraph::{Direction, graph::NodeIndex, visit::EdgeRef};

use crate::graph::{Edge, Extensions, Graph, GraphNodeEdges, GraphNodeWeight, Weight};

use super::{Accessor, GltfEdge, GltfWeight, Node};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SkinEdge {
    InverseBindMatrices,
    Joint(usize),
    Skeleton,
}

impl<'a> TryFrom<&'a Edge> for &'a SkinEdge {
    type Error = ();
    fn try_from(value: &'a Edge) -> Result<Self, Self::Error> {
        match value {
            Edge::Gltf(GltfEdge::Skin(edge)) => Ok(edge),
            _ => Err(()),
        }
    }
}

impl From<SkinEdge> for Edge {
    fn from(edge: SkinEdge) -> Self {
        Self::Gltf(GltfEdge::Skin(edge))
    }
}

#[derive(Clone, Debug, Default)]
pub struct SkinWeight {
    pub extras: gltf::json::Extras,
    pub name: Option<String>,
}

impl From<SkinWeight> for Weight {
    fn from(weight: SkinWeight) -> Self {
        Self::Gltf(GltfWeight::Skin(weight))
    }
}

impl<'a> TryFrom<&'a Weight> for &'a SkinWeight {
    type Error = ();
    fn try_from(value: &'a Weight) -> Result<Self, Self::Error> {
        match value {
            Weight::Gltf(GltfWeight::Skin(weight)) => Ok(weight),
            _ => Err(()),
        }
    }
}

impl<'a> TryFrom<&'a mut Weight> for &'a mut SkinWeight {
    type Error = ();
    fn try_from(value: &'a mut Weight) -> Result<Self, Self::Error> {
        match value {
            Weight::Gltf(GltfWeight::Skin(weight)) => Ok(weight),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Skin(pub NodeIndex);

impl From<NodeIndex> for Skin {
    fn from(index: NodeIndex) -> Self {
        Self(index)
    }
}

impl From<Skin> for NodeIndex {
    fn from(node: Skin) -> Self {
        node.0
    }
}

impl GraphNodeWeight<SkinWeight> for Skin {}
impl GraphNodeEdges for Skin {}
impl Extensions for Skin {}

impl Skin {
    pub fn inverse_bind_matrices(&self, graph: &Graph) -> Option<Accessor> {
        self.find_edge_target(graph, &SkinEdge::InverseBindMatrices)
    }
    pub fn set_inverse_bind_matrices(&self, graph: &mut Graph, accessor: Option<Accessor>) {
        self.set_edge_target(graph, SkinEdge::InverseBindMatrices, accessor);
    }

    pub fn joints(&self, graph: &Graph) -> Vec<Node> {
        let mut joints = graph
            .edges_directed(self.0, Direction::Outgoing)
            .filter_map(|edge_ref| {
                let edge: &SkinEdge = match edge_ref.weight().try_into() {
                    Ok(edge) => edge,
                    Err(()) => return None,
                };

                match edge {
                    SkinEdge::Joint(i) => Some((i, Node::from(edge_ref.target()))),
                    _ => None,
                }
            })
            .collect::<Vec<_>>();

        joints.sort_by_key(|(i, _)| *i);

        joints.into_iter().map(|(_, node)| node).collect()
    }
    pub fn add_joint(&self, graph: &mut Graph, node: &Node, index: usize) {
        self.add_edge_target(graph, SkinEdge::Joint(index), *node);
    }
    pub fn remove_joint(&self, graph: &mut Graph, node: &Node) {
        let target_idx: NodeIndex = (*node).into();

        let found_edge = graph
            .edges_directed(self.0, Direction::Outgoing)
            .filter(|edge_ref| {
                let edge: &SkinEdge = match edge_ref.weight().try_into() {
                    Ok(edge) => edge,
                    Err(()) => return false,
                };

                matches!(edge, SkinEdge::Joint(_))
            })
            .find(|edge_ref| edge_ref.target() == target_idx);

        if let Some(found_edge) = found_edge {
            graph.remove_edge(found_edge.id());
        }
    }
    pub fn create_joint(&self, graph: &mut Graph, index: usize) -> Node {
        self.create_edge_target(graph, SkinEdge::Joint(index))
    }

    pub fn skeleton(&self, graph: &Graph) -> Option<Node> {
        self.find_edge_target(graph, &SkinEdge::Skeleton)
    }
    pub fn set_skeleton(&self, graph: &mut Graph, node: Option<Node>) {
        self.set_edge_target(graph, SkinEdge::Skeleton, node);
    }
}

#[cfg(test)]
mod tests {
    use crate::graph::gltf::GltfDocument;

    use super::*;

    #[test]
    fn inverse_bind_matrices() {
        let mut graph = Graph::default();
        let doc = GltfDocument::new(&mut graph);

        let skin = doc.create_skin(&mut graph);
        let accessor = doc.create_accessor(&mut graph);

        skin.set_inverse_bind_matrices(&mut graph, Some(accessor));
        assert_eq!(skin.inverse_bind_matrices(&graph), Some(accessor));

        skin.set_inverse_bind_matrices(&mut graph, None);
        assert_eq!(skin.inverse_bind_matrices(&graph), None);
    }

    #[test]
    fn joints() {
        let mut graph = Graph::default();
        let doc = GltfDocument::new(&mut graph);

        let skin = doc.create_skin(&mut graph);
        let node_1 = doc.create_node(&mut graph);
        let node_2 = doc.create_node(&mut graph);

        skin.add_joint(&mut graph, &node_1, 0);
        assert_eq!(skin.joints(&graph), vec![node_1]);

        skin.add_joint(&mut graph, &node_2, 1);
        assert_eq!(skin.joints(&graph), vec![node_1, node_2]);

        skin.remove_joint(&mut graph, &node_1);
        assert_eq!(skin.joints(&graph), vec![node_2]);
    }

    #[test]
    fn skeleton() {
        let mut graph = Graph::default();
        let doc = GltfDocument::new(&mut graph);

        let skin = doc.create_skin(&mut graph);
        let node = doc.create_node(&mut graph);

        skin.set_skeleton(&mut graph, Some(node));
        assert_eq!(skin.skeleton(&graph), Some(node));

        skin.set_skeleton(&mut graph, None);
        assert_eq!(skin.skeleton(&graph), None);
    }
}
