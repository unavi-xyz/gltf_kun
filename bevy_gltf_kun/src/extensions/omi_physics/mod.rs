use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;
use gltf_kun::{
    extensions::{
        omi_physics_body::{BodyType, OMIPhysicsBody},
        omi_physics_shape::physics_shape::{
            BoxShape, CapsuleShape, CylinderShape, PhysicsShapeWeight, SphereShape,
        },
    },
    graph::{gltf::document::GltfDocument, ByteNode},
};

use crate::import::gltf::document::ImportContext;

use super::NodeExtensionImport;

pub struct OMIPhysicsPlugin;

impl Plugin for OMIPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ColliderMarker>()
            .register_type::<RigidBodyMarker>()
            .add_systems(Update, (insert_colliders, insert_rigid_bodies));
    }
}

/// Mark a collider to be added to the entity after asset loading.
/// This is needed because `Collider` doesn't implement `Reflect`.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub enum ColliderMarker {
    Cuboid([f32; 3]),
    Sphere(f32),
    Capsule(f32, f32),
    Cylinder(f32, f32),
}

impl Default for ColliderMarker {
    fn default() -> Self {
        Self::Sphere(0.0)
    }
}

pub fn insert_colliders(mut commands: Commands, mut query: Query<(Entity, &ColliderMarker)>) {
    for (entity, marker) in query.iter_mut() {
        let collider = match marker {
            ColliderMarker::Cuboid(size) => Collider::cuboid(size[0], size[1], size[2]),
            ColliderMarker::Sphere(radius) => Collider::ball(*radius),
            ColliderMarker::Capsule(radius, height) => Collider::capsule(*radius, *height),
            ColliderMarker::Cylinder(radius, height) => Collider::cylinder(*radius, *height),
        };

        commands
            .entity(entity)
            .remove::<ColliderMarker>()
            .insert(collider);
    }
}

pub fn insert_rigid_bodies(mut commands: Commands, mut query: Query<(Entity, &RigidBodyMarker)>) {
    for (entity, marker) in query.iter_mut() {
        let rigid_body = match marker.typ {
            RigidBodyType::Static => RigidBody::Static,
            RigidBodyType::Dynamic => RigidBody::Dynamic,
            RigidBodyType::Kinematic => RigidBody::Kinematic,
        };

        commands
            .entity(entity)
            .remove::<RigidBodyMarker>()
            .insert(rigid_body)
            .insert(LinearVelocity(marker.linear_velocity))
            .insert(AngularVelocity(marker.angular_velocity))
            .insert(MassPropertiesBundle {
                mass: Mass(marker.mass),
                center_of_mass: CenterOfMass(marker.center_of_mass),
                inertia: marker.inertia,
                ..default()
            });
    }
}

/// Mark a rigid body to be added to the entity after asset loading.
/// While not necessary on its own, we want to spawn in the rigid body and collider at the same time.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct RigidBodyMarker {
    angular_velocity: Vec3,
    center_of_mass: Vec3,
    inertia: Inertia,
    linear_velocity: Vec3,
    mass: f32,
    typ: RigidBodyType,
}

impl Default for RigidBodyMarker {
    fn default() -> Self {
        Self {
            angular_velocity: Vec3::ZERO,
            center_of_mass: Vec3::ZERO,
            inertia: Inertia::ZERO,
            linear_velocity: Vec3::ZERO,
            mass: 1.0,
            typ: RigidBodyType::Dynamic,
        }
    }
}

#[derive(Reflect)]
pub enum RigidBodyType {
    Static,
    Dynamic,
    Kinematic,
}

impl NodeExtensionImport<GltfDocument> for OMIPhysicsBody {
    fn import_node(context: &mut ImportContext, entity: &mut EntityWorldMut, ext: Self) {
        if let Some(collider) = ext.collider(context.graph) {
            let weight = collider.read(context.graph);

            match weight {
                PhysicsShapeWeight::Box(BoxShape { size }) => {
                    entity.insert(ColliderMarker::Cuboid(size.0));
                }
                PhysicsShapeWeight::Sphere(SphereShape { radius }) => {
                    entity.insert(ColliderMarker::Sphere(radius.0));
                }
                PhysicsShapeWeight::Capsule(CapsuleShape { radius, height }) => {
                    entity.insert(ColliderMarker::Capsule(radius.0, height.0));
                }
                PhysicsShapeWeight::Cylinder(CylinderShape { radius, height }) => {
                    entity.insert(ColliderMarker::Cylinder(radius.0, height.0));
                }
                PhysicsShapeWeight::Convex => {
                    warn!("Convex shapes are not supported yet");
                    return;
                }
                PhysicsShapeWeight::Trimesh => {
                    warn!("Trimesh shapes are not supported yet");
                    return;
                }
            }
        }

        let weight = ext.read(context.graph);

        if let Some(motion) = weight.motion {
            let typ = match motion.typ {
                BodyType::Static => RigidBodyType::Static,
                BodyType::Dynamic => RigidBodyType::Dynamic,
                BodyType::Kinematic => RigidBodyType::Kinematic,
            };

            let inertia = Mat3::from_diagonal(motion.intertial_diagonal.into());
            let inertia = Inertia(inertia);

            let rotation = motion.inertia_orientation.0;
            let rotation = Quat::from_xyzw(rotation[0], rotation[1], rotation[2], rotation[3]);
            inertia.rotated(&rotation.into());

            entity.insert(RigidBodyMarker {
                angular_velocity: motion.angular_velocity.into(),
                center_of_mass: motion.center_of_mass.into(),
                inertia,
                linear_velocity: motion.linear_velocity.into(),
                mass: motion.mass.0,
                typ,
            });
        }
    }
}
