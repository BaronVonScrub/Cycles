use std::any::Any;
use std::fmt::{Debug, Formatter};
use std::ops::{Add, Div, Mul, Neg, Sub};
use bevy::math::{Vec3, VectorSpace};
use bevy::prelude::{FromReflect, Reflect, Transform, TypePath};
use bevy::reflect::{ApplyError, DynamicStruct, FieldIter, GetTypeRegistration, ReflectMut, ReflectOwned, ReflectRef, Struct, Typed, TypeInfo, TypeRegistration};

// Wrapper type for Transform
#[derive(Clone, Reflect)]
pub struct VSTransform(pub(crate) Transform);

impl Div<f32> for VSTransform {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        VSTransform(Transform {
            translation: self.0.translation / rhs,
            rotation: self.0.rotation / rhs, // For simplicity, assuming rotation can be divided by scalar.
            scale: self.0.scale / rhs,
        })
    }
}

impl Add for VSTransform {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        VSTransform(Transform {
            translation: self.0.translation + rhs.0.translation,
            rotation: self.0.rotation * rhs.0.rotation, // Combining rotations.
            scale: self.0.scale + rhs.0.scale,
        })
    }
}

impl Sub for VSTransform {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        VSTransform(Transform {
            translation: self.0.translation - rhs.0.translation,
            rotation: self.0.rotation * rhs.0.rotation.conjugate(),
            scale: self.0.scale - rhs.0.scale,
        })
    }
}

impl Neg for VSTransform {
    type Output = Self;

    fn neg(self) -> Self::Output {
        VSTransform(Transform {
            translation: -self.0.translation,
            rotation: self.0.rotation.conjugate(), // Conjugate for negating rotation.
            scale: -self.0.scale,
        })
    }
}

impl Default for VSTransform {
    fn default() -> Self {
        VSTransform(Transform::IDENTITY)
    }
}

impl Debug for VSTransform {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VSTransform")
            .field("translation", &self.0.translation)
            .field("rotation", &self.0.rotation)
            .field("scale", &self.0.scale)
            .finish()
    }
}

impl Mul<f32> for VSTransform {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        VSTransform(Transform {
            translation: self.0.translation * rhs,
            rotation: self.0.rotation * rhs, // Assuming scalar multiplication for rotation.
            scale: self.0.scale * rhs,
        })
    }
}

impl Copy for VSTransform {}

impl VectorSpace for VSTransform {

    const ZERO: Self = VSTransform(Transform::IDENTITY);

    fn lerp(&self, other: VSTransform, t: f32) -> Self {
        let translation = self.0.translation.lerp(other.0.translation, t);
        let rotation = self.0.rotation.slerp(other.0.rotation, t);
        let scale = elerp(self.0.scale, other.0.scale, t);

        VSTransform(
            Transform{
                translation,
                rotation,
                scale,
            }
        )
    }
}

fn elerp(v1: Vec3, v2: Vec3, t: f32) -> Vec3 {
    let x_factor_log = (1. - t) * v1.x.log2() + t * v2.x.log2();
    let y_factor_log = (1. - t) * v1.y.log2() + t * v2.y.log2();
    let z_factor_log = (1. - t) * v1.z.log2() + t * v2.z.log2();

    Vec3::new(
        x_factor_log.exp2(),
        y_factor_log.exp2(),
        z_factor_log.exp2(),
    )
}

impl From<Transform> for VSTransform {
    fn from(transform: Transform) -> Self {
        VSTransform(transform)
    }
}

impl Into<Transform> for VSTransform {
    fn into(self) -> Transform {
        self.0
    }
}