use bevy::prelude::*;

pub use gltf_kun::extensions::DefaultExtensions;

#[cfg(feature = "omi_physics")]
pub mod omi_physics;

/// Adds support for extensions.
pub struct ExtensionsPlugin;

impl Plugin for ExtensionsPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "omi_physics")]
        app.add_plugins(omi_physics::OmiPhysicsPlugin);
    }
}
