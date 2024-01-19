use petgraph::{graph::NodeIndex, visit::EdgeRef, Direction};

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

    fn extension<T>(&self, graph: &Graph, name: &'static str) -> Option<T>
    where
        for<'a> T: From<&'a Vec<u8>>,
    {
        graph
            .edges_directed((*self).into(), Direction::Outgoing)
            .find(|edge| {
                if let Edge::Extension(n) = edge.weight() {
                    *n == name
                } else {
                    false
                }
            })
            .map(
                |edge| match graph.node_weight(edge.target()).expect("Weight not found") {
                    Weight::Other(bytes) => T::from(bytes),
                    _ => panic!("Incorrect weight type"),
                },
            )
    }

    fn add_extension<T>(&self, graph: &mut Graph, name: &'static str, value: T)
    where
        T: Into<Vec<u8>>,
    {
        let index = graph.add_node(Weight::Other(value.into()));
        graph.add_edge((*self).into(), index, Edge::Extension(name));
    }

    fn remove_extension(&self, graph: &mut Graph, name: &'static str) {
        let edge = graph
            .edges_directed((*self).into(), Direction::Outgoing)
            .find(|edge| {
                if let Edge::Extension(n) = edge.weight() {
                    *n == name
                } else {
                    false
                }
            })
            .map(|edge| edge.id());

        if let Some(edge) = edge {
            graph.remove_edge(edge);
        }
    }
}
