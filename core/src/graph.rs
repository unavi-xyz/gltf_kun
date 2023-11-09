pub struct AssetData {
    pub version: String,
    pub generator: Option<String>,
    pub extensions_used: Vec<String>,
    pub extensions_required: Vec<String>,
}

pub struct MeshData {
    pub name: Option<String>,
}

#[derive(Debug)]
pub struct NodeData {
    pub name: Option<String>,
    pub translation: [f32; 3],
    pub rotation: [f32; 4],
    pub scale: [f32; 3],
}

pub struct SceneData {
    pub name: Option<String>,
}

pub enum GraphNode {
    Asset(AssetData),
    Mesh(MeshData),
    Node(NodeData),
    Scene(SceneData),
}

pub enum GraphEdge {
    Parent,
    Child,
}

pub type GltfGraph = petgraph::graph::DiGraph<GraphNode, GraphEdge>;
