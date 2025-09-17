use crate::floats::{Float, equals, rand};
use std::{cmp, ops};

#[derive(Debug, Copy, Clone)]
pub struct Tuple(Float, Float, Float, Float);

impl Tuple {
    pub fn new(x: Float, y: Float, z: Float, w: Float) -> Tuple {
        Tuple(x, y, z, w)
    }
    pub fn point(x: Float, y: Float, z: Float) -> Tuple {
        Tuple(x, y, z, 1.0)
    }
    pub fn vector(x: Float, y: Float, z: Float) -> Tuple {
        Tuple(x, y, z, 0.0)
    }
    pub fn random_vector(magnitude: Float) -> Tuple {
        Tuple::vector(
            rand(magnitude),
            rand(magnitude),
            rand(magnitude),
        )
    }
    pub fn x(&self) -> Float {
        self.0
    }
    pub fn y(&self) -> Float {
        self.1
    }
    pub fn z(&self) -> Float {
        self.2
    }
    pub fn w(&self) -> Float {
        self.3
    }
    pub fn to_vector(&mut self) -> &Tuple {
        self.3 = 0.0;
        self
    }

    pub fn magnitude(&self) -> Float {
        ((self.x() * self.x()) + (self.y() * self.y()) + (self.z() * self.z())).sqrt()
    }

    pub fn normalize(&self) -> Tuple {
        let mag = self.magnitude();
        Tuple(
            self.x() / mag,
            self.y() / mag,
            self.z() / mag,
            self.w() / mag,
        )
    }

    pub fn dot(self, other: Tuple) -> Float {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z() + self.w() * other.w()
    }

    pub fn angle(self, other: Tuple) -> Float {
        self.normalize().dot(other.normalize()).acos()
    }

    pub fn cross(self, other: Tuple) -> Tuple {
        Self::vector(
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x(),
        )
    }

    pub fn reflect(self, normal: Tuple) -> Tuple {
        self - normal * 2.0 * self.dot(normal)
    }
}

impl cmp::PartialEq for Tuple {
    fn eq(&self, other: &Tuple) -> bool {
        equals(self.x(), other.x())
            && equals(self.y(), other.y())
            && equals(self.z(), other.z())
            && equals(self.w(), other.w())
    }
}

impl ops::Add for Tuple {
    type Output = Tuple;

    fn add(self, other: Tuple) -> Tuple {
        Tuple(
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
            self.w() + other.w(),
        )
    }
}

impl ops::Sub for Tuple {
    type Output = Tuple;

    fn sub(self, other: Tuple) -> Tuple {
        Tuple(
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z(),
            self.w() - other.w(),
        )
    }
}

impl ops::Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Tuple {
        Tuple(-self.x(), -self.y(), -self.z(), -self.w())
    }
}

impl ops::Mul<Float> for Tuple {
    type Output = Tuple;

    fn mul(self, scalar: Float) -> Tuple {
        Tuple(
            self.x() * scalar,
            self.y() * scalar,
            self.z() * scalar,
            self.w() * scalar,
        )
    }
}

impl ops::Div<Float> for Tuple {
    type Output = Tuple;

    fn div(self, scalar: Float) -> Tuple {
        Tuple(
            self.x() / scalar,
            self.y() / scalar,
            self.z() / scalar,
            self.w() / scalar,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_tuple_with_w_1_is_a_point() {
        let a = Tuple(4.3, -4.2, 3.1, 1.0);
        assert_eq!(a.x(), 4.3);
        assert_eq!(a.y(), -4.2);
        assert_eq!(a.z(), 3.1);
        assert_eq!(a.w(), 1.0);
    }

    #[test]
    fn a_tuple_with_w_0_is_a_vector() {
        let a = Tuple(4.3, -4.2, 3.1, 0.0);
        assert_eq!(a.x(), 4.3);
        assert_eq!(a.y(), -4.2);
        assert_eq!(a.z(), 3.1);
        assert_eq!(a.w(), 0.0);
    }

    #[test]
    fn point_creates_tuples_with_w_1() {
        let p = Tuple::point(4.0, -4.0, 3.0);
        assert_eq!(p.x(), 4.0);
        assert_eq!(p.y(), -4.0);
        assert_eq!(p.z(), 3.0);
        assert_eq!(p.w(), 1.0);
    }

    #[test]
    fn vector_creates_tuples_with_w_0() {
        let v = Tuple::vector(4.0, -4.0, 3.0);
        assert_eq!(v.x(), 4.0);
        assert_eq!(v.y(), -4.0);
        assert_eq!(v.z(), 3.0);
        assert_eq!(v.w(), 0.0);
    }

    #[test]
    fn adding_two_tuples() {
        let a1 = Tuple::point(3.0, -2.0, 5.0);
        let a2 = Tuple::vector(-2.0, 3.0, 1.0);
        assert_eq!(a1 + a2, Tuple::point(1.0, 1.0, 6.0));
    }

    #[test]
    fn subtracting_two_points() {
        let p1 = Tuple::point(3.0, 2.0, 1.0);
        let p2 = Tuple::point(5.0, 6.0, 7.0);
        assert_eq!(p1 - p2, Tuple::vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_a_vector_from_a_point() {
        let p = Tuple::point(3.0, 2.0, 1.0);
        let v = Tuple::vector(5.0, 6.0, 7.0);
        assert_eq!(p - v, Tuple::point(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_two_vectors() {
        let v1 = Tuple::vector(3.0, 2.0, 1.0);
        let v2 = Tuple::vector(5.0, 6.0, 7.0);
        assert_eq!(v1 - v2, Tuple::vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn negating_a_tuple() {
        let a = Tuple(1.0, -2.0, 3.0, -4.0);
        assert_eq!(-a, Tuple(-1.0, 2.0, -3.0, 4.0));
    }

    #[test]
    fn multiplying_a_tuple_by_a_scalar() {
        let a = Tuple(1.0, -2.0, 3.0, -4.0);
        assert_eq!(a * 3.5, Tuple(3.5, -7.0, 10.5, -14.0));
    }

    #[test]
    fn multiplying_a_tuple_by_a_fraction() {
        let a = Tuple(1.0, -2.0, 3.0, -4.0);
        assert_eq!(a * 0.5, Tuple(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn dividing_a_tuple_by_a_scalar() {
        let a = Tuple(1.0, -2.0, 3.0, -4.0);
        assert_eq!(a / 2.0, Tuple(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn magnitude_of_vector_1_0_0() {
        let v = Tuple::vector(1.0, 0.0, 0.0);
        assert_eq!(v.magnitude(), Float::sqrt(1.0));
    }

    #[test]
    fn magnitude_of_vector_0_1_0() {
        let v = Tuple::vector(0.0, 1.0, 0.0);
        assert_eq!(v.magnitude(), Float::sqrt(1.0));
    }

    #[test]
    fn magnitude_of_vector_0_0_1() {
        let v = Tuple::vector(0.0, 0.0, 1.0);
        assert_eq!(v.magnitude(), Float::sqrt(1.0));
    }

    #[test]
    fn magnitude_of_vector_1_2_3() {
        let v = Tuple::vector(1.0, 2.0, 3.0);
        assert_eq!(v.magnitude(), Float::sqrt(14.0));
    }

    #[test]
    fn magnitude_of_vector_neg_1_neg_2_neg_3() {
        let v = Tuple::vector(-1.0, -2.0, -3.0);
        assert_eq!(v.magnitude(), Float::sqrt(14.0));
    }

    #[test]
    fn normalizing_vector_4_0_0() {
        let v = Tuple::vector(4.0, 0.0, 0.0);
        assert_eq!(v.normalize(), Tuple::vector(1.0, 0.0, 0.0));
    }

    #[test]
    fn normalizing_vector_1_2_3() {
        let v = Tuple::vector(1.0, 2.0, 3.0);
        assert_eq!(v.normalize(), Tuple::vector(0.26726, 0.53452, 0.80178));
    }

    #[test]
    fn the_magnitude_of_a_normalized_vector() {
        let v = Tuple::vector(1.0, 2.0, 3.0);
        assert!(equals(v.normalize().magnitude(), 1.0));
    }

    #[test]
    fn the_dot_product_of_two_tuples() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);
        assert_eq!(a.dot(b), 20.0);
    }

    #[test]
    fn the_cross_product_of_two_vectors() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);
        assert_eq!(a.cross(b), Tuple::vector(-1.0, 2.0, -1.0));
        assert_eq!(b.cross(a), Tuple::vector(1.0, -2.0, 1.0));
    }

    #[test]
    fn reflecting_a_vector_approaching_at_45() {
        let v = Tuple::vector(1.0, -1.0, 0.0);
        let n = Tuple::vector(0.0, 1.0, 0.0);
        let r = v.reflect(n);
        assert_eq!(r, Tuple::vector(1.0, 1.0, 0.0));
    }

    #[test]
    fn reflecting_a_vector_off_a_slanted_surface() {
        let v = Tuple::vector(0.0, -1.0, 0.0);
        let n = Tuple::vector(2.0_f32.sqrt() / 2.0, 2.0_f32.sqrt() / 2.0, 0.0);
        let r = v.reflect(n);
        assert_eq!(r, Tuple::vector(1.0, 0.0, 0.0));
    }
}
