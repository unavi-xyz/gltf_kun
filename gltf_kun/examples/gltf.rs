use gltf_kun::format::{gltf::GltfFormat, IoFormat};

const CARGO_MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");
const ASSETS_DIR: &str = "../assets";
const MODEL: &str = "TriangleWithoutIndices/TriangleWithoutIndices.gltf";

fn main() {
    let path = format!("{}/{}/{}", CARGO_MANIFEST_DIR, ASSETS_DIR, MODEL);
    println!("Loading: {}", path);

    let file = std::fs::File::open(path).expect("Failed to open file");
    let json = serde_json::from_reader(file).expect("Failed to parse glTF");
    let blob = None;

    let format = GltfFormat { json, blob };
    let doc = format.import().expect("Failed to import glTF");

    doc.nodes().iter().for_each(|node| {
        println!("Node: {:?}", node);
    });

    let exported = GltfFormat::export(doc).expect("Failed to export glTF");
}
