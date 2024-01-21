use serde::{Deserialize, Serialize};

use crate::{
    extensions::{ExtensionExport, ExtensionImport},
    graph::{gltf::document::GltfDocument, ByteNode, Graph, Property},
    io::format::gltf::GltfFormat,
};

use super::{physics_shape::PhysicsShapeWeight, OMIPhysicsShape, EXTENSION_NAME};

#[derive(Debug, Deserialize, Serialize)]
struct RootExtension {
    shapes: Vec<Shape>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Shape {
    #[serde(rename = "type", skip_deserializing)]
    typ: String,
    #[serde(
        alias = "box",
        alias = "sphere",
        alias = "capsule",
        alias = "cylinder",
        alias = "convex",
        alias = "trimesh",
        flatten
    )]
    weight: PhysicsShapeWeight,
}

impl From<PhysicsShapeWeight> for Shape {
    fn from(weight: PhysicsShapeWeight) -> Self {
        let typ = match weight {
            PhysicsShapeWeight::Box(_) => "box",
            PhysicsShapeWeight::Sphere(_) => "sphere",
            PhysicsShapeWeight::Capsule(_) => "capsule",
            PhysicsShapeWeight::Cylinder(_) => "cylinder",
            PhysicsShapeWeight::Convex => "convex",
            PhysicsShapeWeight::Trimesh => "trimesh",
        };

        Self {
            typ: typ.to_string(),
            weight,
        }
    }
}

impl ExtensionExport<GltfDocument, GltfFormat> for OMIPhysicsShape {
    fn export(
        graph: &mut Graph,
        doc: &GltfDocument,
        format: &mut GltfFormat,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let ext = match doc.get_extension::<Self>(graph) {
            Some(ext) => ext,
            None => return Ok(()),
        };

        let shapes = ext
            .shapes(graph)
            .map(|shape| shape.read(graph).into())
            .collect::<Vec<_>>();

        if shapes.is_empty() {
            return Ok(());
        }

        let root_extension = RootExtension { shapes };

        let extensions = format
            .json
            .extensions
            .get_or_insert(gltf::json::extensions::Root::default());

        extensions.others.insert(
            EXTENSION_NAME.to_string(),
            serde_json::to_value(root_extension)?,
        );

        format.json.extensions_used.push(EXTENSION_NAME.to_string());

        Ok(())
    }
}

impl ExtensionImport<GltfDocument, GltfFormat> for OMIPhysicsShape {
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

#[cfg(test)]
mod tests {
    use crate::extensions::omi_physics_shape::physics_shape::{
        BoxShape, CapsuleShape, CylinderShape, Height, Radius, Size, SphereShape,
    };

    use super::*;

    #[test]
    fn test_box() {
        let shape = BoxShape {
            size: Size([1.0, 2.0, 3.0]),
        };

        let json = {
            let weight = PhysicsShapeWeight::Box(shape.clone());
            serde_json::to_string(&Shape::from(weight)).unwrap()
        };

        let expected = r#"{"type":"box","box":{"size":[1.0,2.0,3.0]}}"#;
        assert_eq!(json, expected);

        let shape_2 = {
            let s = serde_json::from_str::<Shape>(&json).unwrap();
            match s.weight {
                PhysicsShapeWeight::Box(s) => s,
                _ => panic!("Wrong shape type"),
            }
        };

        assert_eq!(shape, shape_2);
    }

    #[test]
    fn test_sphere() {
        let shape = SphereShape {
            radius: Radius(1.0),
        };

        let json = {
            let weight = PhysicsShapeWeight::Sphere(shape.clone());
            serde_json::to_string(&Shape::from(weight)).unwrap()
        };

        let expected = r#"{"type":"sphere","sphere":{"radius":1.0}}"#;
        assert_eq!(json, expected);

        let shape_2 = {
            let s = serde_json::from_str::<Shape>(&json).unwrap();
            match s.weight {
                PhysicsShapeWeight::Sphere(s) => s,
                _ => panic!("Wrong shape type"),
            }
        };

        assert_eq!(shape, shape_2);
    }

    #[test]
    fn test_capsule() {
        let shape = CapsuleShape {
            radius: Radius(1.0),
            height: Height(2.5),
        };

        let json = {
            let weight = PhysicsShapeWeight::Capsule(shape.clone());
            serde_json::to_string(&Shape::from(weight)).unwrap()
        };

        let expected = r#"{"type":"capsule","capsule":{"radius":1.0,"height":2.5}}"#;
        assert_eq!(json, expected);

        let shape_2 = {
            let s = serde_json::from_str::<Shape>(&json).unwrap();
            match s.weight {
                PhysicsShapeWeight::Capsule(s) => s,
                _ => panic!("Wrong shape type"),
            }
        };

        assert_eq!(shape, shape_2);
    }

    #[test]
    fn test_cylinder() {
        let shape = CylinderShape {
            radius: Radius(1.0),
            height: Height(2.5),
        };

        let json = {
            let weight = PhysicsShapeWeight::Cylinder(shape.clone());
            serde_json::to_string(&Shape::from(weight)).unwrap()
        };

        let expected = r#"{"type":"cylinder","cylinder":{"radius":1.0,"height":2.5}}"#;
        assert_eq!(json, expected);

        let shape_2 = {
            let s = serde_json::from_str::<Shape>(&json).unwrap();
            match s.weight {
                PhysicsShapeWeight::Cylinder(s) => s,
                _ => panic!("Wrong shape type"),
            }
        };

        assert_eq!(shape, shape_2);
    }

    #[test]
    fn test_default_box() {
        let shape = Shape {
            typ: "box".to_string(),
            weight: PhysicsShapeWeight::Box(BoxShape::default()),
        };

        let json = serde_json::to_string(&shape).unwrap();
        let expected = r#"{"type":"box","box":{}}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_default_sphere() {
        let shape = Shape {
            typ: "sphere".to_string(),
            weight: PhysicsShapeWeight::Sphere(SphereShape::default()),
        };

        let json = serde_json::to_string(&shape).unwrap();
        let expected = r#"{"type":"sphere","sphere":{}}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_default_capsule() {
        let shape = Shape {
            typ: "capsule".to_string(),
            weight: PhysicsShapeWeight::Capsule(CapsuleShape::default()),
        };

        let json = serde_json::to_string(&shape).unwrap();
        let expected = r#"{"type":"capsule","capsule":{}}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_default_cylinder() {
        let shape = Shape {
            typ: "cylinder".to_string(),
            weight: PhysicsShapeWeight::Cylinder(CylinderShape::default()),
        };

        let json = serde_json::to_string(&shape).unwrap();
        let expected = r#"{"type":"cylinder","cylinder":{}}"#;
        assert_eq!(json, expected);
    }
}
