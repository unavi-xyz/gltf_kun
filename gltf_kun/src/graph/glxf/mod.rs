use petgraph::graph::DiGraph;

#[derive(Debug)]
pub enum GlxfWeight {}

#[derive(Debug, PartialEq, Eq)]
pub enum GlxfEdge {}

pub type GlxfGraph = DiGraph<GlxfWeight, GlxfEdge>;
