use std::path::Path;

use gltf_kun::{
    extensions::{
        omi_physics_body::{weight::BodyType, OmiPhysicsBody},
        omi_physics_shape::{
            physics_shape::{BoxShape, PhysicsShapeWeight, Size},
            OmiPhysicsShape,
        },
        DefaultExtensions, Extension,
    },
    graph::{gltf::document::GltfDocument, ByteNode, Graph, Property},
    io::format::gltf::{GltfExport, GltfImport},
};
use tracing_test::traced_test;

const ASSETS_DIR: &str = "../assets";
const CARGO_MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");
const MODEL: &str = "DynamicBox.gltf";

#[tokio::test]
#[traced_test]
async fn main() {
    let assets = Path::new(CARGO_MANIFEST_DIR).join(ASSETS_DIR);
    let path = assets.join(MODEL);

    // Import
    let mut graph = Graph::default();
    let doc = GltfImport::<DefaultExtensions>::import_file(&mut graph, &path)
        .await
        .expect("Failed to import glTF");

    validate_doc(&graph, &doc);

    // Export to file
    let out =
        GltfExport::<DefaultExtensions>::export(&mut graph, &doc).expect("Failed to export glTF");

    let json = serde_json::to_value(&out.json).expect("Failed to serialize glTF");
    validate_json(&json);

    let path = assets.join("temp/physics_extensions/model.gltf");
    std::fs::create_dir_all(path.parent().unwrap()).expect("Failed to create directory");
    out.write_file(&path).expect("Failed to write glTF to file");

    // Import written file
    let mut graph = Graph::default();
    let doc = GltfImport::<DefaultExtensions>::import_file(&mut graph, &path)
        .await
        .expect("Failed to import glTF");

    validate_doc(&graph, &doc);
}

fn validate_doc(graph: &Graph, doc: &GltfDocument) {
    let shape_ext = doc
        .get_extension::<OmiPhysicsShape>(graph)
        .expect("OMI_physics_shape extension not found");

    assert_eq!(shape_ext.shapes(graph).count(), 1);

    let shape = shape_ext.shapes(graph).next().expect("No shape");
    match shape.read(graph) {
        PhysicsShapeWeight::Box(BoxShape { size }) => {
            assert_eq!(size, Size([1.0, 2.0, 3.0]));
        }
        _ => panic!("Invalid shape"),
    }

    for (i, node) in doc.nodes(graph).iter().enumerate() {
        let body_ext = node
            .get_extension::<OmiPhysicsBody>(graph)
            .expect("OMI_physics_body extension not found");

        if i == 0 {
            let weight = body_ext.read(graph);
            let motion = weight.motion.expect("No motion");

            assert_eq!(motion.typ, BodyType::Dynamic);
        } else {
            let collider = body_ext.collider(graph).expect("No collider");
            assert_eq!(collider, shape);
        }
    }
}

fn validate_json(json: &serde_json::Value) {
    let extensions_used = json
        .get("extensionsUsed")
        .expect("No extensionsUsed")
        .as_array()
        .expect("extensionsUsed is not an array");

    assert!(
        extensions_used.contains(&serde_json::Value::String(
            OmiPhysicsBody::name().to_string()
        )),
        "OMI_physics_body extension not found in extensionsUsed"
    );
    assert!(
        extensions_used.contains(&serde_json::Value::String(
            OmiPhysicsShape::name().to_string()
        )),
        "OMI_physics_shape extension not found in extensionsUsed"
    );
}
