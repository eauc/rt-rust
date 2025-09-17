use rt_rust::cameras::Camera;
use rt_rust::colors::WHITE;
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

    let light = Light::new_spot(
        Tuple::point(3.0, 5.0, 3.0),
        WHITE,
        Tuple::point(0.0, 1.5, 0.0) - Tuple::point(3.0, 5.0, 3.0),
        PI / 6.0,
        0.8,
    );

    let mut world = World::new();
    world.lights = vec![light];
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

    let image = camera.render(&mut world);
    let ppm = image.to_ppm();
    std::fs::write("examples/spot_lights.ppm", ppm).unwrap();
}
