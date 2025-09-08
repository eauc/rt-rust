pub type Coordinate = f32;

pub const EPSILON: Coordinate = 0.00001;

pub fn equals(a: Coordinate, b: Coordinate) -> bool {
    (a - b).abs() < EPSILON
}
