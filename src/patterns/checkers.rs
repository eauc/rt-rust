use crate::colors::Color;
use crate::matrices::Matrix;
use crate::patterns::Pattern;
use crate::tuples::Tuple;

pub struct CheckerPattern {
    a: Color,
    b: Color,
    transform_inverse: Matrix<4>,
}

impl CheckerPattern {
    pub fn new(a: Color, b: Color) -> CheckerPattern {
        CheckerPattern {
            a,
            b,
            transform_inverse: Matrix::identity(),
        }
    }

    pub fn set_transform(&mut self, transform: Matrix<4>) {
        self.transform_inverse = transform.inverse();
    }
}

impl Pattern for CheckerPattern {
    fn transform_inverse(&self) -> Matrix<4> {
        self.transform_inverse
    }
    fn color_at(&self, point: Tuple) -> Color {
        let c = (point.x().floor() + point.y().floor() + point.z().floor()) as i32;
        if c % 2 == 0 {
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
    fn checkers_should_repeat_in_x() {
        let pattern = CheckerPattern::new(WHITE, BLACK);
        assert_eq!(pattern.color_at(Tuple::point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.color_at(Tuple::point(0.99, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.color_at(Tuple::point(1.01, 0.0, 0.0)), BLACK);
    }

    #[test]
    fn checkers_should_repeat_in_y() {
        let pattern = CheckerPattern::new(WHITE, BLACK);
        assert_eq!(pattern.color_at(Tuple::point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.color_at(Tuple::point(0.0, 0.99, 0.0)), WHITE);
        assert_eq!(pattern.color_at(Tuple::point(0.0, 1.01, 0.0)), BLACK);
    }

    #[test]
    fn checkers_should_repeat_in_z() {
        let pattern = CheckerPattern::new(WHITE, BLACK);
        assert_eq!(pattern.color_at(Tuple::point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.color_at(Tuple::point(0.0, 0.0, 0.99)), WHITE);
        assert_eq!(pattern.color_at(Tuple::point(0.0, 0.0, 1.01)), BLACK);
    }
}
