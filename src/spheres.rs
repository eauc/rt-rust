use crate::intersections::Intersection;
use crate::matrices::Matrix;
use crate::rays::Ray;
use crate::tuples::Tuple;
use std::cmp;

#[derive(Debug, cmp::PartialEq)]
pub struct Sphere {
    transform: Matrix<4>,
}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere {
            transform: Matrix::identity(),
        }
    }

    pub fn intersect<'a>(&'a self, ray: &Ray) -> Vec<Intersection<'a>> {
        let ray = ray.transform(self.transform.inverse());
        let sphere_to_ray = ray.origin - Tuple::point(0.0, 0.0, 0.0);
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return vec![];
        }
        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
        vec![Intersection::new(t1, &self), Intersection::new(t2, &self)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transformations;

    #[test]
    fn a_ray_intersects_a_sphere_at_two_points() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(&r);
        assert_eq!(
            xs,
            vec![Intersection::new(4.0, &s), Intersection::new(6.0, &s)]
        );
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_a_tangent() {
        let r = Ray::new(Tuple::point(0.0, 1.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(&r);
        assert_eq!(
            xs,
            vec![Intersection::new(5.0, &s), Intersection::new(5.0, &s)]
        );
    }

    #[test]
    fn a_ray_misses_a_sphere() {
        let r = Ray::new(Tuple::point(0.0, 2.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(&r);
        assert_eq!(xs, vec![]);
    }

    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(&r);
        assert_eq!(
            xs,
            vec![Intersection::new(-1.0, &s), Intersection::new(1.0, &s)]
        );
    }

    #[test]
    fn a_sphere_is_behind_a_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(&r);
        assert_eq!(
            xs,
            vec![Intersection::new(-6.0, &s), Intersection::new(-4.0, &s)]
        );
    }

    #[test]
    fn a_spheres_default_transformation() {
        let s = Sphere::new();
        assert_eq!(s.transform, Matrix::identity());
    }

    #[test]
    fn changing_a_spheres_transformation() {
        let mut s = Sphere::new();
        let t = transformations::translation(2.0, 3.0, 4.0);
        s.transform = t;
        assert_eq!(s.transform, t);
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let mut s = Sphere::new();
        let t = transformations::scaling(2.0, 2.0, 2.0);
        s.transform = t;
        let xs = s.intersect(&r);
        assert_eq!(
            xs,
            vec![Intersection::new(3.0, &s), Intersection::new(7.0, &s)]
        );
    }

    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let mut s = Sphere::new();
        let t = transformations::translation(5.0, 0.0, 0.0);
        s.transform = t;
        let xs = s.intersect(&r);
        assert_eq!(xs, vec![]);
    }
}
