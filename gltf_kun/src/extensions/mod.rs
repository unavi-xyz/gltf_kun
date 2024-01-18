//! glTF extensions.
//!
//! Each extension IO is implemented for a specfic [format](crate::io::format).

use std::{collections::HashMap, error::Error, sync::Arc};

use petgraph::{graph::NodeIndex, visit::EdgeRef};
use serde::{Deserialize, Serialize};

use crate::{
    graph::{
        gltf::{document::GltfDocument, GltfEdge, GltfWeight},
        Edge, Graph, Weight,
    },
    io::format::gltf::GltfFormat,
};

pub mod omi_physics_body;
pub mod omi_physics_shape;

pub trait Extension<T: Serialize + for<'de> Deserialize<'de>> {
    fn name(&self) -> &'static str;

    fn decode_property(&self, bytes: &[u8]) -> Option<T> {
        bincode::deserialize(bytes).ok()
    }

    fn encode_property(&self, property: T) -> Vec<u8> {
        bincode::serialize(&property).unwrap()
    }

    fn properties(&self, graph: &Graph) -> Vec<T> {
        graph
            .node_indices()
            .flat_map(|n| {
                graph
                    .edges_directed(n, petgraph::Direction::Outgoing)
                    .filter_map(|e| match e.weight() {
                        Edge::Gltf(GltfEdge::Extension(name)) => {
                            if **name != *self.name() {
                                return None;
                            }

                            match graph.node_weight(e.target()) {
                                Some(Weight::Gltf(GltfWeight::Other(bytes))) => {
                                    self.decode_property(bytes)
                                }
                                _ => None,
                            }
                        }
                        _ => None,
                    })
            })
            .collect()
    }
}

pub trait ExtensionProperty<T: Serialize + for<'de> Deserialize<'de>>:
    Copy + Into<NodeIndex>
{
    fn extension(&self) -> &dyn Extension<T>;

    /// Reads the weight from the graph.
    /// Changes need to be written back to the graph using [Self::write].
    fn read(&self, graph: &Graph) -> T {
        match &graph[(*self).into()] {
            Weight::Gltf(GltfWeight::Other(bytes)) => self
                .extension()
                .decode_property(bytes)
                .expect("Failed to decode physics body"),
            _ => panic!("Incorrect weight type"),
        }
    }

    /// Writes the weight to the graph.
    fn write(&mut self, graph: &mut Graph, weight: T) {
        graph[(*self).into()] =
            Weight::Gltf(GltfWeight::Other(self.extension().encode_property(weight)));
    }
}

pub trait ExtensionIO<D, F>: Send + Sync {
    fn name(&self) -> &'static str;

    /// Export the extension from the document to the format.
    fn export(&self, graph: &mut Graph, doc: &D, format: &mut F) -> Result<(), Box<dyn Error>>;

    /// Import the extension from the format to the document.
    fn import(&self, graph: &mut Graph, format: &mut F, doc: &D) -> Result<(), Box<dyn Error>>;
}

pub struct Extensions<D, F> {
    pub map: HashMap<String, Arc<Box<dyn ExtensionIO<D, F>>>>,
}

impl Default for Extensions<GltfDocument, GltfFormat> {
    fn default() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
}

impl<D, F> Extensions<D, F> {
    pub fn add<E: ExtensionIO<D, F> + 'static>(&mut self, extension: E) {
        self.map
            .insert(extension.name().to_string(), Arc::new(Box::new(extension)));
    }

    pub fn get(&self, name: &str) -> Option<&Arc<Box<dyn ExtensionIO<D, F>>>> {
        self.map.get(name)
    }
}

impl<D, F> Clone for Extensions<D, F> {
    fn clone(&self) -> Self {
        let map = self
            .map
            .iter()
            .map(|(k, v)| (k.clone(), Arc::clone(v)))
            .collect();

        Self { map }
    }
}
