use gltf_kun::{
    extensions::DefaultExtensions,
    graph::{gltf::document::GltfDocument, Graph},
};

#[cfg(feature = "omi_physics_body")]
pub mod omi_physics_body;

pub trait BevyExtensionIO {
    fn import_bevy(graph: &mut Graph, doc: &mut GltfDocument);
}

impl BevyExtensionIO for DefaultExtensions {
    fn import_bevy(graph: &mut Graph, doc: &mut GltfDocument) {
        todo!()
    }
}
