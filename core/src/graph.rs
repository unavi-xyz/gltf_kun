use petgraph::graph::NodeIndex;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct AssetData {
    pub version: String,
    pub generator: Option<String>,
    pub extensions_used: Vec<String>,
    pub extensions_required: Vec<String>,
}

#[derive(Default, Debug, Clone)]
pub struct SceneData {
    pub name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct NodeData {
    pub name: Option<String>,
    pub translation: [f32; 3],
    pub rotation: [f32; 4],
    pub scale: [f32; 3],
}

impl Default for NodeData {
    fn default() -> Self {
        NodeData {
            name: None,
            translation: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0, 1.0],
            scale: [1.0, 1.0, 1.0],
        }
    }
}

#[derive(Debug, Clone)]
pub struct MeshData {
    pub name: Option<String>,
}

pub enum GraphEdge {
    Child,
    Mesh,
    Material,
}

#[derive(Debug, Clone)]
pub enum GraphData {
    Asset(AssetData),
    Mesh(MeshData),
    Node(NodeData),
    Scene(SceneData),
}

pub type GltfGraph = petgraph::graph::DiGraph<GraphData, GraphEdge>;

pub struct GraphNode {
    pub(crate) graph: Arc<Mutex<GltfGraph>>,
    pub(crate) index: NodeIndex,
}

impl GraphNode {
    pub fn new(graph: Arc<Mutex<GltfGraph>>, index: NodeIndex) -> Self {
        GraphNode { graph, index }
    }

    pub fn data(&self) -> GraphData {
        let graph = self.graph.lock().unwrap();
        graph[self.index].clone()
    }

    pub fn set_data(&mut self, data: GraphData) {
        let mut graph = self.graph.lock().unwrap();
        graph[self.index] = data;
    }
}

pub trait NodeCover {
    type Data;
    fn new(graph: Arc<Mutex<GltfGraph>>, index: NodeIndex) -> Self;
    fn data(&self) -> Self::Data;
    fn set_data(&mut self, data: Self::Data);
}
