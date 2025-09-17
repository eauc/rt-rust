use crate::colors::Color;
use crate::floats::Float;
use crate::rays::Ray;
use crate::tuples::Tuple;

mod point_lights;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Lights {
    Point,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Light {
    pub position: Tuple,
    pub intensity: Color,
    light: Lights,
}

impl Light {
    pub fn new_point(position: Tuple, intensity: Color) -> Light {
        Light {
            position: position,
            intensity: intensity,
            light: Lights::Point,
        }
    }

    pub fn shadowed<T>(&self, point: Tuple, hit_fn: T) -> Light
    where
        T: Fn(&Ray) -> Option<Float>,
    {
        Light {
            intensity: match self.light {
                Lights::Point => point_lights::shadowed_intensity(self.position, self.intensity, point, hit_fn),
            },
            ..*self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_point_light_has_a_position_and_intensity() {
        let position = Tuple::point(0.0, 0.0, 0.0);
        let intensity = Color::new(1.0, 1.0, 1.0);
        let light = Light::new_point(position, intensity);
        assert_eq!(light.position, position);
        assert_eq!(light.intensity, intensity);
    }
}
