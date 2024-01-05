use gltf_json::mesh::Semantic;
use petgraph::graph::DiGraph;

pub mod gltf;
pub mod glxf;

#[derive(Debug)]
pub enum Weight {
    Accessor(gltf::accessor::AccessorWeight),
    Mesh(gltf::mesh::MeshWeight),
    Node(gltf::node::NodeWeight),
    Scene(gltf::scene::SceneWeight),
    Primitive(gltf::primitive::PrimitiveWeight),
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
