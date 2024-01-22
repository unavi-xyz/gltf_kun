use bevy::prelude::*;
use gltf_kun::extensions::omi_physics_body::OMIPhysicsBody;

use crate::export::{extensions::BevyExtensionExport, gltf::ExportContext};

impl BevyExtensionExport for OMIPhysicsBody {
    fn bevy_export(In(context): In<ExportContext>) -> ExportContext {
        info!("Exporting OMI Physics Body");

        context
    }
}
