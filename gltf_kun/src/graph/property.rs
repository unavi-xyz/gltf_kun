use petgraph::{
    graph::{EdgeReference, NodeIndex},
    visit::EdgeRef,
    Direction,
};

use crate::extensions::Extension;

use super::{Edge, Graph, Weight};

/// A property is an object that can have extensions and extras.
pub trait Property: Copy + Into<NodeIndex> {
    fn extensions(&self, graph: &Graph) -> Vec<&str> {
        graph
            .edges_directed((*self).into(), Direction::Outgoing)
            .filter_map(|edge| {
                if let Edge::Extension(n) = edge.weight() {
                    Some(*n)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }
    fn get_extension<T: Extension<Self>>(&self, graph: &Graph) -> Option<T> {
        find_extension_edge((*self).into(), graph, T::name()).map(|edge| T::from(edge.target()))
    }
    fn add_extension<T>(&self, graph: &mut Graph, name: &'static str, value: T)
    where
        T: Into<Vec<u8>>,
    {
        let index = graph.add_node(Weight::Bytes(value.into()));
        graph.add_edge((*self).into(), index, Edge::Extension(name));
    }
    fn remove_extension(&self, graph: &mut Graph, name: &'static str) {
        let edge = find_extension_edge((*self).into(), graph, name).map(|edge| edge.id());

        if let Some(edge) = edge {
            graph.remove_edge(edge);
        }
    }
}

fn find_extension_edge<'a>(
    node: NodeIndex,
    graph: &'a Graph,
    name: &'static str,
) -> Option<EdgeReference<'a, Edge>> {
    graph
        .edges_directed(node, Direction::Outgoing)
        .find(|edge| {
            if let Edge::Extension(n) = edge.weight() {
                *n == name
            } else {
                false
            }
        })
}
