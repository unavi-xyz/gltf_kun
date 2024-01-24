use petgraph::{
    graph::{EdgeReference, NodeIndex},
    visit::EdgeRef,
};

use super::{Edge, Graph, Weight};

/// A node in the graph with a weight.
pub trait GraphNodeWeight<W>: Copy + Into<NodeIndex>
where
    for<'a> &'a W: TryFrom<&'a Weight>,
    for<'a> &'a mut W: TryFrom<&'a mut Weight>,
{
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

                e == &edge
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
}
