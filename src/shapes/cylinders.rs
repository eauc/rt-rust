use crate::bounds::Bounds;
use crate::floats::{EPSILON, Float, equals};
use crate::intersections::Intersection;
use crate::objects::Object;
use crate::rays::Ray;
use crate::tuples::Tuple;

pub struct Cylinder {
    pub closed: bool,
    pub minimum: Float,
    pub maximum: Float,
}

impl Cylinder {
    pub fn new() -> Cylinder {
        Cylinder {
            closed: false,
            minimum: -Float::INFINITY,
            maximum: Float::INFINITY,
        }
    }
    pub fn truncate(&mut self, min: Float, max: Float, closed: bool) {
        self.minimum = min;
        self.maximum = max;
        self.closed = closed;
    }
    pub fn prepare_bounds(&mut self, bounds: &mut Bounds) {
        bounds.min = Tuple::point(-1.0, self.minimum, -1.0);
        bounds.max = Tuple::point(1.0, self.maximum, 1.0);
    }
    pub fn local_intersect<'a>(&'a self, ray: &Ray, object: &'a Object) -> Vec<Intersection<'a>> {
        let mut xs = vec![];
        self.intersect_sides(ray, &mut xs);
        self.intersect_caps(ray, &mut xs);
        xs.iter().map(|t| Intersection::new(*t, object)).collect()
    }
    pub fn local_normal_at(&self, local_point: Tuple) -> Tuple {
        let dist = local_point.x().powi(2) + local_point.z().powi(2);
        if dist < 1.0 && local_point.y() >= self.maximum - EPSILON {
            return Tuple::vector(0.0, 1.0, 0.0);
        }
        if dist < 1.0 && local_point.y() <= self.minimum + EPSILON {
            return Tuple::vector(0.0, -1.0, 0.0);
        }
        Tuple::vector(local_point.x(), 0.0, local_point.z())
    }
    fn intersect_caps<'a>(&'a self, ray: &Ray, xs: &mut Vec<Float>) {
        if !self.closed || equals(ray.direction.y(), 0.0) {
            return;
        }
        let t = (self.minimum - ray.origin.y()) / ray.direction.y();
        if check_cap(ray, t) {
            xs.push(t);
        }
        let t = (self.maximum - ray.origin.y()) / ray.direction.y();
        if check_cap(ray, t) {
            xs.push(t);
        }
    }
    fn intersect_sides<'a>(&'a self, ray: &Ray, xs: &mut Vec<Float>) {
        let a = ray.direction.x().powi(2) + ray.direction.z().powi(2);
        if equals(a, 0.0) {
            return;
        }
        let b = 2.0 * ray.origin.x() * ray.direction.x() + 2.0 * ray.origin.z() * ray.direction.z();
        let c = ray.origin.x().powi(2) + ray.origin.z().powi(2) - 1.0;
        let disc = b.powi(2) - 4.0 * a * c;
        if disc < 0.0 {
            return;
        }
        let t0 = (-b - disc.sqrt()) / (2.0 * a);
        let t1 = (-b + disc.sqrt()) / (2.0 * a);
        let (t0, t1) = (t0.min(t1), t0.max(t1));
        let y0 = ray.origin.y() + t0 * ray.direction.y();
        if self.minimum < y0 && y0 < self.maximum {
            xs.push(t0);
        }
        let y1 = ray.origin.y() + t1 * ray.direction.y();
        if self.minimum < y1 && y1 < self.maximum {
            xs.push(t1);
        }
    }
}

fn check_cap(ray: &Ray, t: Float) -> bool {
    let x = ray.origin.x() + t * ray.direction.x();
    let z = ray.origin.z() + t * ray.direction.z();
    return x.powi(2) + z.powi(2) <= 1.0 + EPSILON;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::floats::Float;

    #[test]
    fn a_ray_misses_a_cylinder() {
        let cyl = Object::new_cylinder();
        let origins = vec![
            Tuple::point(1.0, 0.0, 0.0),
            Tuple::point(0.0, 0.0, 0.0),
            Tuple::point(0.0, 0.0, -5.0),
        ];
        let directions = vec![
            Tuple::vector(0.0, 1.0, 0.0),
            Tuple::vector(0.0, 1.0, 0.0),
            Tuple::vector(1.0, 1.0, 1.0),
        ];
        for (origin, direction) in origins.iter().zip(directions.iter()) {
            let r = Ray::new(*origin, *direction);
            let xs = cyl.as_cylinder().local_intersect(&r, &cyl);
            assert_eq!(xs.len(), 0);
        }
    }

    #[test]
    fn a_ray_strikes_a_cylinder() {
        let cyl = Object::new_cylinder();
        let origins = vec![
            Tuple::point(1.0, 0.0, -5.0),
            Tuple::point(0.0, 0.0, -5.0),
            Tuple::point(0.5, 0.0, -5.0),
        ];
        let directions = vec![
            Tuple::vector(0.0, 0.0, 1.0),
            Tuple::vector(0.0, 0.0, 1.0),
            Tuple::vector(0.1, 1.0, 1.0),
        ];
        let results = vec![vec![5.0, 5.0], vec![4.0, 6.0], vec![4.801988, 4.999992]];
        for i in 0..origins.len() {
            let r = Ray::new(origins[i], directions[i]);
            let xs = cyl.as_cylinder().local_intersect(&r, &cyl);
            assert_eq!(xs.iter().map(|x| x.t).collect::<Vec<_>>(), results[i]);
        }
    }

    #[test]
    fn normal_vector_on_a_cylinder() {
        let cyl = Cylinder::new();
        let points = vec![
            Tuple::point(1.0, 0.0, 0.0),
            Tuple::point(0.0, 5.0, -1.0),
            Tuple::point(0.0, -2.0, 1.0),
            Tuple::point(-1.0, 1.0, 0.0),
        ];
        let normals = vec![
            Tuple::vector(1.0, 0.0, 0.0),
            Tuple::vector(0.0, 0.0, -1.0),
            Tuple::vector(0.0, 0.0, 1.0),
            Tuple::vector(-1.0, 0.0, 0.0),
        ];
        for i in 0..points.len() {
            let n = cyl.local_normal_at(points[i]);
            assert_eq!(n, normals[i]);
        }
    }

    #[test]
    fn the_default_minimum_and_maximum_for_a_cylinder() {
        let cyl = Cylinder::new();
        assert_eq!(cyl.minimum, -Float::INFINITY);
        assert_eq!(cyl.maximum, Float::INFINITY);
    }

    #[test]
    fn intersecting_a_constrained_cylinder() {
        let mut cyl = Object::new_cylinder();
        cyl.as_mut_cylinder().minimum = 1.0;
        cyl.as_mut_cylinder().maximum = 2.0;
        let points = vec![
            Tuple::point(0.0, 1.5, 0.0),
            Tuple::point(0.0, 3.0, -5.0),
            Tuple::point(0.0, 0.0, -5.0),
            Tuple::point(0.0, 2.0, -5.0),
            Tuple::point(0.0, 1.0, -5.0),
            Tuple::point(0.0, 1.5, -2.0),
        ];
        let directions = vec![
            Tuple::vector(0.1, 1.0, 0.0),
            Tuple::vector(0.0, 0.0, 1.0),
            Tuple::vector(0.0, 0.0, 1.0),
            Tuple::vector(0.0, 0.0, 1.0),
            Tuple::vector(0.0, 0.0, 1.0),
            Tuple::vector(0.0, 0.0, 1.0),
        ];
        let counts = vec![0, 0, 0, 0, 0, 2];
        for i in 0..points.len() {
            let r = Ray::new(points[i], directions[i].normalize());
            let xs = cyl.as_cylinder().local_intersect(&r, &cyl);
            assert_eq!(xs.len(), counts[i]);
        }
    }

    #[test]
    fn the_default_closed_value_for_a_cylinder() {
        let cyl = Cylinder::new();
        assert_eq!(cyl.closed, false);
    }

    #[test]
    fn intersecting_the_caps_of_a_closed_cylinder() {
        let mut cyl = Object::new_cylinder();
        cyl.as_mut_cylinder().minimum = 1.0;
        cyl.as_mut_cylinder().maximum = 2.0;
        cyl.as_mut_cylinder().closed = true;
        let points = vec![
            Tuple::point(0.0, 3.0, 0.0),
            Tuple::point(0.0, 3.0, -2.0),
            Tuple::point(0.0, 4.0, -2.0),
            Tuple::point(0.0, 0.0, -2.0),
            Tuple::point(0.0, -1.0, -2.0),
        ];
        let directions = vec![
            Tuple::vector(0.0, -1.0, 0.0),
            Tuple::vector(0.0, -1.0, 2.0),
            Tuple::vector(0.0, -1.0, 1.0),
            Tuple::vector(0.0, 1.0, 2.0),
            Tuple::vector(0.0, 1.0, 1.0),
        ];
        let counts = vec![2, 2, 2, 2, 2];
        for i in 0..points.len() {
            let r = Ray::new(points[i], directions[i].normalize());
            let xs = cyl.as_cylinder().local_intersect(&r, &cyl);
            assert_eq!(xs.len(), counts[i]);
        }
    }

    #[test]
    fn the_normal_vector_on_a_cylinders_end_caps() {
        let mut cyl = Cylinder::new();
        cyl.minimum = 1.0;
        cyl.maximum = 2.0;
        cyl.closed = true;
        let points = vec![
            Tuple::point(0.0, 1.0, 0.0),
            Tuple::point(0.5, 1.0, 0.0),
            Tuple::point(0.0, 1.0, 0.5),
            Tuple::point(0.0, 2.0, 0.0),
            Tuple::point(0.5, 2.0, 0.0),
            Tuple::point(0.0, 2.0, 0.5),
        ];
        let normals = vec![
            Tuple::vector(0.0, -1.0, 0.0),
            Tuple::vector(0.0, -1.0, 0.0),
            Tuple::vector(0.0, -1.0, 0.0),
            Tuple::vector(0.0, 1.0, 0.0),
            Tuple::vector(0.0, 1.0, 0.0),
            Tuple::vector(0.0, 1.0, 0.0),
        ];
        for i in 0..points.len() {
            let n = cyl.local_normal_at(points[i]);
            assert_eq!(n, normals[i]);
        }
    }
}
