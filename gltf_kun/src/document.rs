use crate::graph::{
    accessor::Accessor, mesh::Mesh, node::Node, primitive::Primitive, GltfGraph, Weight,
};

pub struct Document(pub GltfGraph);

impl Document {
    pub fn accessors(&self) -> Vec<Accessor> {
        self.0
            .node_indices()
            .filter(|weight| matches!(self.0[*weight], Weight::Accessor(_)))
            .map(Accessor)
            .collect()
    }

    pub fn meshes(&self) -> Vec<Mesh> {
        self.0
            .node_indices()
            .filter(|weight| matches!(self.0[*weight], Weight::Mesh(_)))
            .map(Mesh)
            .collect()
    }

    pub fn nodes(&self) -> Vec<Node> {
        self.0
            .node_indices()
            .filter(|weight| matches!(self.0[*weight], Weight::Node(_)))
            .map(Node)
            .collect()
    }

    pub fn primitives(&self) -> Vec<Primitive> {
        self.0
            .node_indices()
            .filter(|weight| matches!(self.0[*weight], Weight::Primitive(_)))
            .map(Primitive)
            .collect()
    }

    pub fn scenes(&self) -> Vec<Node> {
        self.0
            .node_indices()
            .filter(|weight| matches!(self.0[*weight], Weight::Scene(_)))
            .map(Node)
            .collect()
    }
}
