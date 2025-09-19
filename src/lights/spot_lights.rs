use crate::colors::{Color, BLACK};
use crate::floats::Float;
use crate::lights::point_lights;
use crate::rays::Ray;
use crate::tuples::Tuple;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpotLight {
    direction: Tuple,
    width: Float,
    narrow_width: Float,
    fade: Float,
}

impl SpotLight {
    pub fn new(direction: Tuple, width: Float, fade: Float) -> SpotLight {
        SpotLight {
            direction,
            width,
            narrow_width: width * (1.0 - fade),
            fade,
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
        let light_to_point = point - light_position;
        let angle = self.direction.angle(light_to_point);
        if point_lights::is_shadowed(light_position, point, &hit_fn) || angle > self.width {
            BLACK
        } else if angle > self.narrow_width {
            light_intensity * (1.0 - (angle - self.narrow_width) / (self.width - self.narrow_width))
        } else {
            light_intensity
        }
    }
}
