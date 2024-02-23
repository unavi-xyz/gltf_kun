use petgraph::{
    stable_graph::{EdgeReference, NodeIndex},
    visit::EdgeRef,
    Direction,
};

use crate::extensions::Extension;

use super::{Edge, Graph};

/// A property is an object that can have extensions.
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
    fn get_extension<T: Extension>(&self, graph: &Graph) -> Option<T> {
        find_extension_edge((*self).into(), graph, T::name()).map(|edge| T::from(edge.target()))
    }
    fn add_extension<T: Extension>(&self, graph: &mut Graph, ext: T) {
        graph.add_edge((*self).into(), ext.into(), Edge::Extension(T::name()));
    }
    fn remove_extension(&self, graph: &mut Graph, name: &'static str) {
        let edge = find_extension_edge((*self).into(), graph, name).map(|edge| edge.id());

        if let Some(edge) = edge {
            graph.remove_edge(edge);
        }
    }
    fn create_extension<T: Extension>(&self, graph: &mut Graph) -> T {
        let ext = T::new(graph);
        self.add_extension(graph, ext);
        ext
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
