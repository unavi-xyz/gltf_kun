use bevy::prelude::*;

#[cfg(feature = "export")]
pub mod export;
#[cfg(feature = "import")]
pub mod import;

pub struct OmiPhysicsPlugin;

impl Plugin for OmiPhysicsPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "import")]
        {
            app.register_type::<import::ColliderMarker>()
                .register_type::<import::RigidBodyMarker>()
                .add_systems(
                    Update,
                    (import::insert_colliders, import::insert_rigid_bodies),
                );
        }
    }
}
