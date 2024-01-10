use gltf_kun::io::format::{gltf::GltfFormat, ExportFormat};
use tracing_test::traced_test;

const CARGO_MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");
const ASSETS_DIR: &str = "../assets";
const MODEL: &str = "TriangleWithoutIndices/TriangleWithoutIndices.gltf";

#[traced_test]
fn main() {
    let path = format!("{}/{}/{}", CARGO_MANIFEST_DIR, ASSETS_DIR, MODEL);
    let doc = GltfFormat::import_file(&path).expect("Failed to import glTF");
    let out = GltfFormat::export(doc).expect("Failed to export glTF");

    assert!(!out.json.accessors.is_empty());
    assert!(!out.json.buffer_views.is_empty());
    assert!(!out.json.buffers.is_empty());
    assert!(!out.json.meshes.is_empty());
    assert!(!out.json.nodes.is_empty());
    assert!(!out.json.scenes.is_empty());
}
