use petgraph::graph::NodeIndex;

use super::{Graph, Weight};

pub trait GraphNode<W>: Copy + Into<NodeIndex>
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
