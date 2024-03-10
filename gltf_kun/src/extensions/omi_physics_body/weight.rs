use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct OmiPhysicsBodyWeight {
    pub motion: Option<Motion>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Motion {
    /// The type of the physics body.
    #[serde(rename = "type")]
    pub typ: BodyType,
    /// The mass of the physics body in kilograms.
    #[serde(default, skip_serializing_if = "is_default_mass")]
    pub mass: Mass,
    /// The initial linear velocity of the body in meters per second.
    #[serde(
        default,
        rename = "linearVelocity",
        skip_serializing_if = "slice_is_zero"
    )]
    pub linear_velocity: [f32; 3],
    /// The initial angular velocity of the body in radians per second.
    #[serde(
        default,
        rename = "angularVelocity",
        skip_serializing_if = "slice_is_zero"
    )]
    pub angular_velocity: [f32; 3],
    /// The center of mass offset from the origin in meters.
    #[serde(
        default,
        rename = "centerOfMass",
        skip_serializing_if = "slice_is_zero"
    )]
    pub center_of_mass: [f32; 3],
    /// The inertia around principle axes in kilogram meter squared (kg⋅m²).
    #[serde(
        default,
        rename = "inertialDiagonal",
        skip_serializing_if = "slice_is_zero"
    )]
    pub intertial_diagonal: [f32; 3],
    /// The inertia orientation as a Quaternion.
    #[serde(
        default,
        rename = "inertiaOrientation",
        skip_serializing_if = "is_default_quat"
    )]
    pub inertia_orientation: Quat,
}

#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Mass(pub f32);

impl Default for Mass {
    fn default() -> Self {
        Self(1.0)
    }
}

impl From<f32> for Mass {
    fn from(value: f32) -> Self {
        Self(value)
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Quat(pub [f32; 4]);

impl Default for Quat {
    fn default() -> Self {
        Self([0.0, 0.0, 0.0, 1.0])
    }
}

impl Motion {
    pub fn new(typ: BodyType) -> Self {
        Self {
            typ,
            angular_velocity: Default::default(),
            center_of_mass: Default::default(),
            inertia_orientation: Default::default(),
            intertial_diagonal: Default::default(),
            linear_velocity: Default::default(),
            mass: Default::default(),
        }
    }
}

impl From<&Vec<u8>> for OmiPhysicsBodyWeight {
    fn from(bytes: &Vec<u8>) -> Self {
        if bytes.is_empty() {
            return Self::default();
        }
        serde_json::from_slice(bytes).expect("Failed to deserialize weight")
    }
}

impl From<&OmiPhysicsBodyWeight> for Vec<u8> {
    fn from(value: &OmiPhysicsBodyWeight) -> Self {
        serde_json::to_vec(value).expect("Failed to serialize weight")
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub enum BodyType {
    #[serde(rename = "static")]
    Static,
    #[serde(rename = "dynamic")]
    Dynamic,
    #[serde(rename = "kinematic")]
    Kinematic,
}

fn is_default_mass(mass: &Mass) -> bool {
    mass.0 == 1.0
}

fn is_default_quat(quat: &Quat) -> bool {
    quat.0 == [0.0, 0.0, 0.0, 1.0]
}

fn float_is_zero(num: &f32) -> bool {
    *num == 0.0
}

fn slice_is_zero(slice: &[f32]) -> bool {
    slice.iter().all(float_is_zero)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_mass() {
        assert!(Mass::default().0 == 1.0);
        assert!(is_default_mass(&Mass::default()));
    }

    #[test]
    fn default_quat() {
        assert!(Quat::default().0 == [0.0, 0.0, 0.0, 1.0]);
        assert!(is_default_quat(&Quat::default()));
    }
}
