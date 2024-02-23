use bevy::prelude::*;
use bevy_xpbd_3d::components::RigidBody;

#[cfg(feature = "export")]
pub mod export;
#[cfg(feature = "import")]
pub mod import;

pub struct OMIPhysicsPlugin;

impl Plugin for OMIPhysicsPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "import")]
        {
            app.register_type::<import::ColliderMarker>()
                .register_type::<import::RigidBodyMarker>()
                .register_type::<RigidBody>()
                .add_systems(
                    Update,
                    (import::insert_colliders, import::insert_rigid_bodies),
                );
        }
    }
}
