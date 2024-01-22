use gltf_kun::extensions::omi_physics_body::OMIPhysicsBody;

use crate::export::{extensions::BevyExtensionExport, gltf::ExportContext};

impl BevyExtensionExport for OMIPhysicsBody {
    fn bevy_export(&self, context: &mut ExportContext) {
        println!("Exporting OMI Physics Body");
    }
}

