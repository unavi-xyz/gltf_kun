use std::error::Error;

use crate::{graph::gltf::document::GltfDocument, io::format::gltf::GltfFormat};

use super::{ExtensionIO, OMIPhysicsBodyExtension, EXTENSION_NAME};

impl ExtensionIO<GltfDocument, GltfFormat> for OMIPhysicsBodyExtension {
    fn name(&self) -> &'static str {
        EXTENSION_NAME
    }

    fn export(
        &self,
        _graph: &mut crate::graph::Graph,
        _doc: &GltfDocument,
        _format: &mut GltfFormat,
    ) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn import(
        &self,
        _graph: &mut crate::graph::Graph,
        _format: &mut GltfFormat,
        _doc: &GltfDocument,
    ) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        extensions::omi_physics_body::OMIPhysicsBodyExtension,
        graph::{Graph, Property},
    };

    use super::*;

    #[test]
    fn test_io() {
        let mut graph = Graph::default();
        let doc = GltfDocument::new(&mut graph);
        let node = doc.create_node(&mut graph);

        node.create_extension::<OMIPhysicsBodyExtension>(&mut graph);
    }
}
