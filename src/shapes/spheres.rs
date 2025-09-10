use crate::intersections::Intersection;
use crate::materials::Material;
use crate::matrices::Matrix;
use crate::rays::Ray;
use crate::shapes::Shape;
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

    pub fn glass(transform: Matrix<4>) -> Sphere {
        let mut s = Sphere::new(transform);
        s.material.ambient = 0.0;
        s.material.diffuse = 0.588235;
        s.material.specular = 0.9;
        s.material.transparency = 1.0;
        s.material.reflective = 0.08;
        s.material.refractive_index = 1.5;
        s
    }
}

impl Shape for Sphere {
    fn material(&self) -> &Material {
        &self.material
    }
    fn transform_inverse(&self) -> Matrix<4> {
        self.transform_inverse
    }
    fn transform_inverse_transpose(&self) -> Matrix<4> {
        self.transform_inverse_transpose
    }
    fn local_intersect<'a>(&'a self, ray: &Ray) -> Vec<Intersection<'a>> {
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
        vec![Intersection::new(t1, self), Intersection::new(t2, self)]
    }
    fn local_normal_at(&self, local_point: Tuple) -> Tuple {
        local_point - Tuple::point(0.0, 0.0, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shapes::{intersect, normal_at};
    use crate::transformations::{rotation_z, scaling, translation};
    use std::f32::consts::PI;

    #[test]
    fn a_ray_intersects_a_sphere_at_two_points() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let xs = intersect(&s, &r);
        assert_eq!(xs.iter().map(|x| x.t).collect::<Vec<f32>>(), vec![4.0, 6.0]);
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_a_tangent() {
        let r = Ray::new(Tuple::point(0.0, 1.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let xs = intersect(&s, &r);
        assert_eq!(xs.iter().map(|x| x.t).collect::<Vec<f32>>(), vec![5.0, 5.0]);
    }

    #[test]
    fn a_ray_misses_a_sphere() {
        let r = Ray::new(Tuple::point(0.0, 2.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let xs = intersect(&s, &r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let xs = intersect(&s, &r);
        assert_eq!(
            xs.iter().map(|x| x.t).collect::<Vec<f32>>(),
            vec![-1.0, 1.0]
        );
    }

    #[test]
    fn a_sphere_is_behind_a_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let xs = intersect(&s, &r);
        assert_eq!(
            xs.iter().map(|x| x.t).collect::<Vec<f32>>(),
            vec![-6.0, -4.0]
        );
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new(scaling(2.0, 2.0, 2.0));
        let xs = intersect(&s, &r);
        assert_eq!(xs.iter().map(|x| x.t).collect::<Vec<f32>>(), vec![3.0, 7.0]);
    }

    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new(translation(5.0, 0.0, 0.0));
        let xs = intersect(&s, &r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_x_axis() {
        let s = Sphere::default();
        let n = normal_at(&s, Tuple::point(1.0, 0.0, 0.0));
        assert_eq!(n, Tuple::vector(1.0, 0.0, 0.0));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_y_axis() {
        let s = Sphere::default();
        let n = normal_at(&s, Tuple::point(0.0, 1.0, 0.0));
        assert_eq!(n, Tuple::vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_z_axis() {
        let s = Sphere::default();
        let n = normal_at(&s, Tuple::point(0.0, 0.0, 1.0));
        assert_eq!(n, Tuple::vector(0.0, 0.0, 1.0));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_nonaxial_point() {
        let s = Sphere::default();
        let n = normal_at(
            &s,
            Tuple::point(
                (3.0_f32).sqrt() / 3.0,
                (3.0_f32).sqrt() / 3.0,
                (3.0_f32).sqrt() / 3.0,
            ),
        );
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
        let n = normal_at(
            &s,
            Tuple::point(
                (3.0_f32).sqrt() / 3.0,
                (3.0_f32).sqrt() / 3.0,
                (3.0_f32).sqrt() / 3.0,
            ),
        );
        assert_eq!(n, n.normalize());
    }

    #[test]
    fn computing_the_normal_on_a_translated_sphere() {
        let s = Sphere::new(translation(0.0, 1.0, 0.0));
        let n = normal_at(&s, Tuple::point(0.0, 1.70711, -0.70711));
        assert_eq!(n, Tuple::vector(0.0, 0.70711, -0.70711));
    }

    #[test]
    fn computing_the_normal_on_a_transformed_sphere() {
        let s = Sphere::new(scaling(1.0, 0.5, 1.0) * rotation_z(PI / 5.0));
        let n = normal_at(
            &s,
            Tuple::point(0.0, (2.0_f32).sqrt() / 2.0, -(2.0_f32).sqrt() / 2.0),
        );
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
        m.ambient = 1.23456;
        s.material = m.clone();
        assert_eq!(s.material, m);
    }

    #[test]
    fn a_glassy_sphere() {
        let s = Sphere::glass(Matrix::identity());
        assert_eq!(s.material.transparency, 1.0);
        assert_eq!(s.material.refractive_index, 1.5);
    }
}
