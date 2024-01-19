use crate::{
    extensions::ExtensionIO,
    graph::{gltf::document::GltfDocument, Graph, Property},
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
        let ext = match doc.get_extension::<OMIPhysicsShapeExtension>(graph) {
            Some(ext) => ext,
            None => return Ok(()),
        };

        Ok(())
    }

    fn import(
        &self,
        _graph: &mut Graph,
        _format: &mut GltfFormat,
        _doc: &GltfDocument,
    ) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}
