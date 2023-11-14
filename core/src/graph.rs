use petgraph::graph::NodeIndex;
use std::{cell::RefCell, rc::Rc};

#[derive(Default, Debug, Clone)]
pub enum AccessorType {
    #[default]
    Scalar,
    Vec2,
    Vec3,
    Vec4,
    Mat2,
    Mat3,
    Mat4,
}

#[derive(Debug, Clone)]
pub enum AccessorArray {
    Byte(Box<[i8]>),
    UnsignedByte(Box<[u8]>),
    Short(Box<[i16]>),
    UnsignedShort(Box<[u16]>),
    UnsignedInt(Box<[u32]>),
    Float(Box<[f32]>),
}

impl Default for AccessorArray {
    fn default() -> Self {
        AccessorArray::Float(Box::new([]))
    }
}

impl From<Vec<usize>> for AccessorArray {
    fn from(vec: Vec<usize>) -> Self {
        let vec = vec.iter().map(|&x| x as u32).collect::<Vec<_>>();
        AccessorArray::UnsignedInt(vec.into_boxed_slice())
    }
}

impl From<Vec<[u8; 2]>> for AccessorArray {
    fn from(vec: Vec<[u8; 2]>) -> Self {
        let vec = vec.iter().flatten().copied().collect::<Vec<_>>();
        AccessorArray::UnsignedByte(vec.into_boxed_slice())
    }
}

impl From<Vec<[u8; 4]>> for AccessorArray {
    fn from(vec: Vec<[u8; 4]>) -> Self {
        let vec = vec.iter().flatten().copied().collect::<Vec<_>>();
        AccessorArray::UnsignedByte(vec.into_boxed_slice())
    }
}

impl From<Vec<[u16; 2]>> for AccessorArray {
    fn from(vec: Vec<[u16; 2]>) -> Self {
        let vec = vec.iter().flatten().copied().collect::<Vec<_>>();
        AccessorArray::UnsignedShort(vec.into_boxed_slice())
    }
}

impl From<Vec<[u16; 4]>> for AccessorArray {
    fn from(vec: Vec<[u16; 4]>) -> Self {
        let vec = vec.iter().flatten().copied().collect::<Vec<_>>();
        AccessorArray::UnsignedShort(vec.into_boxed_slice())
    }
}

impl From<Vec<u32>> for AccessorArray {
    fn from(vec: Vec<u32>) -> Self {
        AccessorArray::UnsignedInt(vec.into_boxed_slice())
    }
}

impl From<Vec<[u32; 2]>> for AccessorArray {
    fn from(vec: Vec<[u32; 2]>) -> Self {
        let vec = vec.iter().flatten().copied().collect::<Vec<_>>();
        AccessorArray::UnsignedInt(vec.into_boxed_slice())
    }
}

impl From<Vec<[u32; 3]>> for AccessorArray {
    fn from(vec: Vec<[u32; 3]>) -> Self {
        let vec = vec.iter().flatten().copied().collect::<Vec<_>>();
        AccessorArray::UnsignedInt(vec.into_boxed_slice())
    }
}

impl From<Vec<[u32; 4]>> for AccessorArray {
    fn from(vec: Vec<[u32; 4]>) -> Self {
        let vec = vec.iter().flatten().copied().collect::<Vec<_>>();
        AccessorArray::UnsignedInt(vec.into_boxed_slice())
    }
}

impl From<Vec<f32>> for AccessorArray {
    fn from(vec: Vec<f32>) -> Self {
        AccessorArray::Float(vec.into_boxed_slice())
    }
}

impl From<Vec<[f32; 2]>> for AccessorArray {
    fn from(vec: Vec<[f32; 2]>) -> Self {
        let vec = vec.iter().flatten().copied().collect::<Vec<_>>();
        AccessorArray::Float(vec.into_boxed_slice())
    }
}

impl From<Vec<[f32; 3]>> for AccessorArray {
    fn from(vec: Vec<[f32; 3]>) -> Self {
        let vec = vec.iter().flatten().copied().collect::<Vec<_>>();
        AccessorArray::Float(vec.into_boxed_slice())
    }
}

impl From<Vec<[f32; 4]>> for AccessorArray {
    fn from(vec: Vec<[f32; 4]>) -> Self {
        let vec = vec.iter().flatten().copied().collect::<Vec<_>>();
        AccessorArray::Float(vec.into_boxed_slice())
    }
}

#[derive(Debug, Default, Clone)]
pub struct AccessorData {
    pub name: Option<String>,
    pub accessor_type: AccessorType,
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

#[derive(Debug, Clone)]
pub struct AttributeData {
    pub semantic: AttributeSemantic,
}

#[derive(Debug, Clone)]
pub enum AttributeSemantic {
    Position,
    Normal,
    Tangent,
    TexCoord(u8),
    Color(u8),
    Joints(u8),
    Weights(u8),
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
