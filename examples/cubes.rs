use rt_rust::cameras::Camera;
use rt_rust::colors::Color;
use rt_rust::lights::Light;
use rt_rust::objects::Object;
use rt_rust::transformations::{rotation_y, rotation_z, scaling, translation, view_transform};
use rt_rust::tuples::Tuple;
use rt_rust::worlds::World;
use std::f32::consts::PI;

fn main() {
    let mut floor = Object::new_plane().with_transform(translation(0.0, -2.0, 0.0));
    floor.material.specular = 0.0;
    floor.material.diffuse = 0.2;
    floor.material.ambient = 0.0;
    floor.material.reflective = 0.9;

    let mut middle = Object::new_cube().with_transform(translation(0.0, 0.0, 0.0));
    middle.material.color = Color::new(1.0, 0.5, 0.2);
    let mut left = Object::new_cube().with_transform(
        translation(3.0, 0.0, 0.0)
            * scaling(0.5, 0.5, 0.5)
            * rotation_z(PI / 6.0)
            * rotation_y(PI / 6.0),
    );
    left.material.color = Color::new(0.2, 1.0, 0.5);
    let mut right = Object::new_cube().with_transform(
        translation(0.0, 0.0, 3.0)
            * scaling(0.75, 0.75, 0.75)
            * rotation_z(PI / 6.0)
            * rotation_y(PI / 6.0),
    );
    right.material.color = Color::new(0.2, 0.5, 1.0);

    let light = Light::new_point(Tuple::point(10.0, 10.0, 2.0), Color::new(1.0, 1.0, 1.0));
    let mut world = World::new();
    world.lights = vec![light];
    world.objects = vec![floor, middle, left, right];

    let camera = Camera::new(
        500,
        300,
        PI / 3.0,
        view_transform(
            Tuple::point(10.0, 4.0, 10.0),
            Tuple::point(0.0, 0.0, 0.0),
            Tuple::vector(0.0, 1.0, 0.0),
        ),
    );

    let image = camera.render(&mut world, 5);
    let ppm = image.to_ppm();
    std::fs::write("examples/cubes.ppm", ppm).unwrap();
}
