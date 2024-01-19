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

        let ext = match doc.get_extension::<OMIPhysicsShapeExtension>(graph) {
            Some(ext) => ext,
            None => doc.create_extension::<OMIPhysicsShapeExtension>(graph),
        };

        root_extension.shapes.iter().for_each(|shape| {
            ext.create_shape(graph, shape);
        });

        Ok(())
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

        let shape = serde_json::to_value(weight).expect("Failed to serialize shape");
        let shape_map = shape.as_object().unwrap();
        shape_map.iter().for_each(|(key, value)| {
            map.insert(name.to_string(), value.clone());
            map.remove(key);
        });

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

#[cfg(test)]
mod tests {

    use crate::extensions::omi_physics_shape::physics_shape::{
        BoxShape, CapsuleShape, CylinderShape, SphereShape,
    };

    use super::*;

    #[test]
    fn test_box() {
        let shape = BoxShape {
            size: [1.0, 2.0, 3.0],
        };
        let weight = PhysicsShapeWeight::Box(shape.clone());
        let value = Value::from(weight);
        let json = serde_json::to_string(&value).unwrap();

        let expected = r#"{"box":{"size":[1.0,2.0,3.0]},"type":"box"}"#;
        assert_eq!(json, expected);

        let value = serde_json::from_str::<Value>(expected).unwrap();
        let weight = PhysicsShapeWeight::from(value);
        let shape_2 = match weight {
            PhysicsShapeWeight::Box(s) => s,
            _ => panic!("Wrong shape type"),
        };

        assert_eq!(shape, shape_2);
    }

    #[test]
    fn test_sphere() {
        let shape = SphereShape { radius: 1.0 };
        let weight = PhysicsShapeWeight::Sphere(shape.clone());
        let value = Value::from(weight);
        let json = serde_json::to_string(&value).unwrap();

        let expected = r#"{"sphere":{"radius":1.0},"type":"sphere"}"#;
        assert_eq!(json, expected);

        let value = serde_json::from_str::<Value>(expected).unwrap();
        let weight = PhysicsShapeWeight::from(value);
        let shape_2 = match weight {
            PhysicsShapeWeight::Sphere(s) => s,
            _ => panic!("Wrong shape type"),
        };

        assert_eq!(shape, shape_2);
    }

    #[test]
    fn test_capsule() {
        let shape = CapsuleShape {
            radius: 1.0,
            height: 2.0,
        };
        let weight = PhysicsShapeWeight::Capsule(shape.clone());
        let value = Value::from(weight);
        let json = serde_json::to_string(&value).unwrap();

        let expected = r#"{"capsule":{"height":2.0,"radius":1.0},"type":"capsule"}"#;
        assert_eq!(json, expected);

        let value = serde_json::from_str::<Value>(expected).unwrap();
        let weight = PhysicsShapeWeight::from(value);
        let shape_2 = match weight {
            PhysicsShapeWeight::Capsule(s) => s,
            _ => panic!("Wrong shape type"),
        };

        assert_eq!(shape, shape_2);
    }

    #[test]
    fn test_cylinder() {
        let shape = CylinderShape {
            radius: 1.0,
            height: 2.0,
        };
        let weight = PhysicsShapeWeight::Cylinder(shape.clone());
        let value = Value::from(weight);
        let json = serde_json::to_string(&value).unwrap();

        let expected = r#"{"cylinder":{"height":2.0,"radius":1.0},"type":"cylinder"}"#;
        assert_eq!(json, expected);

        let value = serde_json::from_str::<Value>(expected).unwrap();
        let weight = PhysicsShapeWeight::from(value);
        let shape_2 = match weight {
            PhysicsShapeWeight::Cylinder(s) => s,
            _ => panic!("Wrong shape type"),
        };

        assert_eq!(shape, shape_2);
    }
}
