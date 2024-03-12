use petgraph::{graph::NodeIndex, stable_graph::EdgeReference, visit::EdgeRef, Direction};

use super::{Edge, Graph};

/// Helpers for using "Other" edges.
pub trait OtherEdgeHelpers: Copy + From<NodeIndex> + Into<NodeIndex> {
    fn find_other_edges<'a>(
        self,
        graph: &'a Graph,
        edge: &str,
    ) -> impl Iterator<Item = EdgeReference<'a, Edge>> {
        graph
            .edges_directed(self.into(), Direction::Outgoing)
            .filter(move |e| match e.weight() {
                Edge::Other(name) => name == edge,
                _ => false,
            })
    }

    fn find_properties<P: From<NodeIndex> + Into<NodeIndex>>(
        self,
        graph: &Graph,
        edge: &str,
    ) -> Vec<P> {
        let mut properties = self
            .find_other_edges(graph, edge)
            .map(|e| P::from(e.target()))
            .collect::<Vec<_>>();

        properties.sort_by_key(|p| {
            let index: NodeIndex = (*p).into();
            index.index()
        });

        properties
    }
    fn find_property<P: From<NodeIndex> + Into<NodeIndex>>(
        self,
        graph: &Graph,
        edge: &str,
    ) -> Option<P> {
        self.find_properties(graph, edge).pop()
    }

    fn set_property<P: Into<NodeIndex>>(
        self,
        graph: &mut Graph,
        edge: String,
        property: Option<P>,
    ) {
        let found = self
            .find_other_edges(graph, &edge)
            .map(|e| e.id())
            .collect::<Vec<_>>();

        for e in found {
            graph.remove_edge(e);
        }

        if let Some(property) = property {
            self.add_property(graph, edge, property);
        }
    }
    fn add_property<P: Into<NodeIndex>>(self, graph: &mut Graph, edge: String, property: P) {
        graph.add_edge(self.into(), property.into(), Edge::Other(edge));
    }
    fn remove_property<P: Copy + Into<NodeIndex>>(
        self,
        graph: &mut Graph,
        edge: &str,
        property: P,
    ) {
        let found = self
            .find_other_edges(graph, edge)
            .find(|e| e.target() == property.into());

        if let Some(edge) = found {
            graph.remove_edge(edge.id());
        }
    }
}
