use petgraph::{
    graph::{EdgeReference, NodeIndex},
    visit::EdgeRef,
};

use super::{Edge, Graph, Weight};

/// A node in the graph with a weight.
pub trait GraphNodeWeight<W>: Copy + Into<NodeIndex> + From<NodeIndex>
where
    W: Default + Into<Weight>,
    for<'a> &'a W: TryFrom<&'a Weight>,
    for<'a> &'a mut W: TryFrom<&'a mut Weight>,
{
    fn new(graph: &mut Graph) -> Self {
        let index = graph.add_node(W::default().into());
        Self::from(index)
    }

    fn get<'a>(&self, graph: &'a Graph) -> &'a W {
        self.try_get(graph).expect("Weight not found")
    }
    fn get_mut<'a>(&mut self, graph: &'a mut Graph) -> &'a mut W {
        self.try_get_mut(graph).expect("Weight not found")
    }
    fn try_get<'a>(&self, graph: &'a Graph) -> Option<&'a W> {
        graph.node_weight((*self).into())?.try_into().ok()
    }
    fn try_get_mut<'a>(&mut self, graph: &'a mut Graph) -> Option<&'a mut W> {
        graph.node_weight_mut((*self).into())?.try_into().ok()
    }
}

/// A node in the graph with outgoing edges.
pub trait GraphNodeEdges<E>: Copy + Into<NodeIndex>
where
    for<'a> &'a E: TryFrom<&'a Edge> + PartialEq,
    Edge: From<E>,
{
    fn find_edge<'a>(&'a self, graph: &'a Graph, edge: &E) -> Option<EdgeReference<Edge>> {
        graph
            .edges_directed((*self).into(), petgraph::Direction::Outgoing)
            .find(|edge_ref| {
                let e: &E = match edge_ref.weight().try_into() {
                    Ok(e) => e,
                    Err(_) => return false,
                };

                e == edge
            })
    }
    fn find_edge_target<T: From<NodeIndex>>(&self, graph: &Graph, edge: &E) -> Option<T> {
        self.find_edge(graph, edge)
            .map(|edge_ref| edge_ref.target().into())
    }
    fn set_edge_target<T: Into<NodeIndex>>(&self, graph: &mut Graph, edge: E, target: Option<T>) {
        let found_edge = self.find_edge(graph, &edge).map(|edge_ref| edge_ref.id());

        if let Some(found_edge) = found_edge {
            graph.remove_edge(found_edge);
        }

        if let Some(target) = target {
            graph.add_edge((*self).into(), target.into(), edge.into());
        }
    }

    fn find_edges<'a>(
        &'a self,
        graph: &'a Graph,
        edge: &E,
    ) -> impl Iterator<Item = EdgeReference<Edge>> {
        graph
            .edges_directed((*self).into(), petgraph::Direction::Outgoing)
            .filter(|edge_ref| {
                let e: &E = match edge_ref.weight().try_into() {
                    Ok(e) => e,
                    Err(_) => return false,
                };

                e == edge
            })
    }
    fn edge_targets_iter<'a, T: From<NodeIndex>>(
        &'a self,
        graph: &'a Graph,
        edge: &'a E,
    ) -> impl Iterator<Item = T> {
        self.find_edges(graph, edge)
            .map(|edge_ref| edge_ref.target().into())
    }
    fn edge_targets<'a, T: From<NodeIndex> + Ord>(
        &'a self,
        graph: &'a Graph,
        edge: &'a E,
    ) -> Vec<T> {
        let mut vec = self.edge_targets_iter(graph, edge).collect::<Vec<_>>();
        vec.sort();
        vec
    }
    fn add_edge_target<T: Into<NodeIndex>>(&self, graph: &mut Graph, edge: E, target: T) {
        graph.add_edge((*self).into(), target.into(), edge.into());
    }
    fn remove_edge_target<T: Into<NodeIndex>>(&self, graph: &mut Graph, edge: E, target: T) {
        let target_idx: NodeIndex = target.into();

        let found_edge = self
            .find_edges(graph, &edge)
            .find(|edge_ref| edge_ref.target() == target_idx);

        if let Some(found_edge) = found_edge {
            graph.remove_edge(found_edge.id());
        }
    }
    fn create_edge_target<W, T: GraphNodeWeight<W>>(&self, graph: &mut Graph, edge: E) -> T
    where
        W: Default + Into<Weight>,
        for<'a> &'a W: TryFrom<&'a Weight>,
        for<'a> &'a mut W: TryFrom<&'a mut Weight>,
    {
        let target = T::new(graph);
        self.add_edge_target(graph, edge, target);
        target
    }
}
