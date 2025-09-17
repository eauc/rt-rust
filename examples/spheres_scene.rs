use rt_rust::cameras::Camera;
use rt_rust::colors::Color;
use rt_rust::lights::Light;
use rt_rust::materials::Material;
use rt_rust::objects::Object;
use rt_rust::transformations::{rotation_x, rotation_y, scaling, translation, view_transform};
use rt_rust::tuples::Tuple;
use rt_rust::worlds::World;
use std::f32::consts::PI;

fn main() {
    let mut wall_material = Material::default();
    wall_material.color = Color::new(1.0, 0.9, 0.9);
    wall_material.specular = 0.0;
    let mut floor = Object::new_sphere().with_transform(scaling(10.0, 0.01, 10.0));
    floor.material = wall_material.clone();
    let mut left_wall = Object::new_sphere().with_transform(
        translation(0.0, 0.0, 5.0)
            * rotation_y(-PI / 4.0)
            * rotation_x(PI / 2.0)
            * scaling(10.0, 0.01, 10.0),
    );
    left_wall.material = wall_material.clone();
    let mut right_wall = Object::new_sphere().with_transform(
        translation(0.0, 0.0, 5.0)
            * rotation_y(PI / 4.0)
            * rotation_x(PI / 2.0)
            * scaling(10.0, 0.01, 10.0),
    );
    right_wall.material = wall_material.clone();

    let mut middle = Object::new_sphere().with_transform(translation(-0.5, 1.0, 0.5));
    middle.material.color = Color::new(0.1, 1.0, 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;

    let mut right =
        Object::new_sphere().with_transform(translation(1.5, 0.5, -0.5) * scaling(0.5, 0.5, 0.5));
    right.material.color = Color::new(0.5, 1.0, 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;

    let mut left = Object::new_sphere()
        .with_transform(translation(-1.5, 0.33, -0.75) * scaling(0.33, 0.33, 0.33));
    left.material.color = Color::new(1.0, 0.8, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;

    let light = Light::new_point(Tuple::point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
    let mut world = World::new();
    world.lights = vec![light];
    world.objects = vec![floor, left_wall, right_wall, middle, right, left];

    let camera = Camera::new(
        300,
        150,
        1.0,
        PI / 3.0,
        view_transform(
            Tuple::point(0.0, 1.5, -5.0),
            Tuple::point(0.0, 1.0, 0.0),
            Tuple::vector(0.0, 1.0, 0.0),
        ),
    );

    let image = camera.render(&mut world);
    let ppm = image.to_ppm();
    std::fs::write("examples/spheres_scene.ppm", ppm).unwrap();
}
