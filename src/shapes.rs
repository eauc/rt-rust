use crate::coordinates::Coordinate;
use crate::rays::Ray;
use crate::tuples::Tuple;

pub mod cones;
pub mod cubes;
pub mod cylinders;
pub mod planes;
pub mod spheres;

pub enum Shapes {
    Cone(cones::Cone),
    Cube(cubes::Cube),
    Cylinder(cylinders::Cylinder),
    Plane(planes::Plane),
    Sphere(spheres::Sphere),
    Test(TestShape),
}

impl Shapes {
    pub fn local_intersect<'a>(&'a self, ray: &Ray) -> Vec<Coordinate> {
        match self {
            Shapes::Cone(cone) => cone.local_intersect(ray),
            Shapes::Cube(cube) => cube.local_intersect(ray),
            Shapes::Cylinder(cylinder) => cylinder.local_intersect(ray),
            Shapes::Plane(plane) => plane.local_intersect(ray),
            Shapes::Sphere(sphere) => sphere.local_intersect(ray),
            Shapes::Test(test) => test.local_intersect(ray),
        }
    }

    pub fn local_normal_at(&self, point: Tuple) -> Tuple {
        match self {
            Shapes::Cone(cone) => cone.local_normal_at(point),
            Shapes::Cube(cube) => cube.local_normal_at(point),
            Shapes::Cylinder(cylinder) => cylinder.local_normal_at(point),
            Shapes::Plane(plane) => plane.local_normal_at(point),
            Shapes::Sphere(sphere) => sphere.local_normal_at(point),
            Shapes::Test(test) => test.local_normal_at(point),
        }
    }
}

pub struct TestShape;

impl TestShape {
    fn local_intersect<'a>(&'a self, ray: &Ray) -> Vec<Coordinate> {
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
