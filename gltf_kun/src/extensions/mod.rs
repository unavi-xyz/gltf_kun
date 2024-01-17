//! glTF extensions.
//!
//! Each extension IO is implemented for a specfic [format](crate::io::format).

use std::{collections::HashMap, error::Error, sync::Arc};

use petgraph::visit::EdgeRef;
use serde::{Deserialize, Serialize};

use crate::graph::gltf::{Edge, GltfGraph, Weight};

pub mod omi_physics_body;
pub mod omi_physics_shape;

pub trait Extension {
    type PropertyWeight: Serialize + for<'de> Deserialize<'de>;

    fn name(&self) -> &'static str;

    fn decode_property(&self, bytes: &[u8]) -> Option<Self::PropertyWeight> {
        bincode::deserialize(bytes).ok()
    }

    fn encode_property(&self, property: Self::PropertyWeight) -> Vec<u8> {
        bincode::serialize(&property).unwrap()
    }

    fn properties(&self, graph: &GltfGraph) -> Vec<Self::PropertyWeight> {
        graph
            .node_indices()
            .flat_map(|n| {
                graph
                    .edges_directed(n, petgraph::Direction::Outgoing)
                    .filter_map(|e| match e.weight() {
                        Edge::Extension(name) => {
                            if **name != *self.name() {
                                return None;
                            }

                            match graph.node_weight(e.target()) {
                                Some(Weight::Other(bytes)) => match self.decode_property(bytes) {
                                    Some(property) => Some(property),
                                    None => None,
                                },
                                _ => None,
                            }
                        }
                        _ => None,
                    })
            })
            .collect()
    }
}

#[derive(Default)]
pub struct Extensions<D, F> {
    pub map: HashMap<String, Arc<Box<dyn ExtensionIO<D, F>>>>,
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

pub trait ExtensionIO<D, F>: Send + Sync {
    fn name(&self) -> &'static str;

    /// Export the extension from the document to the format.
    fn export(&self, doc: &mut D, format: &mut F) -> Result<(), Box<dyn Error>>;

    /// Import the extension from the format to the document.
    fn import(&self, format: &mut F, doc: &mut D) -> Result<(), Box<dyn Error>>;
}
