use petgraph::graph::NodeIndex;

use super::{Graph, Weight};

/// A serialized node in the graph.
/// Useful for storing arbitrary data in the graph.
pub trait ByteNode<T>: Copy + Into<NodeIndex>
where
    for<'a> T: From<&'a Vec<u8>>,
    for<'a> &'a T: Into<Vec<u8>>,
{
    /// Reads the weight from the graph.
    fn read(&self, graph: &Graph) -> T {
        match &graph[(*self).into()] {
            Weight::Bytes(bytes) => bytes.into(),
            _ => panic!("Incorrect weight type"),
        }
    }

    /// Writes the weight to the graph.
    fn write(&mut self, graph: &mut Graph, weight: &T) {
        graph[(*self).into()] = Weight::Bytes(weight.into());
    }
}
