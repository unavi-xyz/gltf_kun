use gltf_kun::io::format::glb::GlbFormat;
use tracing::info;

const CARGO_MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");
const ASSETS_DIR: &str = "../assets";
const MODEL: &str = "BoxTextured.glb";

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let path = format!("{}/{}/{}", CARGO_MANIFEST_DIR, ASSETS_DIR, MODEL);
    let doc = GlbFormat::import_file(&path).expect("Failed to import glTF");

    doc.nodes().iter().for_each(|node| {
        info!("Node: {:?}", node);
    });
}
