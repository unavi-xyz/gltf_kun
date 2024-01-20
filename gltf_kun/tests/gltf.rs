use std::path::Path;

use gltf_kun::{extensions::DefaultExtensions, graph::Graph, io::format::gltf::GltfIO};
use tracing::debug;
use tracing_test::traced_test;

const ASSETS_DIR: &str = "../assets";
const CARGO_MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");
const MODEL: &str = "BoxTextured/BoxTextured.gltf";

#[tokio::test]
#[traced_test]
async fn main() {
    let assets = Path::new(CARGO_MANIFEST_DIR).join(ASSETS_DIR);
    let path = assets.join(MODEL);

    let io = GltfIO;
    let extensions = Some(&DefaultExtensions);

    // Import / export
    let mut graph = Graph::default();
    let doc = io
        .import_file(&mut graph, &path, extensions)
        .await
        .expect("Failed to import glTF");
    let out = io
        .export(&mut graph, &doc, extensions)
        .expect("Failed to export glTF");
    let json = serde_json::to_string(&out.json).expect("Failed to serialize json");

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

    // Import / export written file
    let mut graph = Graph::default();
    let doc = io
        .import_file(&mut graph, &path, extensions)
        .await
        .expect("Failed to import glTF");
    let out = io
        .export(&mut graph, &doc, extensions)
        .expect("Failed to export glTF");
    let json2 = serde_json::to_string(&out.json).expect("Failed to serialize json");

    assert_eq!(json, json2);
}
