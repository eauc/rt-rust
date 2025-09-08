use rt_rust::cameras::Camera;
use rt_rust::colors::Color;
use rt_rust::lights::PointLight;
use rt_rust::materials::Material;
use rt_rust::matrices::Matrix;
use rt_rust::planes::Plane;
use rt_rust::spheres::Sphere;
use rt_rust::transformations::{rotation_z, scaling, translation, view_transform};
use rt_rust::tuples::Tuple;
use rt_rust::worlds::World;
use std::f32::consts::PI;

fn main() {
    let mut wall_material = Material::default();
    wall_material.color = Color::new(1.0, 0.9, 0.9);
    wall_material.specular = 0.0;
    let mut floor = Plane::new(Matrix::identity());
    floor.material = wall_material;
    let mut left_wall = Plane::new(rotation_z(PI / 2.0) * translation(0.0, -5.0, 0.0));
    left_wall.material = wall_material;

    let mut middle = Sphere::new(translation(-0.5, 1.0, 0.5));
    middle.material.color = Color::new(0.1, 1.0, 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;

    let mut right = Sphere::new(translation(1.5, 0.5, -0.5) * scaling(0.5, 0.5, 0.5));
    right.material.color = Color::new(0.5, 1.0, 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;

    let mut left = Sphere::new(translation(-1.5, 0.33, -0.75) * scaling(0.33, 0.33, 0.33));
    left.material.color = Color::new(1.0, 0.8, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;

    let light = PointLight::new(Tuple::point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
    let world = World::new(
        light,
        vec![&floor, &left_wall, &middle, &right, &left],
    );

    let camera = Camera::new(
        500,
        250,
        PI / 3.0,
        view_transform(
            Tuple::point(0.0, 1.5, -5.0),
            Tuple::point(0.0, 1.0, 0.0),
            Tuple::vector(0.0, 1.0, 0.0),
        ),
    );

    let image = camera.render(&world);
    let ppm = image.to_ppm();
    std::fs::write("examples/spheres_planes.ppm", ppm).unwrap();
}
