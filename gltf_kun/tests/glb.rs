use std::path::Path;

use gltf_kun::io::format::glb::GlbFormat;
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
    let doc = GlbFormat::import_file(&path)
        .await
        .expect("Failed to import glTF");
    let out = GlbFormat::export(doc).expect("Failed to export glTF");
    let out_bytes = out.0.clone();

    assert!(!out_bytes.is_empty());

    // Write to file
    let path = assets.join("temp/glb/model.glb");
    std::fs::create_dir_all(path.parent().unwrap()).expect("Failed to create directory");
    std::fs::write(&path, out.0).expect("Failed to write glb to file");

    // Validate using gltf-rs
    let reader = std::fs::File::open(&path).expect("Failed to open exported glb");
    gltf::Glb::from_reader(&reader).expect("Failed to read exported glb");

    // Import / export written file
    let doc = GlbFormat::import_file(&path)
        .await
        .expect("Failed to import glTF");
    let out = GlbFormat::export(doc).expect("Failed to export glTF");
    let out_bytes2 = out.0.clone();

    // TODO: Figure out why this fails
    // assert_eq!(out_bytes.len(), out_bytes2.len());
}
