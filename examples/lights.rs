use rt_rust::cameras::Camera;
use rt_rust::colors::Color;
use rt_rust::lights::Light;
use rt_rust::matrices::Matrix;
use rt_rust::objects::Object;
use rt_rust::transformations::{translation, view_transform};
use rt_rust::tuples::Tuple;
use rt_rust::worlds::World;
use std::f32::consts::PI;

fn main() {
    let floor = Object::new_plane().with_transform(Matrix::identity());

    let sphere = Object::new_sphere().with_transform(translation(0.0, 1.5, 0.0));

    let red = Light::new_point(Tuple::point(0.0, 10.0, 0.0), Color::new(1.0, 0.0, 0.0));
    let blue = Light::new_point(Tuple::point(0.0, 10.0, 10.0), Color::new(0.0, 0.0, 1.0));
    let green = Light::new_point(Tuple::point(10.0, 10.0, 0.0), Color::new(0.0, 1.0, 0.0));

    let mut world = World::new();
    world.lights = vec![red, blue, green];
    world.objects = vec![floor, sphere];

    let camera = Camera::new(
        500,
        400,
        PI / 3.0,
        view_transform(
            Tuple::point(5.0, 3.0, 5.0),
            Tuple::point(0.0, 1.0, 0.0),
            Tuple::vector(0.0, 1.0, 0.0),
        ),
    );

    let image = camera.render(&mut world, 5);
    let ppm = image.to_ppm();
    std::fs::write("examples/lights.ppm", ppm).unwrap();
}
