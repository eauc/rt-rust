use crate::colors::{Color, BLACK};
use crate::floats::Float;
use crate::rays::Ray;
use crate::tuples::Tuple;

pub fn is_shadowed<T>(light_position: Tuple, point: Tuple, hit_fn: &T) -> bool
where
    T: Fn(&Ray) -> Option<Float>,
{
    let v = light_position - point;
    let distance = v.magnitude();
    let direction = v.normalize();
    let r = Ray::new(point, direction);

    if let Some(hit) = hit_fn(&r)
        && hit < distance
    {
        true
    } else {
        false
    }
}

pub fn shadowed_intensity<T>(
    light_position: Tuple,
    light_intensity: Color,
    point: Tuple,
    hit_fn: T,
) -> Color
where
    T: Fn(&Ray) -> Option<Float>,
{
    if is_shadowed(light_position, point, &hit_fn) {
        BLACK
    } else {
        light_intensity
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn there_is_no_shadow_when_nothing_is_collinear_with_point_and_light() {
        let light_position = Tuple::point(-10.0, 10.0, -10.0);
        let p = Tuple::point(0.0, 10.0, 0.0);
        assert!(!is_shadowed(light_position, p, &|_| None));
    }

    #[test]
    fn there_is_a_shadow_when_an_object_is_between_the_point_and_the_light() {
        let light_position = Tuple::point(-10.0, 10.0, -10.0);
        let p = Tuple::point(10.0, -10.0, 10.0);
        assert!(is_shadowed(light_position, p, &|_| Some(1.0)));
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_light() {
        let light_position = Tuple::point(-10.0, 10.0, -10.0);
        let p = Tuple::point(-20.0, 20.0, -20.);
        assert!(!is_shadowed(light_position, p, &|_| Some(20.0)));
    }
}
