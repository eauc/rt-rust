use rand::prelude::*;

pub type Float = f32;

pub const EPSILON: Float = 0.0001;
pub const PI: Float = std::f32::consts::PI;
pub const SQRT_2: Float = std::f32::consts::SQRT_2;
#[allow(clippy::excessive_precision)]
pub const SQRT_3: Float = 1.732050807568877293527446341505872367_f32;

pub fn equals(a: Float, b: Float) -> bool {
    (a - b).abs() < EPSILON
}

pub fn rand(magnitude: Float) -> Float {
    magnitude * rand::rng().random_range(-1.0..1.0)
}
