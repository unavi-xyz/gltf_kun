use bevy::prelude::*;

#[cfg(feature = "omi_physics")]
pub mod omi_physics;

pub struct ExtensionsPlugin;

impl Plugin for ExtensionsPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "omi_physics")]
        app.add_plugins(omi_physics::OMIPhysicsPlugin);
    }
}