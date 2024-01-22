use bevy::prelude::*;
use gltf_kun::{
    extensions::{omi_physics_body::OMIPhysicsBody, DefaultExtensions},
    graph::gltf::document::GltfDocument,
};

use super::gltf::ExportContext;

pub trait BevyExtensionExport {
    fn bevy_export(context: &mut ExportContext);
}

pub trait BevyExportExtensions<D>: Send + Sync + 'static {
    fn bevy_export(context: In<ExportContext>) -> ExportContext;
}

impl BevyExportExtensions<GltfDocument> for DefaultExtensions {
    fn bevy_export(In(mut context): In<ExportContext>) -> ExportContext {
        OMIPhysicsBody::bevy_export(&mut context);

        context
    }
}
