use petgraph::graph::DiGraph;

use crate::extension::Extension;

pub type GltfNode = ();
pub type GltfEdge = ();
pub type GltfGraph = DiGraph<GltfNode, GltfEdge>;

pub trait Property {
    fn name(&self) -> &str;
    fn extensions(&self) -> &[impl Extension];
    fn extras(&self) -> &serde_json::Value;

    fn set_name(&mut self, name: String);
    fn set_extensions(&mut self, extensions: Vec<impl Extension>);
    fn set_extras(&mut self, extras: serde_json::Value);
}
