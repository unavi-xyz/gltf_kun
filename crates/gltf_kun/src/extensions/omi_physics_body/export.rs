use std::error::Error;

use crate::{
    extensions::{omi_physics_shape::OmiPhysicsShape, ExtensionExport},
    graph::{gltf::document::GltfDocument, ByteNode, Extensions},
    io::format::gltf::GltfFormat,
};

use super::{
    json::{PhysicsBodyJson, ShapeRefJson},
    OmiPhysicsBody, EXTENSION_NAME,
};

impl ExtensionExport<GltfDocument, GltfFormat> for OmiPhysicsBody {
    fn export(
        graph: &mut crate::graph::Graph,
        doc: &GltfDocument,
        format: &mut GltfFormat,
    ) -> Result<(), Box<dyn Error>> {
        let mut added_extension = false;

        doc.nodes(graph)
            .iter()
            .enumerate()
            .filter_map(|(i, n)| n.get_extension::<Self>(graph).map(|e| (i, e)))
            .for_each(|(i, ext)| {
                let weight = ext.read(graph);

                let node = format
                    .json
                    .nodes
                    .get_mut(i)
                    .expect("Node index out of bounds");

                let extensions = node
                    .extensions
                    .get_or_insert(gltf::json::extensions::scene::Node::default());

                let collider = ext
                    .collider(graph)
                    .iter()
                    .filter_map(|s| doc.get_extension::<OmiPhysicsShape>(graph).map(|e| (e, s)))
                    .find_map(|(e, s)| e.shapes(graph).into_iter().position(|x| x == *s))
                    .map(|shape| ShapeRefJson {
                        shape: shape as isize,
                    });

                let trigger = ext
                    .trigger(graph)
                    .iter()
                    .filter_map(|s| doc.get_extension::<OmiPhysicsShape>(graph).map(|e| (e, s)))
                    .find_map(|(e, s)| e.shapes(graph).into_iter().position(|x| x == *s))
                    .map(|shape| ShapeRefJson {
                        shape: shape as isize,
                    });

                let json = PhysicsBodyJson {
                    collider,
                    trigger,
                    motion: weight.motion,
                };

                extensions.others.insert(
                    EXTENSION_NAME.to_string(),
                    serde_json::to_value(json).expect("Failed to serialize extension"),
                );

                added_extension = true;
            });

        if added_extension {
            format.json.extensions_used.push(EXTENSION_NAME.to_string());
        }

        Ok(())
    }
}
