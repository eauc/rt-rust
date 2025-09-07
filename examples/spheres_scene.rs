use rt_rust::cameras::Camera;
use rt_rust::colors::Color;
use rt_rust::lights::PointLight;
use rt_rust::materials::Material;
use rt_rust::spheres::Sphere;
use rt_rust::transformations;
use rt_rust::tuples::Tuple;
use rt_rust::worlds::World;
use std::f32::consts::PI;

fn main() {
    let mut wall_material = Material::new();
    wall_material.color = Color(1.0, 0.9, 0.9);
    wall_material.specular = 0.0;
    let mut floor = Sphere::new();
    floor.transform = transformations::scaling(10.0, 0.01, 10.0);
    floor.material = wall_material;
    let mut left_wall = Sphere::new();
    left_wall.transform = transformations::translation(0.0, 0.0, 5.0)
        * transformations::rotation_y(-PI / 4.0)
        * transformations::rotation_x(PI / 2.0)
        * transformations::scaling(10.0, 0.01, 10.0);
    left_wall.material = wall_material;
    let mut right_wall = Sphere::new();
    right_wall.transform = transformations::translation(0.0, 0.0, 5.0)
        * transformations::rotation_y(PI / 4.0)
        * transformations::rotation_x(PI / 2.0)
        * transformations::scaling(10.0, 0.01, 10.0);
    right_wall.material = wall_material;

    let mut middle = Sphere::new();
    middle.transform = transformations::translation(-0.5, 1.0, 0.5);
    middle.material.color = Color(0.1, 1.0, 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;

    let mut right = Sphere::new();
    right.transform =
        transformations::translation(1.5, 0.5, -0.5) * transformations::scaling(0.5, 0.5, 0.5);
    right.material.color = Color(0.5, 1.0, 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;

    let mut left = Sphere::new();
    left.transform = transformations::translation(-1.5, 0.33, -0.75)
        * transformations::scaling(0.33, 0.33, 0.33);
    left.material.color = Color(1.0, 0.8, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;

    let light = PointLight::new(Tuple::point(-10.0, 10.0, -10.0), Color(1.0, 1.0, 1.0));
    let world = World {
        light: light,
        objects: vec![floor, left_wall, right_wall, middle, right, left],
    };

    let mut camera = Camera::new(300, 150, PI / 3.0);
    camera.transform = transformations::view_transform(
        Tuple::point(0.0, 1.5, -5.0),
        Tuple::point(0.0, 1.0, 0.0),
        Tuple::vector(0.0, 1.0, 0.0),
    );

    let image = camera.render(&world);
    let ppm = image.to_ppm();
    std::fs::write("examples/spheres_scene.ppm", ppm).unwrap();
}
