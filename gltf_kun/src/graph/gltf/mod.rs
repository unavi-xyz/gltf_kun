use gltf::json::mesh::Semantic;
use petgraph::graph::DiGraph;

pub mod accessor;
pub mod buffer;
pub mod buffer_view;
pub mod mesh;
pub mod node;
pub mod primitive;
pub mod scene;

#[derive(Debug)]
pub enum Weight {
    Accessor(accessor::AccessorWeight),
    Buffer(buffer::BufferWeight),
    BufferView(buffer_view::BufferViewWeight),
    DefaultScene,
    Mesh(mesh::MeshWeight),
    Node(node::NodeWeight),
    Other(Vec<u8>),
    Primitive(primitive::PrimitiveWeight),
    Scene(scene::SceneWeight),
}

#[derive(Debug)]
pub enum Edge {
    Attribute(Semantic),
    Buffer,
    BufferView,
    Child,
    Extension(&'static str),
    Indices,
    Material,
    Mesh,
    Primitive,
    Scene,
}

pub type GltfGraph = DiGraph<Weight, Edge>;
