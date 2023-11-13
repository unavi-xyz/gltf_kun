const CARGO_MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");
const ASSETS_DIR: &str = "../assets";
const MODEL: &str = "TriangleWithoutIndices/TriangleWithoutIndices.gltf";

fn main() {
    let path = format!("{}/{}/{}", CARGO_MANIFEST_DIR, ASSETS_DIR, MODEL);
    let gltf = gltf_kun::import(&path).unwrap();

    gltf.nodes().iter().for_each(|_node| {
        // println!("{:#?}", node.data());
    });
}
