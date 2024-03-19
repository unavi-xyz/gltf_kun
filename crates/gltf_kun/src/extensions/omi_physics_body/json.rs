use serde::{Deserialize, Serialize};

use super::weight::Motion;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct PhysicsBodyJson {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collider: Option<ShapeRefJson>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub motion: Option<Motion>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger: Option<ShapeRefJson>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct ShapeRefJson {
    pub shape: isize,
}

#[cfg(test)]
mod tests {
    use crate::extensions::omi_physics_body::weight::{BodyType, Motion};

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
