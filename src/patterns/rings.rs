use crate::colors::Color;
use crate::tuples::Tuple;

#[derive(Debug, Clone)]
pub struct RingPattern {
    a: Color,
    b: Color,
}

impl RingPattern {
    pub fn new(a: Color, b: Color) -> RingPattern {
        RingPattern {
            a,
            b,
        }
    }

    pub fn color_at(&self, point: Tuple) -> Color {
        let r = (point.x().powi(2) + point.z().powi(2)).sqrt().floor() as i32;
        if r % 2 == 0 {
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
    fn a_ring_should_extend_in_both_x_and_z() {
        let pattern = RingPattern::new(WHITE, BLACK);
        assert_eq!(pattern.color_at(Tuple::point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.color_at(Tuple::point(1.0, 0.0, 0.0)), BLACK);
        assert_eq!(pattern.color_at(Tuple::point(0.0, 0.0, 1.0)), BLACK);
        assert_eq!(pattern.color_at(Tuple::point(0.708, 0.0, 0.708)), BLACK);
    }
}
