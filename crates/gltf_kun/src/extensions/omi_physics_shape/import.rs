use crate::{
    extensions::ExtensionImport,
    graph::{gltf::document::GltfDocument, Extensions, Graph},
    io::format::gltf::GltfFormat,
};

use super::{json::RootExtension, OmiPhysicsShape, EXTENSION_NAME};

impl ExtensionImport<GltfDocument, GltfFormat> for OmiPhysicsShape {
    fn import(
        graph: &mut Graph,
        format: &mut GltfFormat,
        doc: &GltfDocument,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let extensions = match &format.json.extensions {
            Some(extensions) => extensions,
            None => return Ok(()),
        };

        let value = match extensions.others.get(EXTENSION_NAME) {
            Some(extension) => extension,
            None => return Ok(()),
        };

        let root_extension = serde_json::from_value::<RootExtension>(value.clone())?;

        let ext = match doc.get_extension::<Self>(graph) {
            Some(ext) => ext,
            None => doc.create_extension::<Self>(graph),
        };

        root_extension.shapes.iter().for_each(|shape| {
            ext.create_shape(graph, &shape.weight);
        });

        Ok(())
    }
}
