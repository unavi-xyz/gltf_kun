use bevy::prelude::*;
use gltf_kun::extensions::omi_physics_body::OMIPhysicsBody;

use crate::export::{extensions::BevyExtensionExport, gltf::ExportContext};

impl BevyExtensionExport for OMIPhysicsBody {
    fn bevy_export(context: &mut ExportContext) {
        info!("Exporting OMI Physics Body");
    }
}

