use crate::colors::Color;
use crate::matrices::Matrix;
use crate::shapes::Shape;
use crate::tuples::Tuple;

pub mod checkers;
pub mod gradients;
pub mod rings;
pub mod stripes;

pub trait Pattern {
    fn transform_inverse(&self) -> Matrix<4>;
    fn color_at(&self, point: Tuple) -> Color;
}

pub fn color_at_object(pattern: &dyn Pattern, object: &dyn Shape, world_point: Tuple) -> Color {
    let object_point = object.transform_inverse() * world_point;
    let pattern_point = pattern.transform_inverse() * object_point;
    pattern.color_at(pattern_point)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::spheres::Sphere;
    use crate::transformations::{scaling, translation};
    use crate::tuples::Tuple;

    pub struct TestPattern {
        transform_inverse: Matrix<4>,
    }

    impl TestPattern {
        pub fn new(transform: Matrix<4>) -> TestPattern {
            TestPattern {
                transform_inverse: transform.inverse(),
            }
        }
    }

    impl Pattern for TestPattern {
        fn transform_inverse(&self) -> Matrix<4> {
            self.transform_inverse
        }
        fn color_at(&self, point: Tuple) -> Color {
            Color::new(point.x(), point.y(), point.z())
        }
    }

    #[test]
    fn the_default_pattern_transformation_is_the_identity_matrix() {
        let pattern = TestPattern::new(Matrix::identity());
        assert_eq!(pattern.transform_inverse(), Matrix::identity());
    }

    #[test]
    fn assigning_a_transformation() {
        let pattern = TestPattern::new(translation(1.0, 2.0, 3.0));
        assert_eq!(
            pattern.transform_inverse(),
            translation(1.0, 2.0, 3.0).inverse()
        );
    }

    #[test]
    fn a_pattern_with_an_object_transformation() {
        let object = Sphere::new(scaling(2.0, 2.0, 2.0));
        let pattern = TestPattern::new(Matrix::identity());
        let c = color_at_object(&pattern, &object, Tuple::point(2.0, 3.0, 4.0));
        assert_eq!(c, Color::new(1.0, 1.5, 2.0));
    }

    #[test]
    fn a_pattern_with_a_pattern_transformation() {
        let object = Sphere::default();
        let pattern = TestPattern::new(scaling(2.0, 2.0, 2.0));
        let c = color_at_object(&pattern, &object, Tuple::point(2.0, 3.0, 4.0));
        assert_eq!(c, Color::new(1.0, 1.5, 2.0));
    }

    #[test]
    fn a_pattern_with_both_an_object_and_a_pattern_transformation() {
        let object = Sphere::new(scaling(2.0, 2.0, 2.0));
        let pattern = TestPattern::new(translation(0.5, 1.0, 1.5));
        let c = color_at_object(&pattern, &object, Tuple::point(2.5, 3.0, 3.5));
        assert_eq!(c, Color::new(0.75, 0.5, 0.25));
    }
}
