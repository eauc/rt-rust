use crate::colors::{Color};
use crate::floats::Float;
use crate::lights::point_lights;
use crate::rays::Ray;
use crate::tuples::Tuple;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CubeLight {
    size: Float,
    samples: usize,
}

impl CubeLight {
    pub fn new(size: Float, samples: usize) -> CubeLight {
        CubeLight {
            size,
            samples,
        }
    }

    pub fn shadowed_intensity<T>(
        &self,
        light_position: Tuple,
        light_intensity: Color,
        point: Tuple,
        hit_fn: T,
    ) -> Color
    where
        T: Fn(&Ray) -> Option<Float>,
    {
        let mut n_shadowed = 0;
        for _ in 0..self.samples {
            let light_position =
                light_position + Tuple::random_vector(self.size);
            n_shadowed += if point_lights::is_shadowed(light_position, point, &hit_fn) {
                0
            } else {
                1
            };
        }
        light_intensity * (n_shadowed as Float / (self.samples as Float))
    }
}
