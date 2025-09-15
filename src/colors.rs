use crate::floats::{Float, equals};
use std::{cmp, ops};

#[derive(Debug, Copy, Clone)]
pub struct Color(Float, Float, Float);

pub const BLACK: Color = Color(0.0, 0.0, 0.0);
pub const WHITE: Color = Color(1.0, 1.0, 1.0);

impl Color {
    pub fn new(red: Float, green: Float, blue: Float) -> Color {
        Color(red, green, blue)
    }
    pub fn red(&self) -> Float {
        self.0
    }
    pub fn green(&self) -> Float {
        self.1
    }
    pub fn blue(&self) -> Float {
        self.2
    }
}

impl cmp::PartialEq for Color {
    fn eq(&self, other: &Color) -> bool {
        equals(self.red(), other.red())
            && equals(self.green(), other.green())
            && equals(self.blue(), other.blue())
    }
}

impl ops::Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color(
            self.red() + other.red(),
            self.green() + other.green(),
            self.blue() + other.blue(),
        )
    }
}

impl ops::Sub for Color {
    type Output = Color;

    fn sub(self, other: Color) -> Color {
        Color(
            self.red() - other.red(),
            self.green() - other.green(),
            self.blue() - other.blue(),
        )
    }
}

impl ops::Mul<Float> for Color {
    type Output = Color;

    fn mul(self, scalar: Float) -> Color {
        Color(
            self.red() * scalar,
            self.green() * scalar,
            self.blue() * scalar,
        )
    }
}

impl ops::Mul<Color> for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color(
            self.red() * other.red(),
            self.green() * other.green(),
            self.blue() * other.blue(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colors_are_tuples() {
        let c = Color(-0.5, 0.4, 1.7);
        assert_eq!(c.red(), -0.5);
        assert_eq!(c.green(), 0.4);
        assert_eq!(c.blue(), 1.7);
    }

    #[test]
    fn adding_colors() {
        let c1 = Color(0.9, 0.6, 0.75);
        let c2 = Color(0.7, 0.1, 0.25);
        assert_eq!(c1 + c2, Color(1.6, 0.7, 1.0));
    }

    #[test]
    fn subtracting_colors() {
        let c1 = Color(0.9, 0.6, 0.75);
        let c2 = Color(0.7, 0.1, 0.25);
        assert_eq!(c1 - c2, Color(0.2, 0.5, 0.5));
    }

    #[test]
    fn multiplying_a_color_by_a_scalar() {
        let c = Color(0.2, 0.3, 0.4);
        assert_eq!(c * 2.0, Color(0.4, 0.6, 0.8));
    }

    #[test]
    fn multiplying_colors() {
        let c1 = Color(1.0, 0.2, 0.4);
        let c2 = Color(0.9, 1.0, 0.1);
        assert_eq!(c1 * c2, Color(0.9, 0.2, 0.04));
    }
}
