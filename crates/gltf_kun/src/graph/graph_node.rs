use petgraph::{
    stable_graph::{EdgeReference, NodeIndex},
    visit::EdgeRef,
    Direction,
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

    fn take(&mut self, graph: &mut Graph) -> W {
        let weight = self.get_mut(graph);
        std::mem::take(weight)
    }
}

pub trait GraphNodeEdges: Copy + Into<NodeIndex> {
    fn find_edges<'a, E>(
        &'a self,
        graph: &'a Graph,
        edge: &E,
        direction: Direction,
    ) -> impl Iterator<Item = EdgeReference<Edge>>
    where
        for<'b> &'b E: TryFrom<&'b Edge> + PartialEq,
        Edge: From<E>,
    {
        graph
            .edges_directed((*self).into(), direction)
            .filter(|edge_ref| {
                let e: &E = match edge_ref.weight().try_into() {
                    Ok(e) => e,
                    Err(_) => return false,
                };

                e == edge
            })
    }
    fn find_edge<'a, E>(
        &'a self,
        graph: &'a Graph,
        edge: &E,
        direction: Direction,
    ) -> Option<EdgeReference<Edge>>
    where
        for<'b> &'b E: TryFrom<&'b Edge> + PartialEq,
        Edge: From<E>,
    {
        self.find_edges(graph, edge, direction).next()
    }

    fn incoming_sources_iter<'a, E, T: From<NodeIndex>>(
        &'a self,
        graph: &'a Graph,
        edge: &'a E,
    ) -> impl Iterator<Item = T>
    where
        for<'b> &'b E: TryFrom<&'b Edge> + PartialEq,
        Edge: From<E>,
    {
        self.find_edges(graph, edge, Direction::Incoming)
            .map(|edge_ref| edge_ref.source().into())
    }
    fn outgoing_targets_iter<'a, E, T: From<NodeIndex>>(
        &'a self,
        graph: &'a Graph,
        edge: &'a E,
    ) -> impl Iterator<Item = T>
    where
        for<'b> &'b E: TryFrom<&'b Edge> + PartialEq,
        Edge: From<E>,
    {
        self.find_edges(graph, edge, Direction::Outgoing)
            .map(|edge_ref| edge_ref.target().into())
    }

    fn find_edge_source<E, T: From<NodeIndex>>(&self, graph: &Graph, edge: &E) -> Option<T>
    where
        for<'b> &'b E: TryFrom<&'b Edge> + PartialEq,
        Edge: From<E>,
    {
        self.incoming_sources_iter(graph, edge).next()
    }
    fn find_edge_target<E, T: From<NodeIndex>>(&self, graph: &Graph, edge: &E) -> Option<T>
    where
        for<'b> &'b E: TryFrom<&'b Edge> + PartialEq,
        Edge: From<E>,
    {
        self.outgoing_targets_iter(graph, edge).next()
    }
    fn set_edge_target<E, T: Into<NodeIndex>>(&self, graph: &mut Graph, edge: E, target: Option<T>)
    where
        for<'b> &'b E: TryFrom<&'b Edge> + PartialEq,
        Edge: From<E>,
    {
        let found_edge = self
            .find_edge(graph, &edge, Direction::Outgoing)
            .map(|edge_ref| edge_ref.id());

        if let Some(found_edge) = found_edge {
            graph.remove_edge(found_edge);
        }

        if let Some(target) = target {
            graph.add_edge((*self).into(), target.into(), edge.into());
        }
    }

    fn edge_sources<'a, E, T: From<NodeIndex> + Ord>(
        &'a self,
        graph: &'a Graph,
        edge: &'a E,
    ) -> Vec<T>
    where
        for<'b> &'b E: TryFrom<&'b Edge> + PartialEq,
        Edge: From<E>,
    {
        let mut vec = self.incoming_sources_iter(graph, edge).collect::<Vec<_>>();
        vec.sort();
        vec
    }
    fn edge_targets<'a, E, T: From<NodeIndex> + Ord>(
        &'a self,
        graph: &'a Graph,
        edge: &'a E,
    ) -> Vec<T>
    where
        for<'b> &'b E: TryFrom<&'b Edge> + PartialEq,
        Edge: From<E>,
    {
        let mut vec = self.outgoing_targets_iter(graph, edge).collect::<Vec<_>>();
        vec.sort();
        vec
    }

    fn add_edge_target<E, T: Into<NodeIndex>>(&self, graph: &mut Graph, edge: E, target: T)
    where
        for<'b> &'b E: TryFrom<&'b Edge> + PartialEq,
        Edge: From<E>,
    {
        graph.add_edge((*self).into(), target.into(), edge.into());
    }
    fn remove_edge_target<E, T: Into<NodeIndex>>(&self, graph: &mut Graph, edge: E, target: T)
    where
        for<'b> &'b E: TryFrom<&'b Edge> + PartialEq,
        Edge: From<E>,
    {
        let target_idx: NodeIndex = target.into();

        let found_edge = self
            .find_edges(graph, &edge, Direction::Outgoing)
            .find(|edge_ref| edge_ref.target() == target_idx);

        if let Some(found_edge) = found_edge {
            graph.remove_edge(found_edge.id());
        }
    }
    fn create_edge_target<W, E, T: GraphNodeWeight<W>>(&self, graph: &mut Graph, edge: E) -> T
    where
        Edge: From<E>,
        W: Default + Into<Weight>,
        for<'a> &'a W: TryFrom<&'a Weight>,
        for<'a> &'a mut W: TryFrom<&'a mut Weight>,
        for<'b> &'b E: TryFrom<&'b Edge> + PartialEq,
    {
        let target = T::new(graph);
        self.add_edge_target(graph, edge, target);
        target
    }
}
