//! glTF extensions.
//!
//! Each extension IO is implemented for a specfic [format](crate::io::format).

use std::{collections::HashMap, error::Error, sync::Arc};

use petgraph::graph::NodeIndex;

use crate::{
    graph::{gltf::document::GltfDocument, Graph, Property},
    io::format::gltf::GltfFormat,
};

pub mod omi_physics_body;
pub mod omi_physics_shape;

pub trait Extension: Sized + From<NodeIndex> {
    fn name() -> &'static str;

    fn get_extension(graph: &Graph, property: &impl Property) -> Option<Self> {
        property.get_extension::<Self>(graph, Self::name())
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
