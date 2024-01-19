use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    extensions::ExtensionIO,
    graph::{gltf::document::GltfDocument, ByteNode, Graph, Property},
    io::format::gltf::GltfFormat,
};

use super::{physics_shape::PhysicsShapeWeight, OMIPhysicsShapeExtension, EXTENSION_NAME};

#[derive(Debug, Serialize, Deserialize)]
struct RootExtension {
    shapes: Vec<PhysicsShapeWeight>,
}

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

        let shapes = ext
            .shapes(graph)
            .map(|shape| shape.read(graph))
            .collect::<Vec<_>>();

        let root_extension = RootExtension { shapes };

        let extensions = format
            .json
            .extensions
            .get_or_insert(gltf::json::extensions::Root::default());

        extensions.others.insert(
            EXTENSION_NAME.to_string(),
            serde_json::to_value(root_extension)?,
        );

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

impl From<PhysicsShapeWeight> for Value {
    fn from(weight: PhysicsShapeWeight) -> Self {
        let name = match weight {
            PhysicsShapeWeight::Box(_) => "box",
            PhysicsShapeWeight::Sphere(_) => "sphere",
            PhysicsShapeWeight::Capsule(_) => "capsule",
            PhysicsShapeWeight::Cylinder(_) => "cylinder",
            PhysicsShapeWeight::Convex => "convex",
            PhysicsShapeWeight::Trimesh => "trimesh",
        };

        let mut map = serde_json::Map::new();
        map.insert("type".to_string(), name.into());
        map.insert(
            name.to_string(),
            serde_json::to_value(weight).expect("Failed to serialize shape"),
        );

        map.into()
    }
}

impl From<Value> for PhysicsShapeWeight {
    fn from(value: Value) -> Self {
        let map = value.as_object().unwrap();

        let name = map.get("type").unwrap().as_str().unwrap();
        let weight = map.get(name).unwrap();

        match name {
            "box" => PhysicsShapeWeight::Box(
                serde_json::from_value(weight.clone()).expect("Failed to deserialize shape"),
            ),
            "sphere" => PhysicsShapeWeight::Sphere(
                serde_json::from_value(weight.clone()).expect("Failed to deserialize shape"),
            ),
            "capsule" => PhysicsShapeWeight::Capsule(
                serde_json::from_value(weight.clone()).expect("Failed to deserialize shape"),
            ),
            "cylinder" => PhysicsShapeWeight::Cylinder(
                serde_json::from_value(weight.clone()).expect("Failed to deserialize shape"),
            ),
            "convex" => PhysicsShapeWeight::Convex,
            "trimesh" => PhysicsShapeWeight::Trimesh,
            _ => panic!("Unknown shape type"),
        }
    }
}
