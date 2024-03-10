//! glTF extensions.
//!
//! Each extension IO is implemented for a specfic [format](crate::io::format).

use std::error::Error;

use petgraph::graph::NodeIndex;

use crate::{
    graph::{gltf::document::GltfDocument, Graph, Weight},
    io::format::gltf::GltfFormat,
};

#[cfg(feature = "omi_physics")]
pub mod omi_physics_body;
#[cfg(feature = "omi_physics")]
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

/// IO for a collection of extensions.
pub trait ExtensionsIO<D, F> {
    fn export(graph: &mut Graph, doc: &D, format: &mut F) -> Result<(), Box<dyn Error>>;
    fn import(graph: &mut Graph, format: &mut F, doc: &D) -> Result<(), Box<dyn Error>>;
}

pub struct DefaultExtensions;

impl ExtensionsIO<GltfDocument, GltfFormat> for DefaultExtensions {
    fn export(
        graph: &mut Graph,
        doc: &GltfDocument,
        format: &mut GltfFormat,
    ) -> Result<(), Box<dyn Error>> {
        #[cfg(feature = "omi_physics")]
        {
            omi_physics_shape::OMIPhysicsShape::export(graph, doc, format)?;
            omi_physics_body::OMIPhysicsBody::export(graph, doc, format)?;
        }

        Ok(())
    }

    fn import(
        graph: &mut Graph,
        format: &mut GltfFormat,
        doc: &GltfDocument,
    ) -> Result<(), Box<dyn Error>> {
        #[cfg(feature = "omi_physics")]
        {
            omi_physics_shape::OMIPhysicsShape::import(graph, format, doc)?;
            omi_physics_body::OMIPhysicsBody::import(graph, format, doc)?;
        }

        Ok(())
    }
}
