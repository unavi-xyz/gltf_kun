use petgraph::graph::DiGraph;

pub mod gltf;
pub mod glxf;

#[derive(Debug)]
pub enum Weight {
    Node(gltf::node::RawNode),
    Mesh(gltf::mesh::RawMesh),
}

#[derive(Debug)]
pub enum Edge {
    Child,
}

pub type GltfGraph = DiGraph<Weight, Edge>;

pub trait Property {
    fn name(&self) -> Option<String>;
    fn set_name(&mut self, name: Option<String>);

    // fn extras(&self) -> &Option<serde_json::Value>;
    // fn set_extras(&mut self, extras: Option<serde_json::Value>);

    // fn extensions(&self) -> &Vec<Box<dyn ExtensionProperty>>;
    // fn set_extensions(&mut self, extensions: Vec<Box<dyn ExtensionProperty>>);
    // fn add_extension(&mut self, extension: Box<dyn ExtensionProperty>);
}
