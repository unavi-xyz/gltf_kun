use std::error::Error;

use glam::{Quat, Vec3};
use petgraph::stable_graph::NodeIndex;

use crate::{
    document::GltfDocument,
    graph::gltf::{node::Node, GltfGraph, Weight},
    io::format::gltf::GltfFormat,
};

use super::{ExtensionIO, ExtensionProperty};

pub const EXTENSION_NAME: &str = "OMI_physics_body";

#[derive(Clone, Debug)]
pub struct OMIPhysicsBodyExtension;

impl OMIPhysicsBodyExtension {
    pub fn create_body(graph: &mut GltfGraph, node: &Node) -> PhysicsBody {
        let weight = Box::<PhysicsBodyWeight>::default();
        let index = graph.add_node(Weight::ExtensionProperty(weight));

        PhysicsBody(index)
    }
}

impl ExtensionIO<GltfDocument, GltfFormat> for OMIPhysicsBodyExtension {
    fn name(&self) -> &'static str {
        EXTENSION_NAME
    }

    fn export(
        &self,
        doc: &mut GltfDocument,
        format: &mut GltfFormat,
    ) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn import(
        &self,
        format: &mut GltfFormat,
        doc: &mut GltfDocument,
    ) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}

#[derive(Debug, Default)]
pub struct PhysicsBodyWeight {
    pub motion: Option<Motion>,
}

impl ExtensionProperty for PhysicsBodyWeight {
    fn extension_name(&self) -> &'static str {
        EXTENSION_NAME
    }
}

#[derive(Debug)]
pub struct Motion {
    /// The type of the physics body.
    pub typ: BodyType,
    /// The mass of the physics body in kilograms.
    pub mass: f32,
    /// The initial linear velocity of the body in meters per second.
    pub linear_velocity: Vec3,
    /// The initial angular velocity of the body in radians per second.
    pub angular_velocity: Vec3,
    /// The center of mass offset from the origin in meters.
    pub center_of_mass: Vec3,
    /// The inertia around principle axes in kilogram meter squared (kg⋅m²).
    pub intertial_diagonal: Vec3,
    /// The inertia orientation as a Quaternion.
    pub inertia_orientation: Quat,
}

impl Motion {
    pub fn new(typ: BodyType) -> Self {
        Self {
            typ,
            mass: 0.0,
            linear_velocity: Vec3::ZERO,
            angular_velocity: Vec3::ZERO,
            center_of_mass: Vec3::ZERO,
            intertial_diagonal: Vec3::ZERO,
            inertia_orientation: Quat::IDENTITY,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BodyType {
    Static,
    Dynamic,
    Kinematic,
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PhysicsBody(pub NodeIndex);
