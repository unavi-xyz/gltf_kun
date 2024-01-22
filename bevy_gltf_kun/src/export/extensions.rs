use gltf_kun::{
    extensions::{omi_physics_body::OMIPhysicsBody, DefaultExtensions},
    graph::gltf::document::GltfDocument,
};

use super::gltf::ExportContext;

pub trait BevyExtensionExport {
    fn bevy_export(context: &mut ExportContext);
}

pub trait BevyExportExtensions<D> {
    fn bevy_export(context: &mut ExportContext);
}

impl BevyExportExtensions<GltfDocument> for DefaultExtensions {
    fn bevy_export(context: &mut ExportContext) {
        OMIPhysicsBody::bevy_export(context);
    }
}
