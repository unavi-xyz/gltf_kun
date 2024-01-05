use gltf_kun::format::{glb::GlbFormat, IoFormat};

const CARGO_MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");
const ASSETS_DIR: &str = "../assets";
const MODEL: &str = "BoxTextured.glb";

fn main() {
    let path = format!("{}/{}/{}", CARGO_MANIFEST_DIR, ASSETS_DIR, MODEL);
    println!("Loading: {}", path);

    let bytes = std::fs::read(path).expect("Failed to read file");
    let glb = GlbFormat(bytes);

    let graph = glb.to_graph();
    println!("{:#?}", graph);
}
