use crate::bounds::Bounds;
use crate::intersections::Intersection;
use crate::materials::Material;
use crate::matrices::Matrix;
use crate::rays::Ray;
use crate::shapes::Shapes;
use crate::shapes::cones::Cone;
use crate::shapes::cubes::Cube;
use crate::shapes::cylinders::Cylinder;
use crate::shapes::groups::Group;
use crate::shapes::planes::Plane;
use crate::shapes::spheres::Sphere;
use crate::tuples::Tuple;

pub struct Object {
    pub material: Material,
    pub transform: Matrix<4>,
    pub transform_inverse: Matrix<4>,
    pub world_to_object: Matrix<4>,
    pub object_to_world: Matrix<4>,
    pub bounds: Bounds,
    shape: Shapes,
}

impl Object {
    fn new(shape: Shapes) -> Object {
        Object {
            material: Material::default(),
            transform: Matrix::identity(),
            transform_inverse: Matrix::identity(),
            world_to_object: Matrix::identity(),
            object_to_world: Matrix::identity(),
            bounds: Bounds::default(),
            shape,
        }
    }
    pub fn new_cone() -> Object {
        Object::new(Shapes::Cone(Cone::new()))
    }
    pub fn new_cube() -> Object {
        Object::new(Shapes::Cube(Cube::new()))
    }
    pub fn new_cylinder() -> Object {
        Object::new(Shapes::Cylinder(Cylinder::new()))
    }
    pub fn new_group() -> Object {
        Object::new(Shapes::Group(Group::new()))
    }
    pub fn new_plane() -> Object {
        Object::new(Shapes::Plane(Plane::new()))
    }
    pub fn new_sphere() -> Object {
        Object::new(Shapes::Sphere(Sphere::new()))
    }

    pub fn as_cone(&self) -> &Cone {
        match &self.shape {
            Shapes::Cone(cone) => cone,
            _ => panic!("This object is not a cone !"),
        }
    }
    pub fn as_mut_cone(&mut self) -> &mut Cone {
        match &mut self.shape {
            Shapes::Cone(cone) => cone,
            _ => panic!("This object is not a cone !"),
        }
    }
    pub fn as_cube(&self) -> &Cube {
        match &self.shape {
            Shapes::Cube(cube) => cube,
            _ => panic!("This object is not a cube !"),
        }
    }
    pub fn as_cylinder(&self) -> &Cylinder {
        match &self.shape {
            Shapes::Cylinder(cylinder) => cylinder,
            _ => panic!("This object is not a cylinder !"),
        }
    }
    pub fn as_mut_cylinder(&mut self) -> &mut Cylinder {
        match &mut self.shape {
            Shapes::Cylinder(cylinder) => cylinder,
            _ => panic!("This object is not a cylinder !"),
        }
    }
    pub fn as_group(&self) -> &Group {
        match &self.shape {
            Shapes::Group(group) => group,
            _ => panic!("This object is not a group !"),
        }
    }
    pub fn as_mut_group(&mut self) -> &mut Group {
        match &mut self.shape {
            Shapes::Group(group) => group,
            _ => panic!("This object is not a group !"),
        }
    }
    pub fn as_plane(&self) -> &Plane {
        match &self.shape {
            Shapes::Plane(plane) => plane,
            _ => panic!("This object is not a plane !"),
        }
    }
    pub fn as_sphere(&self) -> &Sphere {
        match &self.shape {
            Shapes::Sphere(sphere) => sphere,
            _ => panic!("This object is not a sphere !"),
        }
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
            transform,
            transform_inverse,
            world_to_object: transform_inverse,
            object_to_world: transform_inverse.transpose(),
            ..self
        }
    }

    pub fn prepare(&mut self) {
        self.prepare_bounds();
        self.prepare_transform();
    }
    pub fn prepare_bounds(&mut self) {
        self.shape.prepare_bounds(&mut self.bounds);
    }
    pub fn prepare_transform(&mut self) {
        self.shape.prepare_transform(&self.world_to_object, &self.object_to_world);
    }

    pub fn world_to_object(&self, world_point: Tuple) -> Tuple {
        self.world_to_object * world_point
    }

    pub fn normal_to_world(&self, object_normal: Tuple) -> Tuple {
        let mut n = self.object_to_world * object_normal;
        n.to_vector();
        n.normalize()
    }

    pub fn intersect<'b>(&'b self, ray: &Ray) -> Vec<Intersection<'b>> {
        let local_ray = ray.transform(self.transform_inverse);
        self.shape.local_intersect(&local_ray, self)
    }

    pub fn normal_at(&self, world_point: Tuple) -> Tuple {
        let local_point = self.world_to_object(world_point);
        let local_normal = self.shape.local_normal_at(local_point);
        self.normal_to_world(local_normal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shapes::TestShape;
    use crate::transformations::{rotation_y, scaling, translation};
    use std::f32::consts::PI;

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

    #[test]
    fn converting_a_point_from_world_to_object_space() {
        let s = Object::new_sphere().with_transform(translation(5.0, 0.0, 0.0));
        let mut g2 = Object::new_group().with_transform(scaling(2.0, 2.0, 2.0));
        g2.as_mut_group().add_child(s);
        let mut g1 = Object::new_group().with_transform(rotation_y(PI / 2.0));
        g1.as_mut_group().add_child(g2);
        g1.prepare();
        let s = &g1.as_group().children[0].as_group().children[0];
        let p = s.world_to_object(Tuple::point(-2.0, 0.0, -10.0));
        assert_eq!(p, Tuple::point(0.0, 0.0, -1.0));
    }

    #[test]
    fn converting_a_normal_from_object_to_world_space() {
        let s = Object::new_sphere().with_transform(translation(5.0, 0.0, 0.0));
        let mut g2 = Object::new_group().with_transform(scaling(1.0, 2.0, 3.0));
        g2.as_mut_group().add_child(s);
        let mut g1 = Object::new_group().with_transform(rotation_y(PI / 2.0));
        g1.as_mut_group().add_child(g2);
        g1.prepare();
        let s = &g1.as_group().children[0].as_group().children[0];
        let v = s.normal_to_world(Tuple::vector(
            (3.0_f32).sqrt() / 3.0,
            (3.0_f32).sqrt() / 3.0,
            (3.0_f32).sqrt() / 3.0,
        ));
        assert_eq!(v, Tuple::vector(0.28571427, 0.42857143, -0.8571));
    }

    #[test]
    fn finding_the_normal_on_a_child_object() {
        let s = Object::new_sphere().with_transform(translation(5.0, 0.0, 0.0));
        let mut g2 = Object::new_group().with_transform(scaling(1.0, 2.0, 3.0));
        g2.as_mut_group().add_child(s);
        let mut g1 = Object::new_group().with_transform(rotation_y(PI / 2.0));
        g1.as_mut_group().add_child(g2);
        g1.prepare();
        let s = &g1.as_group().children[0].as_group().children[0];
        let v = s.normal_at(Tuple::point(1.7321, 1.1547, -5.5774));
        assert_eq!(v, Tuple::vector(0.28571427, 0.42857143, -0.8571));
    }
}
