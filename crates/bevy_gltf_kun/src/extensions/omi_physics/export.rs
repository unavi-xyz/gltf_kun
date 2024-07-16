use avian3d::{parry::shape::ShapeType, prelude::*};
use bevy::{ecs::system::RunSystemOnce, prelude::*};
use gltf_kun::{
    extensions::{
        omi_physics_body::{
            weight::{BodyType, Motion},
            OmiPhysicsBody,
        },
        omi_physics_shape::{
            physics_shape::{
                BoxShape, CapsuleShape, Height, PhysicsShapeWeight, Radius, Size, SphereShape,
            },
            OmiPhysicsShape,
        },
    },
    graph::{gltf::GltfDocument, ByteNode, Extensions},
};

use crate::export::{extensions::BevyExtensionExport, gltf::ExportContext};

impl BevyExtensionExport<GltfDocument> for OmiPhysicsBody {
    fn bevy_export(In(context): In<ExportContext>, world: &mut World) -> ExportContext {
        world.run_system_once_with(context, export_physics_bodies)
    }
}

pub fn export_physics_bodies(
    In(mut context): In<ExportContext>,
    bodies: Query<(
        &RigidBody,
        &AngularVelocity,
        &LinearVelocity,
        &Mass,
        &Inertia,
        &CenterOfMass,
    )>,
    colliders: Query<&Collider>,
) -> ExportContext {
    for cached in &context.nodes {
        let body = bodies.get(cached.entity);
        let collider = colliders.get(cached.entity);

        if body.is_err() && collider.is_err() {
            continue;
        }

        let ext = match cached.node.get_extension::<OmiPhysicsBody>(&context.graph) {
            Some(ext) => ext,
            None => cached
                .node
                .create_extension::<OmiPhysicsBody>(&mut context.graph),
        };

        let shapes_ext = match context.doc.get_extension::<OmiPhysicsShape>(&context.graph) {
            Some(ext) => ext,
            None => context
                .doc
                .create_extension::<OmiPhysicsShape>(&mut context.graph),
        };

        if let Ok(body) = body {
            let (body, angular_velocity, linear_velocity, mass, _inertia, center_of_mass) = body;

            // TODO: convert inertia Mat3 to diagonal and orientation

            let mut weight = ext.read(&context.graph);

            let typ = match body {
                RigidBody::Static => BodyType::Static,
                RigidBody::Dynamic => BodyType::Dynamic,
                RigidBody::Kinematic => BodyType::Kinematic,
            };

            let mut motion = Motion::new(typ);
            motion.angular_velocity = angular_velocity.0.to_array();
            motion.linear_velocity = linear_velocity.0.to_array();
            motion.mass = mass.0.into();
            motion.center_of_mass = center_of_mass.0.to_array();
            weight.motion = Some(motion);

            ext.write(&mut context.graph, &weight);
        }

        if let Ok(collider) = collider {
            let collider_shape = collider.shape();

            let shape_weight = match collider_shape.shape_type() {
                ShapeType::Cuboid => {
                    let cuboid = collider_shape.as_cuboid().unwrap();
                    PhysicsShapeWeight::Box(BoxShape {
                        size: Size(cuboid.half_extents.map(|x| x * 2.0).into()),
                    })
                }
                ShapeType::Ball => {
                    let ball = collider_shape.as_ball().unwrap();
                    PhysicsShapeWeight::Sphere(SphereShape {
                        radius: Radius(ball.radius),
                    })
                }
                ShapeType::Capsule => {
                    let capsule = collider_shape.as_capsule().unwrap();
                    PhysicsShapeWeight::Capsule(CapsuleShape {
                        radius: Radius(capsule.radius),
                        height: Height(capsule.half_height() * 2.0),
                    })
                }
                ShapeType::Cylinder => {
                    let cylinder = collider_shape.as_cylinder().unwrap();
                    PhysicsShapeWeight::Capsule(CapsuleShape {
                        radius: Radius(cylinder.radius),
                        height: Height(cylinder.half_height * 2.0),
                    })
                }
                _ => {
                    warn!(
                        "Unsupported collider shape type: {:?}",
                        collider_shape.shape_type()
                    );
                    continue;
                }
            };

            let collider_shape = shapes_ext.create_shape(&mut context.graph, &shape_weight);
            ext.set_collider(&mut context.graph, Some(collider_shape));
        };
    }

    context
}
