use petgraph::graph::NodeIndex;
use std::{cell::RefCell, rc::Rc};

#[derive(Default, Debug, Clone)]
pub enum ElementType {
    #[default]
    Scalar,
    Vec2,
    Vec3,
    Vec4,
    Mat2,
    Mat3,
    Mat4,
}

impl ElementType {
    pub fn size(&self) -> usize {
        match self {
            ElementType::Scalar => 1,
            ElementType::Vec2 => 2,
            ElementType::Vec3 => 3,
            ElementType::Vec4 => 4,
            ElementType::Mat2 => 4,
            ElementType::Mat3 => 9,
            ElementType::Mat4 => 16,
        }
    }
}

impl Into<gltf::json::accessor::Type> for ElementType {
    fn into(self) -> gltf::json::accessor::Type {
        match self {
            ElementType::Scalar => gltf::json::accessor::Type::Scalar,
            ElementType::Vec2 => gltf::json::accessor::Type::Vec2,
            ElementType::Vec3 => gltf::json::accessor::Type::Vec3,
            ElementType::Vec4 => gltf::json::accessor::Type::Vec4,
            ElementType::Mat2 => gltf::json::accessor::Type::Mat2,
            ElementType::Mat3 => gltf::json::accessor::Type::Mat3,
            ElementType::Mat4 => gltf::json::accessor::Type::Mat4,
        }
    }
}

#[derive(Debug, Clone)]
pub enum AccessorArray {
    I8(Box<[i8]>),
    U8(Box<[u8]>),
    I16(Box<[i16]>),
    U16(Box<[u16]>),
    U32(Box<[u32]>),
    F32(Box<[f32]>),
}

impl AccessorArray {
    pub fn len(&self) -> usize {
        match self {
            AccessorArray::I8(array) => array.len(),
            AccessorArray::U8(array) => array.len(),
            AccessorArray::I16(array) => array.len(),
            AccessorArray::U16(array) => array.len(),
            AccessorArray::U32(array) => array.len(),
            AccessorArray::F32(array) => array.len(),
        }
    }
}

impl Default for AccessorArray {
    fn default() -> Self {
        AccessorArray::F32(Box::new([]))
    }
}

impl From<Vec<usize>> for AccessorArray {
    fn from(vec: Vec<usize>) -> Self {
        let vec = vec.iter().map(|&x| x as u32).collect::<Vec<_>>();
        AccessorArray::U32(vec.into_boxed_slice())
    }
}

impl From<Vec<i8>> for AccessorArray {
    fn from(vec: Vec<i8>) -> Self {
        AccessorArray::I8(vec.into_boxed_slice())
    }
}

impl From<Vec<u8>> for AccessorArray {
    fn from(vec: Vec<u8>) -> Self {
        AccessorArray::U8(vec.into_boxed_slice())
    }
}

impl From<Vec<i16>> for AccessorArray {
    fn from(vec: Vec<i16>) -> Self {
        AccessorArray::I16(vec.into_boxed_slice())
    }
}

impl From<Vec<u16>> for AccessorArray {
    fn from(vec: Vec<u16>) -> Self {
        AccessorArray::U16(vec.into_boxed_slice())
    }
}

impl From<Vec<[u8; 2]>> for AccessorArray {
    fn from(vec: Vec<[u8; 2]>) -> Self {
        let vec = vec.iter().flatten().copied().collect::<Vec<_>>();
        AccessorArray::U8(vec.into_boxed_slice())
    }
}

impl From<Vec<[u8; 4]>> for AccessorArray {
    fn from(vec: Vec<[u8; 4]>) -> Self {
        let vec = vec.iter().flatten().copied().collect::<Vec<_>>();
        AccessorArray::U8(vec.into_boxed_slice())
    }
}

impl From<Vec<[u16; 2]>> for AccessorArray {
    fn from(vec: Vec<[u16; 2]>) -> Self {
        let vec = vec.iter().flatten().copied().collect::<Vec<_>>();
        AccessorArray::U16(vec.into_boxed_slice())
    }
}

impl From<Vec<[u16; 4]>> for AccessorArray {
    fn from(vec: Vec<[u16; 4]>) -> Self {
        let vec = vec.iter().flatten().copied().collect::<Vec<_>>();
        AccessorArray::U16(vec.into_boxed_slice())
    }
}

impl From<Vec<u32>> for AccessorArray {
    fn from(vec: Vec<u32>) -> Self {
        AccessorArray::U32(vec.into_boxed_slice())
    }
}

impl From<Vec<[u32; 2]>> for AccessorArray {
    fn from(vec: Vec<[u32; 2]>) -> Self {
        let vec = vec.iter().flatten().copied().collect::<Vec<_>>();
        AccessorArray::U32(vec.into_boxed_slice())
    }
}

impl From<Vec<[u32; 3]>> for AccessorArray {
    fn from(vec: Vec<[u32; 3]>) -> Self {
        let vec = vec.iter().flatten().copied().collect::<Vec<_>>();
        AccessorArray::U32(vec.into_boxed_slice())
    }
}

impl From<Vec<[u32; 4]>> for AccessorArray {
    fn from(vec: Vec<[u32; 4]>) -> Self {
        let vec = vec.iter().flatten().copied().collect::<Vec<_>>();
        AccessorArray::U32(vec.into_boxed_slice())
    }
}

impl From<Vec<f32>> for AccessorArray {
    fn from(vec: Vec<f32>) -> Self {
        AccessorArray::F32(vec.into_boxed_slice())
    }
}

impl From<Vec<[f32; 2]>> for AccessorArray {
    fn from(vec: Vec<[f32; 2]>) -> Self {
        let vec = vec.iter().flatten().copied().collect::<Vec<_>>();
        AccessorArray::F32(vec.into_boxed_slice())
    }
}

impl From<Vec<[f32; 3]>> for AccessorArray {
    fn from(vec: Vec<[f32; 3]>) -> Self {
        let vec = vec.iter().flatten().copied().collect::<Vec<_>>();
        AccessorArray::F32(vec.into_boxed_slice())
    }
}

impl From<Vec<[f32; 4]>> for AccessorArray {
    fn from(vec: Vec<[f32; 4]>) -> Self {
        let vec = vec.iter().flatten().copied().collect::<Vec<_>>();
        AccessorArray::F32(vec.into_boxed_slice())
    }
}

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

impl Into<gltf::json::mesh::Mode> for PrimitiveMode {
    fn into(self) -> gltf::json::mesh::Mode {
        match self {
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

impl Into<gltf::Semantic> for AttributeSemantic {
    fn into(self) -> gltf::Semantic {
        match self {
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
