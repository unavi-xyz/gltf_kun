use gltf::json;

use crate::Gltf;

pub fn gltf_from_json(_root: &json::Root) -> Gltf {
    Gltf::default()
}
