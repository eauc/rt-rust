use crate::colors::Color;
use crate::floats::Float;
use crate::rays::Ray;
use crate::tuples::Tuple;

mod cube_lights;
mod point_lights;
mod sphere_lights;
mod spot_lights;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Lights {
    Cube(cube_lights::CubeLight),
    Point,
    Sphere(sphere_lights::SphereLight),
    Spot(spot_lights::SpotLight),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Light {
    pub position: Tuple,
    pub intensity: Color,
    light: Lights,
}

impl Light {
    fn new(light: Lights, position: Tuple, intensity: Color) -> Light {
        Light {
            position: position,
            intensity: intensity,
            light: light,
        }
    }
    pub fn new_cube(position: Tuple, intensity: Color, size: Float, samples: usize) -> Light {
        Light::new(
            Lights::Cube(cube_lights::CubeLight::new(size, samples)),
            position,
            intensity,
        )
    }
    pub fn new_point(position: Tuple, intensity: Color) -> Light {
        Light::new(Lights::Point, position, intensity)
    }
    pub fn new_sphere(position: Tuple, intensity: Color, size: Float, samples: usize) -> Light {
        Light::new(
            Lights::Sphere(sphere_lights::SphereLight::new(size, samples)),
            position,
            intensity,
        )
    }
    pub fn new_spot(
        position: Tuple,
        intensity: Color,
        direction: Tuple,
        width: Float,
        fade: Float,
    ) -> Light {
        Light::new(
            Lights::Spot(spot_lights::SpotLight::new(direction, width, fade)),
            position,
            intensity,
        )
    }

    pub fn shadowed<T>(&self, point: Tuple, hit_fn: T) -> Light
    where
        T: Fn(&Ray) -> Option<Float>,
    {
        Light {
            intensity: match self.light {
                Lights::Cube(cube) => {
                    cube.shadowed_intensity(self.position, self.intensity, point, hit_fn)
                }
                Lights::Point => {
                    point_lights::shadowed_intensity(self.position, self.intensity, point, hit_fn)
                }
                Lights::Sphere(sphere) => {
                    sphere.shadowed_intensity(self.position, self.intensity, point, hit_fn)
                }
                Lights::Spot(spot) => {
                    spot.shadowed_intensity(self.position, self.intensity, point, hit_fn)
                }
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
