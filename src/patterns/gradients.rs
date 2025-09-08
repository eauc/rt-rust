use crate::colors::Color;
use crate::matrices::Matrix;
use crate::patterns::Pattern;
use crate::tuples::Tuple;

pub struct GradientPattern {
    a: Color,
    b: Color,
    transform_inverse: Matrix<4>,
}

impl GradientPattern {
    pub fn new(a: Color, b: Color) -> GradientPattern {
        GradientPattern {
            a,
            b,
            transform_inverse: Matrix::identity(),
        }
    }

    pub fn set_transform(&mut self, transform: Matrix<4>) {
        self.transform_inverse = transform.inverse();
    }
}

impl Pattern for GradientPattern {
    fn transform_inverse(&self) -> Matrix<4> {
        self.transform_inverse
    }
    fn color_at(&self, point: Tuple) -> Color {
        let distance = self.b - self.a;
        let fraction = point.x() - point.x().floor();
        self.a + distance * fraction
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::colors::{BLACK, WHITE};

    #[test]
    fn a_gradient_linearly_interpolates_between_colors() {
        let pattern = GradientPattern::new(WHITE, BLACK);
        assert_eq!(pattern.color_at(Tuple::point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(
            pattern.color_at(Tuple::point(0.25, 0.0, 0.0)),
            Color::new(0.75, 0.75, 0.75)
        );
        assert_eq!(
            pattern.color_at(Tuple::point(0.5, 0.0, 0.0)),
            Color::new(0.5, 0.5, 0.5)
        );
        assert_eq!(
            pattern.color_at(Tuple::point(0.75, 0.0, 0.0)),
            Color::new(0.25, 0.25, 0.25)
        );
    }
}
