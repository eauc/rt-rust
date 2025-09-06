use crate::colors::Color;
use crate::tuples::Tuple;

pub struct PointLight {
    pub position: Tuple,
    pub intensity: Color,
}

impl PointLight {
    pub fn new(position: Tuple, intensity: Color) -> PointLight {
        PointLight {
            position: position,
            intensity: intensity,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_point_light_has_a_position_and_intensity() {
        let position = Tuple::point(0.0, 0.0, 0.0);
        let intensity = Color(1.0, 1.0, 1.0);
        let light = PointLight::new(position, intensity);
        assert_eq!(light.position, position);
        assert_eq!(light.intensity, intensity);
    }
}
