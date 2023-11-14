use petgraph::graph::NodeIndex;
use std::{cell::RefCell, rc::Rc};

mod accessor_array;
pub use accessor_array::*;

#[derive(Debug, Default, Clone)]
pub struct AccessorData {
    pub name: Option<String>,
    pub element_type: ElementType,
    pub normalized: bool,
    pub array: AccessorArray,
}

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

#[derive(Default, Debug, Clone)]
pub struct MeshData {
    pub name: Option<String>,
    pub primitives: Vec<PrimitiveData>,
    pub weights: Option<Vec<f32>>,
}

#[derive(Default, Debug, Clone)]
pub struct PrimitiveData {
    pub mode: PrimitiveMode,
}

#[derive(Debug, Clone, Default)]
pub enum PrimitiveMode {
    Points,
    Lines,
    LineLoop,
    LineStrip,
    #[default]
    Triangles,
    TriangleStrip,
    TriangleFan,
}

impl From<PrimitiveMode> for gltf::json::mesh::Mode {
    fn from(val: PrimitiveMode) -> Self {
        match val {
            PrimitiveMode::Points => gltf::json::mesh::Mode::Points,
            PrimitiveMode::Lines => gltf::json::mesh::Mode::Lines,
            PrimitiveMode::LineLoop => gltf::json::mesh::Mode::LineLoop,
            PrimitiveMode::LineStrip => gltf::json::mesh::Mode::LineStrip,
            PrimitiveMode::Triangles => gltf::json::mesh::Mode::Triangles,
            PrimitiveMode::TriangleStrip => gltf::json::mesh::Mode::TriangleStrip,
            PrimitiveMode::TriangleFan => gltf::json::mesh::Mode::TriangleFan,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AttributeData {
    pub semantic: AttributeSemantic,
}

#[derive(Debug, Clone)]
pub enum AttributeSemantic {
    Position,
    Normal,
    Tangent,
    TexCoord(u32),
    Color(u32),
    Joints(u32),
    Weights(u32),
}

impl From<AttributeSemantic> for gltf::Semantic {
    fn from(val: AttributeSemantic) -> Self {
        match val {
            AttributeSemantic::Position => gltf::Semantic::Positions,
            AttributeSemantic::Normal => gltf::Semantic::Normals,
            AttributeSemantic::Tangent => gltf::Semantic::Tangents,
            AttributeSemantic::TexCoord(index) => gltf::Semantic::TexCoords(index),
            AttributeSemantic::Color(index) => gltf::Semantic::Colors(index),
            AttributeSemantic::Joints(index) => gltf::Semantic::Joints(index),
            AttributeSemantic::Weights(index) => gltf::Semantic::Weights(index),
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct MaterialData {
    pub name: Option<String>,
}

pub enum GraphEdge {
    Accessor,
    Attribute,
    Child,
    Indices,
    Material,
    Mesh,
    Primitive,
}

#[derive(Debug, Clone)]
pub enum GraphData {
    Accessor(AccessorData),
    Attribute(AttributeData),
    Asset(AssetData),
    Material(MaterialData),
    Mesh(MeshData),
    Node(NodeData),
    Primitive(PrimitiveData),
    Scene(SceneData),
}

pub type GltfGraph = petgraph::graph::DiGraph<GraphData, GraphEdge>;

pub struct GraphNode {
    pub(crate) graph: Rc<RefCell<GltfGraph>>,
    pub(crate) index: NodeIndex,
}

impl GraphNode {
    pub fn new(graph: Rc<RefCell<GltfGraph>>, index: NodeIndex) -> Self {
        GraphNode { graph, index }
    }

    pub fn data(&self) -> GraphData {
        self.graph.borrow()[self.index].clone()
    }

    pub fn set_data(&mut self, data: GraphData) {
        self.graph.borrow_mut()[self.index] = data;
    }
}

pub trait NodeCover {
    type Data;
    fn new(graph: Rc<RefCell<GltfGraph>>, index: NodeIndex) -> Self;
    fn data(&self) -> Self::Data;
    fn set_data(&mut self, data: Self::Data);
}
