use crate::intersections::Intersection;
use crate::materials::Material;
use crate::matrices::Matrix;
use crate::rays::Ray;
use crate::tuples::Tuple;

pub mod cubes;
pub mod cylinders;
pub mod planes;
pub mod spheres;

pub trait Shape {
    fn material(&self) -> &Material;
    fn transform_inverse(&self) -> Matrix<4>;
    fn transform_inverse_transpose(&self) -> Matrix<4>;
    fn local_intersect<'a>(&'a self, ray: &Ray) -> Vec<Intersection<'a>>;
    fn local_normal_at(&self, local_point: Tuple) -> Tuple;
}

pub fn intersect<'a>(shape: &'a dyn Shape, ray: &Ray) -> Vec<Intersection<'a>> {
    let local_ray = ray.transform(shape.transform_inverse());
    shape.local_intersect(&local_ray)
}

pub fn normal_at(shape: &dyn Shape, world_point: Tuple) -> Tuple {
    let local_point = shape.transform_inverse() * world_point;
    let local_normal = shape.local_normal_at(local_point);
    let mut world_normal = shape.transform_inverse_transpose() * local_normal;
    world_normal.to_vector();
    world_normal.normalize()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transformations::{scaling, translation};

    struct TestShape {
        material: Material,
        transform_inverse: Matrix<4>,
        transform_inverse_transpose: Matrix<4>,
    }

    impl TestShape {
        fn new(transform: Matrix<4>) -> TestShape {
            TestShape {
                material: Material::default(),
                transform_inverse: transform.inverse(),
                transform_inverse_transpose: transform.inverse().transpose(),
            }
        }
    }

    impl Shape for TestShape {
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
            assert_eq!(
                ray,
                &Ray::new(Tuple::point(0.0, 0.0, -2.5), Tuple::vector(0.0, 0.0, 0.5))
            );
            vec![]
        }
        fn local_normal_at(&self, local_point: Tuple) -> Tuple {
            let mut result = local_point;
            result.to_vector();
            result
        }
    }

    #[test]
    fn the_default_material() {
        let s = TestShape::new(Matrix::identity());
        assert_eq!(s.material(), &Material::default());
    }

    #[test]
    fn the_default_transformation() {
        let s = TestShape::new(Matrix::identity());
        assert_eq!(s.transform_inverse(), Matrix::identity());
    }

    #[test]
    fn assigning_a_transformation() {
        let s = TestShape::new(translation(2.0, 3.0, 4.0));
        assert_eq!(s.transform_inverse(), translation(2.0, 3.0, 4.0).inverse());
    }

    #[test]
    fn intersecting_a_scaled_shape_with_a_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = TestShape::new(scaling(2.0, 2.0, 2.0));
        let xs = intersect(&s, &r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn computing_the_normal_on_a_translated_shape() {
        let s = TestShape::new(translation(0.0, 1.0, 0.0));
        let n = normal_at(&s, Tuple::point(0.0, 1.70711, -0.70711));
        assert_eq!(n, Tuple::vector(0.0, 0.70711, -0.70711));
    }
}
