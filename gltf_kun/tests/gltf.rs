use std::path::Path;

use gltf_kun::io::format::{gltf::GltfFormat, ExportFormat};
use tracing::debug;
use tracing_test::traced_test;

const CARGO_MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");
const ASSETS_DIR: &str = "../assets";
const MODEL: &str = "BoxTextured/BoxTextured.gltf";

#[test]
#[traced_test]
fn main() {
    let assets = Path::new(CARGO_MANIFEST_DIR).join(ASSETS_DIR);
    let path = assets.join(MODEL);

    // Import / export
    let doc = GltfFormat::import_file(&path).expect("Failed to import glTF");
    let out = GltfFormat::export(doc).expect("Failed to export glTF");
    let out_json = serde_json::to_string(&out.json).expect("Failed to serialize json");

    debug!(
        "JSON:\n{}",
        serde_json::to_string_pretty(&out.json).unwrap()
    );

    // Write to file
    let path = assets.join("temp/gltf/model.gltf");
    std::fs::create_dir_all(path.parent().unwrap()).expect("Failed to create directory");
    out.write_file(&path).expect("Failed to write glTF to file");

    // Validate using gltf-rs
    gltf::import(&path).expect("Failed to read exported glTF");

    // Import written file
    let doc = GltfFormat::import_file(&path).expect("Failed to import glTF");
    let out = GltfFormat::export(doc).expect("Failed to export glTF");
    let out_json2 = serde_json::to_string(&out.json).expect("Failed to serialize json");

    assert_eq!(out_json, out_json2);
}
