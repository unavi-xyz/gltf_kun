use std::error::Error;

use serde::{Deserialize, Serialize};
use tracing::warn;

use crate::{
    extensions::{omi_physics_shape::OMIPhysicsShape, ExtensionExport, ExtensionImport},
    graph::{gltf::document::GltfDocument, ByteNode, Property},
    io::format::gltf::GltfFormat,
};

use super::{Motion, OMIPhysicsBody, OMIPhysicsBodyWeight, EXTENSION_NAME};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct PhysicsBodyJson {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    collider: Option<ShapeRefJson>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    motion: Option<Motion>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    trigger: Option<ShapeRefJson>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct ShapeRefJson {
    pub shape: isize,
}

impl ExtensionExport<GltfDocument, GltfFormat> for OMIPhysicsBody {
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
                    .filter_map(|s| doc.get_extension::<OMIPhysicsShape>(graph).map(|e| (e, s)))
                    .find_map(|(e, s)| e.shapes(graph).position(|x| x == *s))
                    .map(|shape| ShapeRefJson {
                        shape: shape as isize,
                    });

                let trigger = ext
                    .trigger(graph)
                    .iter()
                    .filter_map(|s| doc.get_extension::<OMIPhysicsShape>(graph).map(|e| (e, s)))
                    .find_map(|(e, s)| e.shapes(graph).position(|x| x == *s))
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

impl ExtensionImport<GltfDocument, GltfFormat> for OMIPhysicsBody {
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
                let mut ext = node.create_extension::<Self>(graph);

                // Motion
                if let Some(motion) = json.motion {
                    let weight = OMIPhysicsBodyWeight {
                        motion: Some(motion),
                    };
                    ext.write(graph, &weight);
                }

                if json.collider.is_none() && json.trigger.is_none() {
                    return;
                }

                let omi_physics_shapes = match doc.get_extension::<OMIPhysicsShape>(graph) {
                    Some(ext) => ext,
                    None => {
                        warn!("OMI_physics_shape extension not found");
                        return;
                    }
                };

                // Collider
                if let Some(shape_ref) = json.collider {
                    if let Ok(idx) = shape_ref.shape.try_into() {
                        let shape = omi_physics_shapes
                            .shapes(graph)
                            .nth(idx)
                            .expect("Collider index out of bounds");

                        ext.set_collider(graph, Some(&shape));
                    }
                }

                // Trigger
                if let Some(shape_ref) = json.trigger {
                    if let Ok(idx) = shape_ref.shape.try_into() {
                        let shape = omi_physics_shapes
                            .shapes(graph)
                            .nth(idx)
                            .expect("Trigger index out of bounds");

                        ext.set_trigger(graph, Some(&shape));
                    }
                }
            });

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::extensions::omi_physics_body::{BodyType, Motion};

    use super::*;

    #[test]
    fn motion_serde() {
        let json = PhysicsBodyJson {
            motion: Some(Motion::new(BodyType::Dynamic)),
            trigger: None,
            collider: None,
        };

        let json_str = serde_json::to_string(&json).unwrap();
        let expected = r#"{"motion":{"type":"dynamic"}}"#;
        assert_eq!(json_str, expected);

        let json_2 = serde_json::from_str::<PhysicsBodyJson>(&json_str).unwrap();
        assert_eq!(json, json_2);
    }
}
