use crate::colors::Color;
use crate::matrices::Matrix;
use crate::objects::Object;
use crate::tuples::Tuple;

mod checkers;
mod gradients;
mod rings;
mod stripes;

#[derive(Debug, Clone, Copy)]
pub struct Pattern {
    pattern: Patterns,
    transform_inverse: Matrix<4>,
}

impl Pattern {
    fn new(pattern: Patterns) -> Pattern {
        Pattern {
            pattern,
            transform_inverse: Matrix::identity(),
        }
    }

    pub fn new_checker(a: Color, b: Color) -> Pattern {
        Pattern::new(Patterns::Checker(checkers::CheckerPattern::new(a, b)))
    }
    pub fn new_gradient(a: Color, b: Color) -> Pattern {
        Pattern::new(Patterns::Gradient(gradients::GradientPattern::new(a, b)))
    }
    pub fn new_ring(a: Color, b: Color) -> Pattern {
        Pattern::new(Patterns::Ring(rings::RingPattern::new(a, b)))
    }
    pub fn new_stripe(a: Color, b: Color) -> Pattern {
        Pattern::new(Patterns::Stripe(stripes::StripePattern::new(a, b)))
    }
    pub fn new_test() -> Pattern {
        Pattern::new(Patterns::Test(TestPattern))
    }

    pub fn with_transform(self, transform: Matrix<4>) -> Pattern {
        Pattern {
            transform_inverse: transform.inverse(),
            ..self
        }
    }

    pub fn color_at_object(&self, object: &Object, world_point: Tuple) -> Color {
        let object_point = object.world_to_object(world_point);
        let pattern_point = self.transform_inverse * object_point;
        self.pattern.color_at(pattern_point)
    }
}

#[derive(Debug, Clone, Copy)]
enum Patterns {
    Checker(checkers::CheckerPattern),
    Gradient(gradients::GradientPattern),
    Ring(rings::RingPattern),
    Stripe(stripes::StripePattern),
    Test(TestPattern),
}

impl Patterns {
    fn color_at(&self, point: Tuple) -> Color {
        match *self {
            Patterns::Checker(ref pattern) => pattern.color_at(point),
            Patterns::Stripe(ref pattern) => pattern.color_at(point),
            Patterns::Gradient(ref pattern) => pattern.color_at(point),
            Patterns::Ring(ref pattern) => pattern.color_at(point),
            Patterns::Test(ref pattern) => pattern.color_at(point),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct TestPattern;

impl TestPattern {
    fn color_at(&self, point: Tuple) -> Color {
        Color::new(point.x(), point.y(), point.z())
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::transformations::{scaling, translation};
    use crate::tuples::Tuple;

    #[test]
    fn a_pattern_with_an_object_transformation() {
        let object = Object::new_sphere().with_transform(scaling(2.0, 2.0, 2.0));
        let pattern = Pattern::new(Patterns::Test(TestPattern));
        let c = pattern.color_at_object(&object, Tuple::point(2.0, 3.0, 4.0));
        assert_eq!(c, Color::new(1.0, 1.5, 2.0));
    }

    #[test]
    fn a_pattern_with_a_pattern_transformation() {
        let object = Object::new_sphere();
        let pattern =
            Pattern::new(Patterns::Test(TestPattern)).with_transform(scaling(2.0, 2.0, 2.0));
        let c = pattern.color_at_object(&object, Tuple::point(2.0, 3.0, 4.0));
        assert_eq!(c, Color::new(1.0, 1.5, 2.0));
    }

    #[test]
    fn a_pattern_with_both_an_object_and_a_pattern_transformation() {
        let object = Object::new_sphere().with_transform(scaling(2.0, 2.0, 2.0));
        let pattern =
            Pattern::new(Patterns::Test(TestPattern)).with_transform(translation(0.5, 1.0, 1.5));
        let c = pattern.color_at_object(&object, Tuple::point(2.5, 3.0, 3.5));
        assert_eq!(c, Color::new(0.75, 0.5, 0.25));
    }
}
