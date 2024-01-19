use crate::{
    extensions::ExtensionIO,
    graph::{gltf::document::GltfDocument, Graph},
    io::format::gltf::GltfFormat,
};

use super::{OMIPhysicsShapeExtension, EXTENSION_NAME};

impl ExtensionIO<GltfDocument, GltfFormat> for OMIPhysicsShapeExtension {
    fn name(&self) -> &'static str {
        EXTENSION_NAME
    }

    fn export(
        &self,
        graph: &mut Graph,
        doc: &GltfDocument,
        format: &mut GltfFormat,
    ) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn import(
        &self,
        graph: &mut Graph,
        format: &mut GltfFormat,
        doc: &GltfDocument,
    ) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}
