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
        extensions::{
            omi_physics_body::{
                physics_body::{BodyType, Motion},
                OMIPhysicsBodyExtension,
            },
            ExtensionProperty,
        },
        graph::Graph,
        io::format::glb::GlbIO,
    };

    use super::*;

    #[test]
    fn test_io() {
        let mut graph = Graph::default();
        let doc = GltfDocument::new(&mut graph);
        let node = doc.create_node(&mut graph);

        let mut physics_body = OMIPhysicsBodyExtension::create_body(&mut graph, &node);

        let mut weight = physics_body.read(&graph);
        weight.motion = Some(Motion::new(BodyType::Dynamic));
        physics_body.write(&mut graph, &weight);

        let mut io = GlbIO::default();
        io.extensions.add(OMIPhysicsBodyExtension);
    }
}
