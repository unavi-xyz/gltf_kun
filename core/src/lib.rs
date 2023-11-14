//! An extensible library for building [glTF](https://github.com/KhronosGroup/glTF) toolchains.
//! Builds upon the [`gltf`](https://github.com/gltf-rs/gltf) and [`petgraph`](https://github.com/petgraph/petgraph) crates to create a traversable graph of the glTF document.
//!
//! # Basic Usage
//!
//! ```
//! use gltf_kun::Gltf;
//!
//! // Create a glTF document
//! let mut gltf = Gltf::default();
//!
//! // Create a node
//! let mut node = gltf.create_node();
//! node.set_name("My Node");
//! node.set_translation(1.0, 2.0, 3.0);
//!
//! // Create a scene and add the node to it
//! let mut scene = gltf.create_scene();
//! scene.add_node(&node);
//!
//! // Iterate over all nodes in the scene
//! scene.nodes().iter().for_each(|n| {
//!     println!("Node: {}", n.name());
//! });
//!
//! // Export to binary glb
//! let bytes = gltf.to_glb().to_vec();
//! std::fs::write("model.glb", bytes).unwrap();
//! ```

mod from_json;
pub mod graph;
mod properties;
mod to_json;

pub use properties::*;

use accessor::Accessor;
use graph::{AccessorData, GltfGraph, GraphData, MeshData, NodeData, SceneData};
use mesh::Mesh;
use node::Node;
use scene::Scene;
use std::{borrow::Cow, cell::RefCell, rc::Rc};

#[derive(Default)]
pub struct Gltf {
    graph: Rc<RefCell<GltfGraph>>,
}

impl Gltf {
    pub fn from_json(json: &gltf::json::Root) -> Self {
        from_json::gltf_from_json(json)
    }

    pub fn to_json(&self) -> (gltf::json::Root, Vec<u8>) {
        to_json::gltf_to_json(self)
    }

    pub fn to_glb(&self) -> gltf::Glb {
        let (json, binary) = self.to_json();

        let json_string = gltf::json::serialize::to_string(&json).expect("Serialization error");
        let mut json_offset = json_string.len() as u32;
        align_to_multiple_of_four(&mut json_offset);

        gltf::binary::Glb {
            header: gltf::binary::Header {
                magic: *b"glTF",
                version: 2,
                length: json_offset + binary.len() as u32,
            },
            bin: Some(Cow::Owned(to_padded_byte_vector(binary))),
            json: Cow::Owned(json_string.into_bytes()),
        }
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

fn align_to_multiple_of_four(n: &mut u32) {
    *n = (*n + 3) & !3;
}

fn to_padded_byte_vector<T>(vec: Vec<T>) -> Vec<u8> {
    let byte_length = vec.len() * std::mem::size_of::<T>();
    let byte_capacity = vec.capacity() * std::mem::size_of::<T>();
    let alloc = vec.into_boxed_slice();
    let ptr = Box::<[T]>::into_raw(alloc) as *mut u8;
    let mut new_vec = unsafe { Vec::from_raw_parts(ptr, byte_length, byte_capacity) };
    while new_vec.len() % 4 != 0 {
        new_vec.push(0); // pad to multiple of four bytes
    }
    new_vec
}
