use bevy::asset::LoadContext;
use gltf_kun::{
    extensions::{
        omi_physics_body::OMIPhysicsBody, omi_physics_shape::OMIPhysicsShape, DefaultExtensions,
    },
    graph::{gltf::document::GltfDocument, Graph},
};

#[cfg(feature = "omi_physics_body")]
pub mod omi_physics_body;
#[cfg(feature = "omi_physics_shape")]
pub mod omi_physics_shape;

pub trait BevyExtensionImport<D> {
    fn import_bevy(graph: &mut Graph, doc: &mut D, load_context: &mut LoadContext);
}

pub trait BevyExtensionExport<D> {
    fn export_bevy(graph: &mut Graph, doc: &mut D, load_context: &mut LoadContext);
}

pub trait BevyExtensionsIO<D> {
    fn import_bevy(graph: &mut Graph, doc: &mut D, load_context: &mut LoadContext);
    fn export_bevy(graph: &mut Graph, doc: &mut D, load_context: &mut LoadContext);
}

impl BevyExtensionsIO<GltfDocument> for DefaultExtensions {
    fn import_bevy(graph: &mut Graph, doc: &mut GltfDocument, load_context: &mut LoadContext) {
        #[cfg(feature = "omi_physics_shape")]
        OMIPhysicsShape::import_bevy(graph, doc, load_context);
        #[cfg(feature = "omi_physics_body")]
        OMIPhysicsBody::import_bevy(graph, doc, load_context);
    }

    fn export_bevy(graph: &mut Graph, doc: &mut GltfDocument, load_context: &mut LoadContext) {
        #[cfg(feature = "omi_physics_shape")]
        OMIPhysicsShape::export_bevy(graph, doc, load_context);
        #[cfg(feature = "omi_physics_body")]
        OMIPhysicsBody::export_bevy(graph, doc, load_context);
    }
}
