use serde::{Deserialize, Serialize};

use super::physics_shape::PhysicsShapeWeight;

#[derive(Debug, Deserialize, Serialize)]
pub struct RootExtension {
    pub shapes: Vec<Shape>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Shape {
    #[serde(rename = "type", skip_deserializing)]
    pub typ: String,
    #[serde(
        alias = "box",
        alias = "sphere",
        alias = "capsule",
        alias = "cylinder",
        alias = "convex",
        alias = "trimesh",
        flatten
    )]
    pub weight: PhysicsShapeWeight,
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

#[cfg(test)]
mod tests {
    use crate::extensions::omi_physics_shape::physics_shape::{
        BoxShape, CapsuleShape, CylinderShape, Height, Radius, Size, SphereShape,
    };

    use super::*;

    #[test]
    fn box_serde() {
        let shape = BoxShape {
            size: Size([1.0, 2.0, 3.0]),
        };

        let json = {
            let weight = PhysicsShapeWeight::Box(shape.clone());
            serde_json::to_string(&Shape::from(weight)).expect("shape should serialize to json")
        };

        let expected = r#"{"type":"box","box":{"size":[1.0,2.0,3.0]}}"#;
        assert_eq!(json, expected);

        let shape_2 = {
            let s = serde_json::from_str::<Shape>(&json).expect("json should deserialize");
            match s.weight {
                PhysicsShapeWeight::Box(s) => s,
                _ => panic!("Wrong shape type"),
            }
        };

        assert_eq!(shape, shape_2);
    }

    #[test]
    fn sphere_serde() {
        let shape = SphereShape {
            radius: Radius(1.0),
        };

        let json = {
            let weight = PhysicsShapeWeight::Sphere(shape.clone());
            serde_json::to_string(&Shape::from(weight)).expect("shape should serialize to json")
        };

        let expected = r#"{"type":"sphere","sphere":{"radius":1.0}}"#;
        assert_eq!(json, expected);

        let shape_2 = {
            let s = serde_json::from_str::<Shape>(&json).expect("json should deserialize");
            match s.weight {
                PhysicsShapeWeight::Sphere(s) => s,
                _ => panic!("Wrong shape type"),
            }
        };

        assert_eq!(shape, shape_2);
    }

    #[test]
    fn capsule_serde() {
        let shape = CapsuleShape {
            radius: Radius(1.0),
            height: Height(2.5),
        };

        let json = {
            let weight = PhysicsShapeWeight::Capsule(shape.clone());
            serde_json::to_string(&Shape::from(weight)).expect("shape should serialize to json")
        };

        let expected = r#"{"type":"capsule","capsule":{"radius":1.0,"height":2.5}}"#;
        assert_eq!(json, expected);

        let shape_2 = {
            let s = serde_json::from_str::<Shape>(&json).expect("json should deserialize");
            match s.weight {
                PhysicsShapeWeight::Capsule(s) => s,
                _ => panic!("Wrong shape type"),
            }
        };

        assert_eq!(shape, shape_2);
    }

    #[test]
    fn cylinder_serde() {
        let shape = CylinderShape {
            radius: Radius(1.0),
            height: Height(2.5),
        };

        let json = {
            let weight = PhysicsShapeWeight::Cylinder(shape.clone());
            serde_json::to_string(&Shape::from(weight)).expect("shape should serialize to json")
        };

        let expected = r#"{"type":"cylinder","cylinder":{"radius":1.0,"height":2.5}}"#;
        assert_eq!(json, expected);

        let shape_2 = {
            let s = serde_json::from_str::<Shape>(&json).expect("json should deserialize");
            match s.weight {
                PhysicsShapeWeight::Cylinder(s) => s,
                _ => panic!("Wrong shape type"),
            }
        };

        assert_eq!(shape, shape_2);
    }

    #[test]
    fn default_box_serde() {
        let shape = Shape {
            typ: "box".to_string(),
            weight: PhysicsShapeWeight::Box(BoxShape::default()),
        };

        let json = serde_json::to_string(&shape).expect("json should serialize");
        let expected = r#"{"type":"box","box":{}}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn default_sphere_serde() {
        let shape = Shape {
            typ: "sphere".to_string(),
            weight: PhysicsShapeWeight::Sphere(SphereShape::default()),
        };

        let json = serde_json::to_string(&shape).expect("json should serialize");
        let expected = r#"{"type":"sphere","sphere":{}}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn default_capsule_serde() {
        let shape = Shape {
            typ: "capsule".to_string(),
            weight: PhysicsShapeWeight::Capsule(CapsuleShape::default()),
        };

        let json = serde_json::to_string(&shape).expect("json should serialize");
        let expected = r#"{"type":"capsule","capsule":{}}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn default_cylinder_serde() {
        let shape = Shape {
            typ: "cylinder".to_string(),
            weight: PhysicsShapeWeight::Cylinder(CylinderShape::default()),
        };

        let json = serde_json::to_string(&shape).expect("json should serialize");
        let expected = r#"{"type":"cylinder","cylinder":{}}"#;
        assert_eq!(json, expected);
    }
}
