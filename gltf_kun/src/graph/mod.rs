use gltf::json::mesh::Semantic;
use petgraph::graph::DiGraph;

pub mod accessor;
pub mod mesh;
pub mod node;
pub mod primitive;
pub mod scene;

#[derive(Debug)]
pub enum Weight {
    Accessor(accessor::AccessorWeight),
    Mesh(mesh::MeshWeight),
    Node(node::NodeWeight),
    Scene(scene::SceneWeight),
    Primitive(primitive::PrimitiveWeight),
}

#[derive(Debug)]
pub enum Edge {
    Attribute(Semantic),
    Child,
    Indices,
    Material,
    Mesh,
    Primitive,
}

pub type GltfGraph = DiGraph<Weight, Edge>;
