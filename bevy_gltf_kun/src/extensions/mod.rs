use gltf_kun::{
    extensions::DefaultExtensions,
    graph::{gltf::document::GltfDocument, Graph},
};

#[cfg(feature = "omi_physics_body")]
pub mod omi_physics_body;

pub trait BevyExtensionIO<D> {
    fn import_bevy(graph: &mut Graph, doc: &mut D);
}

impl BevyExtensionIO<GltfDocument> for DefaultExtensions {
    fn import_bevy(graph: &mut Graph, doc: &mut GltfDocument) {
        todo!()
    }
}
