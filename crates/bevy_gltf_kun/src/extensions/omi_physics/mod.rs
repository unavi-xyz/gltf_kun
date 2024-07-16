use bevy::prelude::*;

pub mod export;
pub mod import;

pub struct OmiPhysicsPlugin;

impl Plugin for OmiPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<import::ColliderMarker>()
            .register_type::<import::RigidBodyMarker>()
            .add_systems(
                Update,
                (import::insert_colliders, import::insert_rigid_bodies),
            );
    }
}
