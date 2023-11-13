use std::sync::{Arc, Mutex};

pub mod accessor;
pub mod attribute;
mod children;
pub mod graph;
pub mod mesh;
pub mod node;
pub mod primitive;
pub mod scene;

use accessor::Accessor;
use graph::{AccessorData, GltfGraph, GraphData, MeshData, NodeCover, NodeData, SceneData};
use mesh::Mesh;
use node::Node;
use scene::Scene;

#[derive(Default)]
pub struct Gltf {
    graph: Arc<Mutex<GltfGraph>>,
}

impl Gltf {
    /// Create a new Gltf from json
    pub fn from_json(_json: &gltf::json::Root) -> Self {
        let gltf = Gltf::default();

        gltf
    }

    pub fn nodes(&self) -> Vec<Node> {
        let graph = self.graph.lock().unwrap();

        graph
            .node_indices()
            .filter_map(|index| match graph[index] {
                GraphData::Node(_) => Some(Node::new(self.graph.clone(), index)),
                _ => None,
            })
            .collect()
    }

    pub fn create_accessor(&mut self) -> Accessor {
        let index = self
            .graph
            .lock()
            .unwrap()
            .add_node(GraphData::Accessor(AccessorData::default()));

        Accessor::new(self.graph.clone(), index)
    }

    pub fn create_scene(&mut self) -> Scene {
        let index = self
            .graph
            .lock()
            .unwrap()
            .add_node(GraphData::Scene(SceneData::default()));

        Scene::new(self.graph.clone(), index)
    }

    pub fn create_node(&mut self) -> Node {
        let index = self
            .graph
            .lock()
            .unwrap()
            .add_node(GraphData::Node(NodeData::default()));

        Node::new(self.graph.clone(), index)
    }

    pub fn create_mesh(&mut self) -> Mesh {
        let index = self
            .graph
            .lock()
            .unwrap()
            .add_node(GraphData::Mesh(MeshData::default()));

        Mesh::new(self.graph.clone(), index)
    }
}

/// Import a glTF from the file system
pub fn import(path: &str) -> Result<Gltf, gltf::Error> {
    let (doc, _, _) = gltf::import(path)?;
    Ok(Gltf::from_json(&doc.into_json()))
}
