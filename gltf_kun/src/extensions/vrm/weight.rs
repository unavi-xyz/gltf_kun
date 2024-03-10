use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct VrmWeight {}

impl From<&Vec<u8>> for VrmWeight {
    fn from(bytes: &Vec<u8>) -> Self {
        if bytes.is_empty() {
            return Self::default();
        }
        serde_json::from_slice(bytes).expect("Failed to deserialize weight")
    }
}

impl From<&VrmWeight> for Vec<u8> {
    fn from(value: &VrmWeight) -> Self {
        serde_json::to_vec(value).expect("Failed to serialize weight")
    }
}
