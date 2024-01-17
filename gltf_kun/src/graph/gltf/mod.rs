use gltf::json::mesh::Semantic;
use petgraph::graph::DiGraph;

use crate::extensions::ExtensionProperty;

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
    Primitive(primitive::PrimitiveWeight),
    Scene(scene::SceneWeight),
    ExtensionProperty(Box<dyn ExtensionProperty>),
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
    Scene,
}

pub type GltfGraph = DiGraph<Weight, Edge>;
