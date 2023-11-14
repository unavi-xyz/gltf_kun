pub mod accessor;
pub mod attribute;
mod children;
mod from_json;
pub mod graph;
pub mod mesh;
pub mod node;
pub mod primitive;
pub mod scene;
mod to_json;

use std::{cell::RefCell, rc::Rc};

use accessor::Accessor;
use graph::{AccessorData, GltfGraph, GraphData, MeshData, NodeCover, NodeData, SceneData};
use mesh::Mesh;
use node::Node;
use scene::Scene;

#[derive(Default)]
pub struct Gltf {
    graph: Rc<RefCell<GltfGraph>>,
}

impl Gltf {
    pub fn from_json(json: &gltf::json::Root) -> Self {
        from_json::gltf_from_json(json)
    }

    pub fn to_json(&self) -> gltf::json::Root {
        to_json::gltf_to_json(self)
    }

    pub fn nodes(&self) -> Vec<Node> {
        self.graph
            .borrow()
            .node_indices()
            .filter_map(|index| match self.graph.borrow()[index] {
                GraphData::Node(_) => Some(Node::new(self.graph.clone(), index)),
                _ => None,
            })
            .collect()
    }

    pub fn meshes(&self) -> Vec<Mesh> {
        self.graph
            .borrow()
            .node_indices()
            .filter_map(|index| match self.graph.borrow()[index] {
                GraphData::Mesh(_) => Some(Mesh::new(self.graph.clone(), index)),
                _ => None,
            })
            .collect()
    }

    pub fn accessors(&self) -> Vec<Accessor> {
        self.graph
            .borrow()
            .node_indices()
            .filter_map(|index| match self.graph.borrow()[index] {
                GraphData::Accessor(_) => Some(Accessor::new(self.graph.clone(), index)),
                _ => None,
            })
            .collect()
    }

    pub fn create_accessor(&self) -> Accessor {
        let index = self
            .graph
            .borrow_mut()
            .add_node(GraphData::Accessor(AccessorData::default()));

        Accessor::new(self.graph.clone(), index)
    }

    pub fn create_scene(&mut self) -> Scene {
        let index = self
            .graph
            .borrow_mut()
            .add_node(GraphData::Scene(SceneData::default()));

        Scene::new(self.graph.clone(), index)
    }

    pub fn create_node(&mut self) -> Node {
        let index = self
            .graph
            .borrow_mut()
            .add_node(GraphData::Node(NodeData::default()));

        Node::new(self.graph.clone(), index)
    }

    pub fn create_mesh(&mut self) -> Mesh {
        let index = self
            .graph
            .borrow_mut()
            .add_node(GraphData::Mesh(MeshData::default()));

        Mesh::new(self.graph.clone(), index)
    }
}

/// Import a glTF from the file system
pub fn import(path: &str) -> Result<Gltf, gltf::Error> {
    let (doc, _, _) = gltf::import(path)?;
    Ok(Gltf::from_json(&doc.into_json()))
}
