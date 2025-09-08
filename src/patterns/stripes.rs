use crate::colors::Color;
use crate::matrices::Matrix;
use crate::patterns::Pattern;
use crate::tuples::Tuple;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StripePattern {
    a: Color,
    b: Color,
    transform_inverse: Matrix<4>,
}

impl StripePattern {
    pub fn new(a: Color, b: Color) -> StripePattern {
        StripePattern {
            a,
            b,
            transform_inverse: Matrix::identity(),
        }
    }

    pub fn set_transform(&mut self, transform: Matrix<4>) {
        self.transform_inverse = transform.inverse();
    }
}

impl Pattern for StripePattern {
    fn transform_inverse(&self) -> Matrix<4> {
        self.transform_inverse
    }

    fn color_at(&self, point: Tuple) -> Color {
        if point.x().floor() % 2.0 == 0.0 {
            self.a
        } else {
            self.b
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::colors::{BLACK, WHITE};
    use crate::patterns::color_at_object;
    use crate::spheres::Sphere;
    use crate::transformations::{scaling, translation};

    #[test]
    fn a_stripe_pattern_is_constant_in_y() {
        let pattern = StripePattern::new(WHITE, BLACK);
        assert_eq!(pattern.color_at(Tuple::point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.color_at(Tuple::point(0.0, 1.0, 0.0)), WHITE);
        assert_eq!(pattern.color_at(Tuple::point(0.0, 2.0, 0.0)), WHITE);
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_z() {
        let pattern = StripePattern::new(WHITE, BLACK);
        assert_eq!(pattern.color_at(Tuple::point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.color_at(Tuple::point(0.0, 0.0, 1.0)), WHITE);
        assert_eq!(pattern.color_at(Tuple::point(0.0, 0.0, 2.0)), WHITE);
    }

    #[test]
    fn a_stripe_pattern_alternates_in_x() {
        let pattern = StripePattern::new(WHITE, BLACK);
        assert_eq!(pattern.color_at(Tuple::point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.color_at(Tuple::point(0.9, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.color_at(Tuple::point(1.0, 0.0, 0.0)), BLACK);
        assert_eq!(pattern.color_at(Tuple::point(-0.1, 0.0, 0.0)), BLACK);
        assert_eq!(pattern.color_at(Tuple::point(-1.0, 0.0, 0.0)), BLACK);
        assert_eq!(pattern.color_at(Tuple::point(-1.1, 0.0, 0.0)), WHITE);
    }

    #[test]
    fn stripes_with_an_object_transformation() {
        let object = Sphere::new(scaling(2.0, 2.0, 2.0));
        let pattern = StripePattern::new(WHITE, BLACK);
        let c = color_at_object(&pattern, &object, Tuple::point(1.5, 0.0, 0.0));
        assert_eq!(c, WHITE);
    }

    #[test]
    fn stripes_with_a_pattern_transformation() {
        let object = Sphere::default();
        let mut pattern = StripePattern::new(WHITE, BLACK);
        pattern.set_transform(scaling(2.0, 2.0, 2.0));
        let c = color_at_object(&pattern, &object, Tuple::point(1.5, 0.0, 0.0));
        assert_eq!(c, WHITE);
    }

    #[test]
    fn stripes_with_both_an_object_and_a_pattern_transformation() {
        let object = Sphere::new(scaling(2.0, 2.0, 2.0));
        let mut pattern = StripePattern::new(WHITE, BLACK);
        pattern.set_transform(translation(0.5, 0.0, 0.0));
        let c = color_at_object(&pattern, &object, Tuple::point(2.5, 0.0, 0.0));
        assert_eq!(c, WHITE);
    }
}
