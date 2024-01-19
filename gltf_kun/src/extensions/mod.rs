//! glTF extensions.
//!
//! Each extension IO is implemented for a specfic [format](crate::io::format).

use std::{collections::HashMap, error::Error};

use petgraph::graph::NodeIndex;

use crate::graph::{Graph, Property, Weight};

pub mod omi_physics_body;
pub mod omi_physics_shape;

pub trait Extension<T: Property>: Copy + Sized + Into<NodeIndex> + From<NodeIndex> {
    fn name() -> &'static str;

    fn new(graph: &mut Graph) -> Self {
        let index = graph.add_node(Weight::Bytes(Default::default()));
        Self::from(index)
    }
}

pub trait ExtensionIO<D, F>: Copy + Send + Sync {
    fn name() -> &'static str;

    /// Export the extension from the document to the format.
    fn export(&self, graph: &mut Graph, doc: &D, format: &mut F) -> Result<(), Box<dyn Error>>;

    /// Import the extension from the format to the document.
    fn import(&self, graph: &mut Graph, format: &mut F, doc: &D) -> Result<(), Box<dyn Error>>;
}

pub struct Extensions<D, F> {
    pub map: HashMap<String, Box<dyn ExtensionIO<D, F>>>,
}

impl<D, F> Clone for Extensions<D, F> {
    fn clone(&self) -> Self {
        let map = self.map.iter().map(|(k, v)| (k.clone(), *v)).collect();
        Self { map }
    }
}

impl<D, F> Extensions<D, F> {
    pub fn add<E: ExtensionIO<D, F>>(&mut self, extension: E) {
        self.map.insert(E::name().to_string(), Box::new(extension));
    }

    pub fn get(&self, name: &str) -> Option<&Box<dyn ExtensionIO<D, F>>> {
        self.map.get(name)
    }
}
