use gltf_kun::io::format::{glb::GlbFormat, ExportFormat};
use tracing_test::traced_test;

const CARGO_MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");
const ASSETS_DIR: &str = "../assets";
const MODEL: &str = "BoxTextured.glb";

#[traced_test]
fn main() {
    let path = format!("{}/{}/{}", CARGO_MANIFEST_DIR, ASSETS_DIR, MODEL);
    let doc = GlbFormat::import_file(&path).expect("Failed to import glTF");
    let out = GlbFormat::export(doc).expect("Failed to export glTF");

    let (doc, _, _) = gltf::import_slice(&out.0).expect("Failed to import exported glb");

    assert!(doc.accessors().len() > 0);
    assert!(doc.buffers().len() > 0);
    assert!(doc.images().len() > 0);
    assert!(doc.materials().len() > 0);
    assert!(doc.meshes().len() > 0);
    assert!(doc.nodes().len() > 0);
    assert!(doc.scenes().len() > 0);
}
