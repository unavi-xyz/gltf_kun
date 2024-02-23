use petgraph::stable_graph::StableDiGraph;

#[derive(Clone, Debug)]
pub enum GlxfWeight {}

#[derive(Clone, Debug)]
pub enum GlxfEdge {}

pub type GlxfGraph = StableDiGraph<GlxfWeight, GlxfEdge>;
