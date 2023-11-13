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

#[derive(Default, Debug, Clone)]
pub enum ComponentType {
    Byte = 5120,
    UnsignedByte = 5121,
    Short = 5122,
    UnsignedShort = 5123,
    UnsignedInt = 5125,
    #[default]
    Float = 5126,
}

#[derive(Default, Debug, Clone)]
pub struct AccessorData {
    pub name: Option<String>,
    pub accessor_type: AccessorType,
    pub component_type: ComponentType,
    pub count: usize,
    pub normalized: bool,
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

#[derive(Debug, Clone)]
#[derive(Default)]
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
