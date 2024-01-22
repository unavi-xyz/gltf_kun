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

impl NodeExtensionImport<GltfDocument> for OMIPhysicsBody {
    fn import_node(context: &mut ImportContext, entity: &mut EntityWorldMut, ext: Self) {
        if let Some(collider) = ext.collider(context.graph) {
            let weight = collider.read(context.graph);

            match weight {
                PhysicsShapeWeight::Box(BoxShape { size }) => {
                    entity.insert(Collider::cuboid(size.0[0], size.0[1], size.0[2]));
                }
                PhysicsShapeWeight::Sphere(SphereShape { radius }) => {
                    entity.insert(Collider::ball(radius.0));
                }
                PhysicsShapeWeight::Capsule(CapsuleShape { radius, height }) => {
                    entity.insert(Collider::capsule(radius.0, height.0));
                }
                PhysicsShapeWeight::Cylinder(CylinderShape { radius, height }) => {
                    entity.insert(Collider::cylinder(radius.0, height.0));
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
            match motion.typ {
                BodyType::Static => {
                    entity.insert(RigidBody::Static);
                }
                BodyType::Dynamic => {
                    entity.insert(RigidBody::Dynamic);
                }
                BodyType::Kinematic => {
                    entity.insert(RigidBody::Kinematic);
                }
            }

            entity.insert(Mass(motion.mass.0));
            entity.insert(LinearVelocity(motion.linear_velocity.into()));
            entity.insert(AngularVelocity(motion.angular_velocity.into()));
        }
    }
}
