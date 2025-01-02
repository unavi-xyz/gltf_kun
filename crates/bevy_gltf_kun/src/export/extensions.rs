use bevy::prelude::*;
use gltf_kun::{extensions::DefaultExtensions, graph::gltf::GltfDocument};

use super::gltf::ExportContext;

pub trait BevyExtensionExport<D>: Send + Sync + 'static {
    fn bevy_export(context: In<ExportContext>, world: &mut World) -> ExportContext;
}

impl BevyExtensionExport<GltfDocument> for DefaultExtensions {
    fn bevy_export(In(mut context): In<ExportContext>, world: &mut World) -> ExportContext {
        #[cfg(feature = "omi_physics")]
        {
            use bevy::ecs::system::RunSystemOnce;

            context = world
                .run_system_once_with(
                    context,
                    gltf_kun::extensions::omi_physics_body::OmiPhysicsBody::bevy_export,
                )
                .expect("omi physics");
        }

        context
    }
}
