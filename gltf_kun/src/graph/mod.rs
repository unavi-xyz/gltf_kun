use petgraph::graph::DiGraph;

pub mod gltf;
pub mod glxf;

#[derive(Debug)]
pub enum Weight {
    Mesh(gltf::mesh::MeshWeight),
    Node(gltf::node::NodeWeight),
    Primitive(gltf::primitive::PrimitiveWeight),
}

#[derive(Debug)]
pub enum Edge {
    Attributes,
    Child,
    Indices,
    Material,
    Mesh,
    Primitive,
}

pub type GltfGraph = DiGraph<Weight, Edge>;
