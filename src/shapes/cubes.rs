use crate::floats::{Float, EPSILON};
use crate::intersections::Intersection;
use crate::objects::Object;
use crate::rays::Ray;
use crate::tuples::Tuple;

#[derive(Debug, Clone, PartialEq)]
pub struct Cube;

impl Cube {
    pub fn new() -> Cube {
        Cube
    }

    pub fn local_intersect<'a>(
        &'a self,
        ray: &Ray,
        object: &'a Object,
        xs: &mut Vec<Intersection<'a>>,
    ) {
        let (xtmin, xtmax) = check_axis(ray.origin.x(), ray.direction.x());
        let (ytmin, ytmax) = check_axis(ray.origin.y(), ray.direction.y());
        let (ztmin, ztmax) = check_axis(ray.origin.z(), ray.direction.z());
        let tmin = xtmin.max(ytmin).max(ztmin);
        let tmax = xtmax.min(ytmax).min(ztmax);
        if tmin > tmax {
            return;
        }
        xs.push(Intersection::new(tmin, object));
        xs.push(Intersection::new(tmax, object));
    }

    pub fn local_normal_at(&self, local_point: Tuple) -> Tuple {
        let maxc = local_point
            .x()
            .abs()
            .max(local_point.y().abs())
            .max(local_point.z().abs());
        match maxc {
            val if val == local_point.x().abs() => Tuple::vector(local_point.x(), 0.0, 0.0),
            val if val == local_point.y().abs() => Tuple::vector(0.0, local_point.y(), 0.0),
            val if val == local_point.z().abs() => Tuple::vector(0.0, 0.0, local_point.z()),
            _ => panic!(),
        }
    }
}

impl Default for Cube {
    fn default() -> Cube {
        Cube::new()
    }
}

fn check_axis(origin: Float, direction: Float) -> (Float, Float) {
    let tmin_numerator = -1.0 - origin;
    let tmax_numerator = 1.0 - origin;

    let (tmin, tmax) = if direction.abs() >= EPSILON {
        (tmin_numerator / direction, tmax_numerator / direction)
    } else {
        (
            tmin_numerator * Float::INFINITY,
            tmax_numerator * Float::INFINITY,
        )
    };
    if tmin > tmax {
        (tmax, tmin)
    } else {
        (tmin, tmax)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rays::Ray;
    use crate::tuples::Tuple;

    #[test]
    fn a_ray_intersects_a_cube() {
        let c = Object::new_cube();
        let origins = vec![
            Tuple::point(5.0, 0.5, 0.0),
            Tuple::point(-5.0, 0.5, 0.0),
            Tuple::point(0.5, 5.0, 0.0),
            Tuple::point(0.5, -5.0, 0.0),
            Tuple::point(0.5, 0.0, 5.0),
            Tuple::point(0.5, 0.0, -5.0),
            Tuple::point(0.0, 0.5, 0.0),
        ];
        let directions = vec![
            Tuple::vector(-1.0, 0.0, 0.0),
            Tuple::vector(1.0, 0.0, 0.0),
            Tuple::vector(0.0, -1.0, 0.0),
            Tuple::vector(0.0, 1.0, 0.0),
            Tuple::vector(0.0, 0.0, -1.0),
            Tuple::vector(0.0, 0.0, 1.0),
            Tuple::vector(0.0, 0.0, 1.0),
        ];
        let results = vec![
            vec![4.0, 6.0],
            vec![4.0, 6.0],
            vec![4.0, 6.0],
            vec![4.0, 6.0],
            vec![4.0, 6.0],
            vec![4.0, 6.0],
            vec![-1.0, 1.0],
        ];
        for i in 0..origins.len() {
            let r = Ray::new(origins[i], directions[i]);
            let mut xs = Vec::new();
            c.as_cube().local_intersect(&r, &c, &mut xs);
            // println!("{:?} {:?}", origins[i], directions[i]);
            assert_eq!(xs.iter().map(|x| x.t).collect::<Vec<_>>(), results[i]);
        }
    }

    #[test]
    fn a_ray_misses_a_cube() {
        let c = Object::new_cube();
        let origins = vec![
            Tuple::point(-2.0, 0.0, 0.0),
            Tuple::point(0.0, -2.0, 0.0),
            Tuple::point(0.0, 0.0, -2.0),
            Tuple::point(2.0, 0.0, 2.0),
            Tuple::point(0.0, 2.0, 2.0),
            Tuple::point(2.0, 2.0, 0.0),
        ];
        let directions = vec![
            Tuple::vector(0.2673, 0.5345, 0.8018),
            Tuple::vector(0.8018, 0.2673, 0.5345),
            Tuple::vector(0.5345, 0.8018, 0.2673),
            Tuple::vector(0.0, 0.0, -1.0),
            Tuple::vector(0.0, -1.0, 0.0),
            Tuple::vector(-1.0, 0.0, 0.0),
        ];
        for i in 0..origins.len() {
            let r = Ray::new(origins[i], directions[i]);
            let mut xs = Vec::new();
            c.as_cube().local_intersect(&r, &c, &mut xs);
            assert_eq!(xs.len(), 0);
        }
    }

    #[test]
    fn the_normal_on_the_surface_of_a_cube() {
        let c = Cube::new();
        let points = vec![
            Tuple::point(1.0, 0.5, -0.8),
            Tuple::point(-1.0, -0.2, 0.9),
            Tuple::point(-0.4, 1.0, -0.1),
            Tuple::point(0.3, -1.0, -0.7),
            Tuple::point(-0.6, 0.3, 1.0),
            Tuple::point(0.4, 0.4, -1.0),
            Tuple::point(1.0, 1.0, 1.0),
            Tuple::point(-1.0, -1.0, -1.0),
        ];
        let normals = vec![
            Tuple::vector(1.0, 0.0, 0.0),
            Tuple::vector(-1.0, 0.0, 0.0),
            Tuple::vector(0.0, 1.0, 0.0),
            Tuple::vector(0.0, -1.0, 0.0),
            Tuple::vector(0.0, 0.0, 1.0),
            Tuple::vector(0.0, 0.0, -1.0),
            Tuple::vector(1.0, 0.0, 0.0),
            Tuple::vector(-1.0, 0.0, 0.0),
        ];
        for i in 0..points.len() {
            let n = c.local_normal_at(points[i]);
            assert_eq!(n, normals[i]);
        }
    }
}
