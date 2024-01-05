use petgraph::graph::DiGraph;

pub mod gltf;
pub mod glxf;

#[derive(Debug)]
pub enum Weight {
    Node(gltf::node::NodeWeight),
    Mesh(gltf::mesh::MeshWeight),
}

#[derive(Debug)]
pub enum Edge {
    Child,
    Mesh,
}

pub type GltfGraph = DiGraph<Weight, Edge>;
