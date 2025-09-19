use crate::colors::Color;
use crate::tuples::Tuple;

#[derive(Debug, Clone, Copy)]
pub struct GradientPattern {
    a: Color,
    b: Color,
}

impl GradientPattern {
    pub fn new(a: Color, b: Color) -> GradientPattern {
        GradientPattern { a, b }
    }

    pub fn color_at(&self, point: Tuple) -> Color {
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
