use crate::bounds::Bounds;
use crate::intersections::Intersection;
use crate::matrices::Matrix;
use crate::objects::Object;
use crate::rays::Ray;
use crate::tuples::Tuple;

pub mod cones;
pub mod cubes;
pub mod cylinders;
pub mod groups;
pub mod planes;
pub mod spheres;
pub mod triangles;

pub enum Shapes {
    Cone(cones::Cone),
    Cube(cubes::Cube),
    Cylinder(cylinders::Cylinder),
    Group(groups::Group),
    Plane(planes::Plane),
    Sphere(spheres::Sphere),
    Test(TestShape),
    Triangle(triangles::Triangle),
}

impl Shapes {
    pub fn prepare_bounds(&mut self, bounds: &mut Bounds) {
        match self {
            Shapes::Cone(cone) => cone.prepare_bounds(bounds),
            Shapes::Cube(_) => (),
            Shapes::Cylinder(cylinder) => cylinder.prepare_bounds(bounds),
            Shapes::Group(group) => group.prepare_bounds(bounds),
            Shapes::Plane(plane) => plane.prepare_bounds(bounds),
            Shapes::Sphere(_) => (),
            Shapes::Test(_) => (),
            Shapes::Triangle(triangle) => triangle.prepare_bounds(bounds),
        }
    }
    pub fn prepare_transform(&mut self, world_to_object: &Matrix<4>, object_to_world: &Matrix<4>) {
        match self {
            Shapes::Group(group) => group.prepare_transform(world_to_object, object_to_world),
            _ => (),
        }
    }

    pub fn local_intersect<'a>(&'a self, ray: &Ray, object: &'a Object) -> Vec<Intersection<'a>> {
        match self {
            Shapes::Cone(cone) => cone.local_intersect(ray, object),
            Shapes::Cube(cube) => cube.local_intersect(ray, object),
            Shapes::Cylinder(cylinder) => cylinder.local_intersect(ray, object),
            Shapes::Group(group) => group.local_intersect(ray, object),
            Shapes::Plane(plane) => plane.local_intersect(ray, object),
            Shapes::Sphere(sphere) => sphere.local_intersect(ray, object),
            Shapes::Test(test) => test.local_intersect(ray, object),
            Shapes::Triangle(triangle) => triangle.local_intersect(ray, object),
        }
    }

    pub fn local_normal_at(&self, point: Tuple) -> Tuple {
        match self {
            Shapes::Cone(cone) => cone.local_normal_at(point),
            Shapes::Cube(cube) => cube.local_normal_at(point),
            Shapes::Cylinder(cylinder) => cylinder.local_normal_at(point),
            Shapes::Group(group) => group.local_normal_at(point),
            Shapes::Plane(plane) => plane.local_normal_at(point),
            Shapes::Sphere(sphere) => sphere.local_normal_at(point),
            Shapes::Test(test) => test.local_normal_at(point),
            Shapes::Triangle(triangle) => triangle.local_normal_at(point),
        }
    }
}

pub struct TestShape;

impl TestShape {
    fn local_intersect<'a>(&'a self, ray: &Ray, _object: &'a Object) -> Vec<Intersection<'a>> {
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
