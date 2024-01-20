//! glTF extensions.
//!
//! Each extension IO is implemented for a specfic [format](crate::io::format).

use std::error::Error;

use petgraph::graph::NodeIndex;

use crate::{
    graph::{gltf::document::GltfDocument, Graph, Weight},
    io::format::gltf::GltfFormat,
};

use self::{omi_physics_body::io::OMIPhysicsBodyIO, omi_physics_shape::io::OMIPhysicsShapeIO};

pub mod omi_physics_body;
pub mod omi_physics_shape;

pub trait Extension: Copy + Sized + Into<NodeIndex> + From<NodeIndex> {
    fn name() -> &'static str;

    fn new(graph: &mut Graph) -> Self {
        let index = graph.add_node(Weight::Bytes(Default::default()));
        Self::from(index)
    }
}

pub trait ExtensionExport<D, F> {
    /// Export the extension from document -> format.
    fn export(graph: &mut Graph, doc: &D, format: &mut F) -> Result<(), Box<dyn Error>>;
}

pub trait ExtensionImport<D, F> {
    /// Import the extension from format -> document.
    fn import(graph: &mut Graph, format: &mut F, doc: &D) -> Result<(), Box<dyn Error>>;
}

pub trait Extensions<D, F> {
    fn export(&self, graph: &mut Graph, doc: &D, format: &mut F) -> Result<(), Box<dyn Error>>;
    fn import(&self, graph: &mut Graph, format: &mut F, doc: &D) -> Result<(), Box<dyn Error>>;
}

pub struct DefaultExtensions;

impl Extensions<GltfDocument, GltfFormat> for DefaultExtensions {
    fn export(
        &self,
        graph: &mut Graph,
        doc: &GltfDocument,
        format: &mut GltfFormat,
    ) -> Result<(), Box<dyn Error>> {
        OMIPhysicsShapeIO::export(graph, doc, format)?;
        OMIPhysicsBodyIO::export(graph, doc, format)?;

        Ok(())
    }

    fn import(
        &self,
        graph: &mut Graph,
        format: &mut GltfFormat,
        doc: &GltfDocument,
    ) -> Result<(), Box<dyn Error>> {
        OMIPhysicsShapeIO::import(graph, format, doc)?;
        OMIPhysicsBodyIO::import(graph, format, doc)?;

        Ok(())
    }
}
