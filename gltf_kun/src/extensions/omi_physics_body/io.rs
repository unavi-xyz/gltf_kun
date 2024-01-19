use std::error::Error;

use crate::{
    graph::{gltf::document::GltfDocument, ByteNode, Property},
    io::format::gltf::GltfFormat,
};

use super::{ExtensionIO, OMIPhysicsBodyExtension, PhysicsBodyWeight, EXTENSION_NAME};

impl ExtensionIO<GltfDocument, GltfFormat> for OMIPhysicsBodyExtension {
    fn name(&self) -> &'static str {
        EXTENSION_NAME
    }

    fn export(
        &self,
        graph: &mut crate::graph::Graph,
        doc: &GltfDocument,
        format: &mut GltfFormat,
    ) -> Result<(), Box<dyn Error>> {
        doc.nodes(graph)
            .iter()
            .enumerate()
            .filter_map(|(i, n)| {
                n.get_extension::<OMIPhysicsBodyExtension>(graph)
                    .map(|ext| (i, ext.read(graph)))
            })
            .for_each(|(i, weight)| {
                let node = format
                    .json
                    .nodes
                    .get_mut(i)
                    .expect("Node index out of bounds");

                let extensions = node
                    .extensions
                    .get_or_insert(gltf::json::extensions::scene::Node::default());

                extensions.others.insert(
                    EXTENSION_NAME.to_string(),
                    serde_json::to_value(weight).expect("Failed to serialize extension"),
                );
            });

        Ok(())
    }

    fn import(
        &self,
        graph: &mut crate::graph::Graph,
        format: &mut GltfFormat,
        doc: &GltfDocument,
    ) -> Result<(), Box<dyn Error>> {
        format
            .json
            .nodes
            .iter()
            .enumerate()
            .filter_map(|(i, n)| n.extensions.as_ref().map(|e| (i, e)))
            .filter_map(|(i, e)| e.others.get(EXTENSION_NAME).map(|v| (i, v)))
            .filter_map(|(i, v)| {
                serde_json::from_value::<PhysicsBodyWeight>(v.clone())
                    .ok()
                    .map(|w| (i, w))
            })
            .for_each(|(i, weight)| {
                let nodes = doc.nodes(graph);
                let node = nodes.get(i).expect("Node index out of bounds");
                let mut ext = node.create_extension::<OMIPhysicsBodyExtension>(graph);
                ext.write(graph, &weight);
            });

        Ok(())
    }
}
