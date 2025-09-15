use crate::colors::Color;
use crate::tuples::Tuple;

#[derive(Debug, Clone)]
pub struct StripePattern {
    a: Color,
    b: Color,
}

impl StripePattern {
    pub fn new(a: Color, b: Color) -> StripePattern {
        StripePattern {
            a,
            b,
        }
    }

    pub fn color_at(&self, point: Tuple) -> Color {
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
}
