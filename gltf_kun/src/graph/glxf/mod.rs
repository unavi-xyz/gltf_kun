use petgraph::graph::DiGraph;

#[derive(Debug)]
pub enum Weight {}

#[derive(Debug)]
pub enum Edge {}

pub type GlxfGraph = DiGraph<Weight, Edge>;
