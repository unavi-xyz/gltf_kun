use petgraph::graph::NodeIndex;

use super::{Graph, Weight};

/// A node in the graph.
/// Provides methods for accessing the weight.
pub trait GraphNode<T>: Copy + Into<NodeIndex>
where
    for<'a> &'a T: TryFrom<&'a Weight>,
    for<'a> &'a mut T: TryFrom<&'a mut Weight>,
{
    fn get<'a>(&self, graph: &'a Graph) -> &'a T {
        self.try_get(graph).expect("Weight not found")
    }

    fn get_mut<'a>(&mut self, graph: &'a mut Graph) -> &'a mut T {
        self.try_get_mut(graph).expect("Weight not found")
    }

    fn try_get<'a>(&self, graph: &'a Graph) -> Option<&'a T> {
        graph.node_weight((*self).into())?.try_into().ok()
    }

    fn try_get_mut<'a>(&mut self, graph: &'a mut Graph) -> Option<&'a mut T> {
        graph.node_weight_mut((*self).into())?.try_into().ok()
    }
}
