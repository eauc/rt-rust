use crate::coordinates::equals;
use crate::intersections::Intersection;
use crate::materials::Material;
use crate::matrices::Matrix;
use crate::rays::Ray;
use crate::shapes::Shape;
use crate::tuples::Tuple;

pub struct Plane {
    pub material: Material,
    transform_inverse: Matrix<4>,
    transform_inverse_transpose: Matrix<4>,
}

impl Plane {
    pub fn new(tranform: Matrix<4>) -> Plane {
        let transform_inverse = tranform.inverse();
        Plane {
            material: Material::default(),
            transform_inverse,
            transform_inverse_transpose: transform_inverse.transpose(),
        }
    }
}

impl Shape for Plane {
    fn material(&self) -> &Material {
        &self.material
    }
    fn transform_inverse(&self) -> Matrix<4> {
        self.transform_inverse
    }
    fn transform_inverse_transpose(&self) -> Matrix<4> {
        self.transform_inverse_transpose
    }
    fn local_intersect<'a>(&'a self, ray: &Ray) -> Vec<crate::intersections::Intersection<'a>> {
        if equals(ray.direction.y(), 0.0) {
            return vec![];
        }
        let t = -ray.origin.y() / ray.direction.y();
        vec![Intersection::new(t, self)]
    }
    fn local_normal_at(&self, _point: Tuple) -> Tuple {
        Tuple::vector(0.0, 1.0, 0.0)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn the_normal_of_a_plane_is_constant_everywhere() {
        let p = Plane::new(Matrix::identity());
        let n1 = p.local_normal_at(Tuple::point(0.0, 0.0, 0.0));
        let n2 = p.local_normal_at(Tuple::point(10.0, 0.0, -10.0));
        let n3 = p.local_normal_at(Tuple::point(-5.0, 0.0, 150.0));
        assert_eq!(n1, Tuple::vector(0.0, 1.0, 0.0));
        assert_eq!(n2, Tuple::vector(0.0, 1.0, 0.0));
        assert_eq!(n3, Tuple::vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn intersect_with_a_ray_parallel_to_the_plane() {
        let p = Plane::new(Matrix::identity());
        let r = Ray::new(Tuple::point(0.0, 10.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = p.local_intersect(&r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn intersect_with_a_ray_coplanar_to_the_plane() {
        let p = Plane::new(Matrix::identity());
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = p.local_intersect(&r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_above() {
        let p = Plane::new(Matrix::identity());
        let r = Ray::new(Tuple::point(0.0, 1.0, 0.0), Tuple::vector(0.0, -1.0, 0.0));
        let xs = p.local_intersect(&r);
        assert_eq!(xs.iter().map(|i| i.t).collect::<Vec<f32>>(), vec![1.0]);
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_below() {
        let p = Plane::new(Matrix::identity());
        let r = Ray::new(Tuple::point(0.0, -1.0, 0.0), Tuple::vector(0.0, 1.0, 0.0));
        let xs = p.local_intersect(&r);
        assert_eq!(xs.iter().map(|i| i.t).collect::<Vec<f32>>(), vec![1.0]);
    }
}
