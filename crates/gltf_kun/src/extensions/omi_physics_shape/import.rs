use crate::{
    extensions::ExtensionImport,
    graph::{Extensions, Graph, gltf::document::GltfDocument},
    io::format::gltf::GltfFormat,
};

use super::{EXTENSION_NAME, OmiPhysicsShape, json::RootExtension};

impl ExtensionImport<GltfDocument, GltfFormat> for OmiPhysicsShape {
    fn import(
        graph: &mut Graph,
        format: &mut GltfFormat,
        doc: &GltfDocument,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let Some(extensions) = &format.json.extensions else {
            return Ok(());
        };

        let Some(value) = extensions.others.get(EXTENSION_NAME) else {
            return Ok(());
        };

        let root_extension = serde_json::from_value::<RootExtension>(value.clone())?;

        let ext = doc
            .get_extension::<Self>(graph)
            .unwrap_or_else(|| doc.create_extension::<Self>(graph));

        root_extension.shapes.iter().for_each(|shape| {
            ext.create_shape(graph, &shape.weight);
        });

        Ok(())
    }
}
