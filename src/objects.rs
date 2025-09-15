use crate::floats::Float;
use crate::intersections::Intersection;
use crate::materials::Material;
use crate::matrices::Matrix;
use crate::rays::Ray;
use crate::shapes::Shapes;
use crate::shapes::cones::Cone;
use crate::shapes::cubes::Cube;
use crate::shapes::cylinders::Cylinder;
use crate::shapes::planes::Plane;
use crate::shapes::spheres::Sphere;
use crate::tuples::Tuple;

pub struct Object {
    pub material: Material,
    pub transform_inverse: Matrix<4>,
    transform_inverse_transpose: Matrix<4>,
    shape: Shapes,
}

impl Object {
    fn new(shape: Shapes) -> Object {
        Object {
            material: Material::default(),
            transform_inverse: Matrix::identity(),
            transform_inverse_transpose: Matrix::identity(),
            shape,
        }
    }
    pub fn new_cone() -> Object {
        Object::new(Shapes::Cone(Cone::new()))
    }
    pub fn new_cone_truncated(min: Float, max: Float, closed: bool) -> Object {
        let mut cone = Cone::new();
        cone.minimum = min;
        cone.maximum = max;
        cone.closed = closed;
        Object::new(Shapes::Cone(cone))
    }
    pub fn new_cube() -> Object {
        Object::new(Shapes::Cube(Cube::new()))
    }
    pub fn new_cylinder() -> Object {
        Object::new(Shapes::Cylinder(Cylinder::new()))
    }
    pub fn new_cylinder_truncated(min: Float, max: Float, closed: bool) -> Object {
        let mut cylinder = Cylinder::new();
        cylinder.minimum = min;
        cylinder.maximum = max;
        cylinder.closed = closed;
        Object::new(Shapes::Cylinder(cylinder))
    }
    pub fn new_plane() -> Object {
        Object::new(Shapes::Plane(Plane::new()))
    }
    pub fn new_sphere() -> Object {
        Object::new(Shapes::Sphere(Sphere::new()))
    }

    pub fn made_of_glass(self) -> Object {
        Object {
            material: Material::glass(),
            ..self
        }
    }

    pub fn with_transform(self, transform: Matrix<4>) -> Object {
        let transform_inverse = transform.inverse();
        Object {
            transform_inverse,
            transform_inverse_transpose: transform_inverse.transpose(),
            ..self
        }
    }

    pub fn intersect<'a>(&'a self, ray: &Ray) -> Vec<Intersection<'a>> {
        let local_ray = ray.transform(self.transform_inverse);
        self.shape
            .local_intersect(&local_ray)
            .iter()
            .map(|t| Intersection::new(*t, self))
            .collect()
    }

    pub fn normal_at(&self, world_point: Tuple) -> Tuple {
        let local_point = self.transform_inverse * world_point;
        let local_normal = self.shape.local_normal_at(local_point);
        let mut world_normal = self.transform_inverse_transpose * local_normal;
        world_normal.to_vector();
        world_normal.normalize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shapes::TestShape;
    use crate::transformations::{scaling, translation};

    fn new_test() -> Object {
        Object::new(Shapes::Test(TestShape))
    }

    #[test]
    fn the_default_material() {
        let s = new_test().with_transform(Matrix::identity());
        assert_eq!(s.material, Material::default());
    }

    #[test]
    fn the_default_transformation() {
        let s = new_test().with_transform(Matrix::identity());
        assert_eq!(s.transform_inverse, Matrix::identity());
    }

    #[test]
    fn assigning_a_transformation() {
        let s = new_test().with_transform(translation(2.0, 3.0, 4.0));
        assert_eq!(s.transform_inverse, translation(2.0, 3.0, 4.0).inverse());
    }

    #[test]
    fn intersecting_a_scaled_shape_with_a_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let o = new_test().with_transform(scaling(2.0, 2.0, 2.0));
        let xs = o.intersect(&r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn computing_the_normal_on_a_translated_shape() {
        let o = new_test().with_transform(translation(0.0, 1.0, 0.0));
        let n = o.normal_at(Tuple::point(0.0, 1.70711, -0.70711));
        assert_eq!(n, Tuple::vector(0.0, 0.70711, -0.70711));
    }
}
