use std::{fs, io};

const CARGO_MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");
const ASSETS_DIR: &str = "../assets";

const MODEL: &str = "TriangleWithoutIndices/TriangleWithoutIndices.gltf";

fn main() {
    let path = format!("{}/{}/{}", CARGO_MANIFEST_DIR, ASSETS_DIR, MODEL);

    let file = fs::File::open(path).unwrap();
    let reader = io::BufReader::new(file);
    let gltf = gltf_kun::Gltf::from_reader(reader).unwrap();

    println!("{:#?}", gltf);
}
