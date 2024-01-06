use gltf_kun::io::format::{glb::GlbFormat, ExportFormat};
use tracing_test::traced_test;

const CARGO_MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");
const ASSETS_DIR: &str = "../assets";
const MODEL: &str = "BoxTextured.glb";

#[traced_test]
fn main() {
    let path = format!("{}/{}/{}", CARGO_MANIFEST_DIR, ASSETS_DIR, MODEL);
    let doc = GlbFormat::import_file(&path).expect("Failed to import glTF");
    let _out = GlbFormat::export(doc).expect("Failed to export glTF");
}
