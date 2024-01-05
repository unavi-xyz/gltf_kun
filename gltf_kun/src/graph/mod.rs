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
    Mesh(mesh::MeshWeight),
    Node(node::NodeWeight),
    Scene(scene::SceneWeight),
    Primitive(primitive::PrimitiveWeight),
}

#[derive(Debug)]
pub enum Edge {
    Attribute(Semantic),
    Buffer,
    BufferView,
    Child,
    Indices,
    Material,
    Mesh,
    Primitive,
}

pub type GltfGraph = DiGraph<Weight, Edge>;
