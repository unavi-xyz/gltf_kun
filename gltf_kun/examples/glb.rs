use gltf_kun::format::{glb::GlbFormat, IoFormat};

const CARGO_MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");
const ASSETS_DIR: &str = "../assets";
const MODEL: &str = "BoxTextured.glb";

fn main() {
    let path = format!("{}/{}/{}", CARGO_MANIFEST_DIR, ASSETS_DIR, MODEL);
    println!("Loading: {}", path);

    let file = std::fs::File::open(path).expect("Failed to open file");
    let glb = gltf::Glb::from_reader(file).expect("Failed to parse glTF");
    let format = GlbFormat(glb);
    let doc = format.import().expect("Failed to import glTF");

    doc.nodes().iter().for_each(|node| {
        println!("Node: {:?}", node);
    });

    let exported = GlbFormat::export(doc).expect("Failed to export glTF");
    let _ = exported.0.to_vec().expect("Failed to convert to bytes");
}
