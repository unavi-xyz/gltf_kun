use std::error::Error;

use tracing::warn;

use crate::{
    extensions::{ExtensionImport, omi_physics_shape::OmiPhysicsShape},
    graph::{ByteNode, Extensions, gltf::document::GltfDocument},
    io::format::gltf::GltfFormat,
};

use super::{EXTENSION_NAME, OmiPhysicsBody, OmiPhysicsBodyWeight, json::PhysicsBodyJson};

impl ExtensionImport<GltfDocument, GltfFormat> for OmiPhysicsBody {
    fn import(
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
                serde_json::from_value::<PhysicsBodyJson>(v.clone())
                    .map(|json| (i, json))
                    .ok()
            })
            .for_each(|(i, json)| {
                let nodes = doc.nodes(graph);
                let node = nodes.get(i).expect("Node index out of bounds");
                let ext = node.create_extension::<Self>(graph);

                if let Some(motion) = json.motion {
                    let weight = OmiPhysicsBodyWeight {
                        motion: Some(motion),
                    };
                    ext.write(graph, &weight);
                }

                if json.collider.is_none() && json.trigger.is_none() {
                    return;
                }

                let omi_physics_shapes = match doc.get_extension::<OmiPhysicsShape>(graph) {
                    Some(ext) => ext,
                    None => {
                        warn!("OMI_physics_shape extension not found");
                        return;
                    }
                };

                if let Some(shape_ref) = json.collider {
                    if let Ok(idx) = shape_ref.shape.try_into() {
                        let idx: usize = idx;

                        let shape = omi_physics_shapes.shapes(graph)[idx];

                        ext.set_collider(graph, Some(shape));
                    }
                }

                if let Some(shape_ref) = json.trigger {
                    if let Ok(idx) = shape_ref.shape.try_into() {
                        let idx: usize = idx;

                        let shape = omi_physics_shapes.shapes(graph)[idx];

                        ext.set_trigger(graph, Some(shape));
                    }
                }
            });

        Ok(())
    }
}
