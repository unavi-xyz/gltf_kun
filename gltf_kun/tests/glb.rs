use std::path::Path;

use gltf_kun::{extensions::DefaultExtensions, graph::Graph, io::format::glb::GlbIO};
use tracing_test::traced_test;

const ASSETS_DIR: &str = "../assets";
const CARGO_MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");
const MODEL: &str = "BoxTextured.glb";

#[tokio::test]
#[traced_test]
async fn main() {
    let assets = Path::new(CARGO_MANIFEST_DIR).join(ASSETS_DIR);
    let path = assets.join(MODEL);

    // Import / export
    let mut graph = Graph::default();
    let doc = GlbIO::<DefaultExtensions>::import_file(&mut graph, &path)
        .await
        .expect("Failed to import glb");
    let out = GlbIO::<DefaultExtensions>::export(&mut graph, &doc).expect("Failed to export glb");
    let bytes = out.0.clone();

    assert!(!bytes.is_empty());

    // Write to file
    let path = assets.join("temp/glb/model.glb");
    std::fs::create_dir_all(path.parent().unwrap()).expect("Failed to create directory");
    std::fs::write(&path, out.0).expect("Failed to write glb to file");

    // Validate using gltf-rs
    let reader = std::fs::File::open(&path).expect("Failed to open exported glb");
    gltf::Glb::from_reader(&reader).expect("Failed to read exported glb");

    // Import / export written file
    let mut graph = Graph::default();
    let doc = GlbIO::<DefaultExtensions>::import_file(&mut graph, &path)
        .await
        .expect("Failed to import glb");
    let out = GlbIO::<DefaultExtensions>::export(&mut graph, &doc).expect("Failed to export glb");
    let bytes2 = out.0.clone();

    assert_eq!(bytes.len(), bytes2.len()); // Gives a better error message
    assert_eq!(bytes, bytes2);
}
