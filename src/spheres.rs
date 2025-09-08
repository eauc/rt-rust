use crate::intersections::Intersection;
use crate::materials::Material;
use crate::matrices::Matrix;
use crate::rays::Ray;
use crate::tuples::Tuple;

#[derive(Debug, PartialEq)]
pub struct Sphere {
    pub material: Material,
    transform: Matrix<4>,
    transform_inverse: Matrix<4>,
    transform_inverse_transpose: Matrix<4>,
}

impl Sphere {
    pub fn new(transform: Matrix<4>) -> Sphere {
        let transform_inverse = transform.inverse();
        Sphere {
            material: Material::default(),
            transform,
            transform_inverse,
            transform_inverse_transpose: transform_inverse.transpose(),
        }
    }

    pub fn default() -> Sphere {
        Sphere::new(Matrix::identity())
    }

    pub fn intersect<'a>(&'a self, ray: &Ray) -> Vec<Intersection<'a>> {
        let ray = ray.transform(self.transform_inverse);
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

    pub fn normal_at(&self, world_point: Tuple) -> Tuple {
        let object_point = self.transform_inverse * world_point;
        let object_normal = object_point - Tuple::point(0.0, 0.0, 0.0);
        let mut world_normal = self.transform_inverse_transpose * object_normal;
        world_normal.to_vector();
        world_normal.normalize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transformations::{rotation_z, scaling, translation};
    use std::f32::consts::PI;

    #[test]
    fn a_ray_intersects_a_sphere_at_two_points() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let xs = s.intersect(&r);
        assert_eq!(
            xs,
            vec![Intersection::new(4.0, &s), Intersection::new(6.0, &s)]
        );
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_a_tangent() {
        let r = Ray::new(Tuple::point(0.0, 1.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let xs = s.intersect(&r);
        assert_eq!(
            xs,
            vec![Intersection::new(5.0, &s), Intersection::new(5.0, &s)]
        );
    }

    #[test]
    fn a_ray_misses_a_sphere() {
        let r = Ray::new(Tuple::point(0.0, 2.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let xs = s.intersect(&r);
        assert_eq!(xs, vec![]);
    }

    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let xs = s.intersect(&r);
        assert_eq!(
            xs,
            vec![Intersection::new(-1.0, &s), Intersection::new(1.0, &s)]
        );
    }

    #[test]
    fn a_sphere_is_behind_a_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let xs = s.intersect(&r);
        assert_eq!(
            xs,
            vec![Intersection::new(-6.0, &s), Intersection::new(-4.0, &s)]
        );
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new(scaling(2.0, 2.0, 2.0));
        let xs = s.intersect(&r);
        assert_eq!(
            xs,
            vec![Intersection::new(3.0, &s), Intersection::new(7.0, &s)]
        );
    }

    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new(translation(5.0, 0.0, 0.0));
        let xs = s.intersect(&r);
        assert_eq!(xs, vec![]);
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_x_axis() {
        let s = Sphere::default();
        let n = s.normal_at(Tuple::point(1.0, 0.0, 0.0));
        assert_eq!(n, Tuple::vector(1.0, 0.0, 0.0));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_y_axis() {
        let s = Sphere::default();
        let n = s.normal_at(Tuple::point(0.0, 1.0, 0.0));
        assert_eq!(n, Tuple::vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_z_axis() {
        let s = Sphere::default();
        let n = s.normal_at(Tuple::point(0.0, 0.0, 1.0));
        assert_eq!(n, Tuple::vector(0.0, 0.0, 1.0));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_nonaxial_point() {
        let s = Sphere::default();
        let n = s.normal_at(Tuple::point(
            (3.0_f32).sqrt() / 3.0,
            (3.0_f32).sqrt() / 3.0,
            (3.0_f32).sqrt() / 3.0,
        ));
        assert_eq!(
            n,
            Tuple::vector(
                (3.0_f32).sqrt() / 3.0,
                (3.0_f32).sqrt() / 3.0,
                (3.0_f32).sqrt() / 3.0
            )
        );
    }

    #[test]
    fn the_normal_is_a_normalized_vector() {
        let s = Sphere::default();
        let n = s.normal_at(Tuple::point(
            (3.0_f32).sqrt() / 3.0,
            (3.0_f32).sqrt() / 3.0,
            (3.0_f32).sqrt() / 3.0,
        ));
        assert_eq!(n, n.normalize());
    }

    #[test]
    fn computing_the_normal_on_a_translated_sphere() {
        let s = Sphere::new(translation(0.0, 1.0, 0.0));
        let n = s.normal_at(Tuple::point(0.0, 1.70711, -0.70711));
        assert_eq!(n, Tuple::vector(0.0, 0.70711, -0.70711));
    }

    #[test]
    fn computing_the_normal_on_a_transformed_sphere() {
        let s = Sphere::new(scaling(1.0, 0.5, 1.0) * rotation_z(PI / 5.0));
        let n = s.normal_at(Tuple::point(
            0.0,
            (2.0_f32).sqrt() / 2.0,
            -(2.0_f32).sqrt() / 2.0,
        ));
        assert_eq!(n, Tuple::vector(0.0, 0.97014, -0.24254));
    }

    #[test]
    fn a_sphere_has_a_default_material() {
        let s = Sphere::default();
        assert_eq!(s.material, Material::default());
    }

    #[test]
    fn a_sphere_may_be_assigned_a_material() {
        let mut s = Sphere::default();
        let mut m = Material::default();
        m.ambient = 1.0;
        s.material = m;
        assert_eq!(s.material, m);
    }
}
