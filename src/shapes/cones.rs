use crate::coordinates::{Coordinate, EPSILON, equals};
use crate::rays::Ray;
use crate::tuples::Tuple;

pub struct Cone {
    pub minimum: Coordinate,
    pub maximum: Coordinate,
    pub closed: bool,
}

impl Cone {
    pub fn new() -> Cone {
        Cone {
            minimum: -Coordinate::INFINITY,
            maximum: Coordinate::INFINITY,
            closed: false,
        }
    }

    pub fn local_intersect<'a>(&'a self, ray: &Ray) -> Vec<Coordinate> {
        let mut xs = vec![];
        self.intersect_sides(ray, &mut xs);
        self.intersect_caps(ray, &mut xs);
        xs
    }

    pub fn local_normal_at(&self, local_point: Tuple) -> Tuple {
        let y = (local_point.x().powi(2) + local_point.z().powi(2)).sqrt();
        let y = if local_point.y() > 0.0 { -y } else { y };
        let dist = local_point.x().powi(2) + local_point.z().powi(2);
        let rad2 = local_point.y().powi(2);
        if dist < rad2 && local_point.y() >= self.maximum - EPSILON {
            return Tuple::vector(0.0, 1.0, 0.0);
        }
        if dist < rad2 && local_point.y() <= self.minimum + EPSILON {
            return Tuple::vector(0.0, -1.0, 0.0);
        }
        Tuple::vector(local_point.x(), y, local_point.z())
    }

    fn intersect_caps<'a>(&'a self, ray: &Ray, xs: &mut Vec<Coordinate>) {
        if !self.closed || equals(ray.direction.y(), 0.0) {
            return;
        }
        let t = (self.minimum - ray.origin.y()) / ray.direction.y();
        if check_cap(ray, t, self.minimum.abs()) {
            xs.push(t);
        }
        let t = (self.maximum - ray.origin.y()) / ray.direction.y();
        if check_cap(ray, t, self.maximum.abs()) {
            xs.push(t);
        }
    }

    fn intersect_sides<'a>(&'a self, ray: &Ray, xs: &mut Vec<Coordinate>) {
        let a = ray.direction.x().powi(2) - ray.direction.y().powi(2) + ray.direction.z().powi(2);
        let b = 2.0 * ray.origin.x() * ray.direction.x() - 2.0 * ray.origin.y() * ray.direction.y()
            + 2.0 * ray.origin.z() * ray.direction.z();
        let c = ray.origin.x().powi(2) - ray.origin.y().powi(2) + ray.origin.z().powi(2);
        if equals(a, 0.0) && !equals(b, 0.0) {
            let t = -c / (2.0 * b);
            xs.push(t);
            return;
        }
        let disc = b.powi(2) - 4.0 * a * c;
        if disc < -EPSILON {
            return;
        }
        let disc = disc.max(0.0);
        let t0 = (-b - disc.sqrt()) / (2.0 * a);
        let t1 = (-b + disc.sqrt()) / (2.0 * a);
        let (t0, t1) = (t0.min(t1), t0.max(t1));
        let y0 = ray.origin.y() + t0 * ray.direction.y();
        if self.minimum - EPSILON < y0 && y0 < self.maximum + EPSILON {
            xs.push(t0);
        }
        let y1 = ray.origin.y() + t1 * ray.direction.y();
        if self.minimum - EPSILON < y1 && y1 < self.maximum + EPSILON {
            xs.push(t1);
        }
    }
}

fn check_cap(ray: &Ray, t: Coordinate, radius: Coordinate) -> bool {
    let x = ray.origin.x() + t * ray.direction.x();
    let z = ray.origin.z() + t * ray.direction.z();
    return x.powi(2) + z.powi(2) <= radius + EPSILON;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intersecting_a_cone_with_a_ray() {
        let shape = Cone::new();
        let origins = vec![
            Tuple::point(0.0, 0.0, -5.0),
            Tuple::point(0.0, 0.0, -5.0),
            Tuple::point(1.0, 1.0, -5.0),
        ];
        let directions = vec![
            Tuple::vector(0.0, 0.0, 1.0),
            Tuple::vector(1.0, 1.0, 1.0),
            Tuple::vector(-0.5, -1.0, 1.0),
        ];
        let results = vec![
            vec![5.0, 5.0],
            vec![8.6602545, 8.6602545],
            vec![4.5500546, 49.449955],
        ];
        for i in 0..origins.len() {
            let r = Ray::new(origins[i], directions[i].normalize());
            let xs = shape.local_intersect(&r);
            assert_eq!(xs, results[i]);
        }
    }

    #[test]
    fn intersecting_a_cone_with_a_ray_parallel_to_one_of_its_halves() {
        let shape = Cone::new();
        let r = Ray::new(
            Tuple::point(0.0, 0.0, -1.0),
            Tuple::vector(0.0, 1.0, 1.0).normalize(),
        );
        let xs = shape.local_intersect(&r);
        assert_eq!(xs, vec![0.35355338]);
    }

    #[test]
    fn intersecting_a_cone_end_caps() {
        let mut shape = Cone::new();
        shape.minimum = -0.5;
        shape.maximum = 0.5;
        shape.closed = true;
        let origins = vec![
            Tuple::point(0.0, 0.0, -5.0),
            Tuple::point(0.0, 0.0, -0.25),
            Tuple::point(0.0, 0.0, -0.25),
        ];
        let directions = vec![
            Tuple::vector(0.0, 1.0, 0.0),
            Tuple::vector(0.0, 1.0, 1.0),
            Tuple::vector(0.0, 1.0, 0.0),
        ];
        let counts = vec![0, 2, 4];
        for i in 0..origins.len() {
            let r = Ray::new(origins[i], directions[i].normalize());
            let xs = shape.local_intersect(&r);
            assert_eq!(xs.len(), counts[i]);
        }
    }

    #[test]
    fn computing_the_normal_vector_on_a_cone() {
        let shape = Cone::new();
        let points = vec![
            Tuple::point(0.0, 0.0, 0.0),
            Tuple::point(1.0, 1.0, 1.0),
            Tuple::point(-1.0, -1.0, 0.0),
        ];
        let normals = vec![
            Tuple::vector(0.0, 0.0, 0.0),
            Tuple::vector(1.0, -((2.0_f32).sqrt()), 1.0),
            Tuple::vector(-1.0, 1.0, 0.0),
        ];
        for i in 0..points.len() {
            let n = shape.local_normal_at(points[i]);
            assert_eq!(n, normals[i]);
        }
    }
}
