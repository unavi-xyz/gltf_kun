use gltf_kun::io::format::gltf::GltfFormat;
use tracing::info;

const CARGO_MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");
const ASSETS_DIR: &str = "../assets";
const MODEL: &str = "TriangleWithoutIndices/TriangleWithoutIndices.gltf";

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let path = format!("{}/{}/{}", CARGO_MANIFEST_DIR, ASSETS_DIR, MODEL);
    let doc = GltfFormat::import_file(&path).expect("Failed to import glTF");

    doc.nodes().iter().for_each(|node| {
        info!("Node: {:?}", node);
    });
}
