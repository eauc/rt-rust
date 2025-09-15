pub type Float = f32;

pub const EPSILON: Float = 0.0001;

pub fn equals(a: Float, b: Float) -> bool {
    (a - b).abs() < EPSILON
}
