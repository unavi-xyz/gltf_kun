use bevy::{ecs::system::RunSystemOnce, prelude::*};
use gltf_kun::{extensions::DefaultExtensions, graph::gltf::GltfDocument};

use super::gltf::ExportContext;

pub trait BevyExtensionExport {
    fn bevy_export(context: In<ExportContext>, world: &mut World) -> ExportContext;
}

pub trait BevyExportExtensions<D>: Send + Sync + 'static {
    fn bevy_export(context: In<ExportContext>, world: &mut World) -> ExportContext;
}

impl BevyExportExtensions<GltfDocument> for DefaultExtensions {
    fn bevy_export(In(context): In<ExportContext>, world: &mut World) -> ExportContext {
        #[cfg(feature = "omi_physics")]
        {
            world.run_system_once_with(
                context,
                gltf_kun::extensions::omi_physics_body::OMIPhysicsBody::bevy_export,
            )
        }
    }
}
