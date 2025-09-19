use rt_rust::cameras::Camera;
use rt_rust::colors::{Color, WHITE};
use rt_rust::floats::PI;
use rt_rust::lights::Light;
use rt_rust::matrices::Matrix;
use rt_rust::objects::Object;
use rt_rust::transformations::{scaling, translation, view_transform};
use rt_rust::tuples::Tuple;
use rt_rust::worlds::World;

fn main() {
    let mut floor = Object::new_plane().with_transform(Matrix::identity());
    floor.material.ambient = 0.0;
    floor.material.diffuse = 0.8;
    floor.material.reflective = 0.5;

    let mut red = Object::new_sphere().with_transform(translation(0.0, 1.5, 0.0));
    red.material.color = Color::new(1.0, 0.0, 0.0);
    let mut green =
        Object::new_sphere().with_transform(translation(2.0, 1.0, 1.0) * scaling(0.5, 0.5, 0.5));
    green.material.color = Color::new(0.0, 1.0, 0.0);
    let mut blue = Object::new_sphere().with_transform(translation(-3.0, 1.5, -4.0));
    blue.material.color = Color::new(0.0, 0.0, 1.0);

    let light = Light::new_point(Tuple::point(5.0, 5.0, 0.0), WHITE);

    let mut world = World::new();
    world.lights = vec![light];
    world.objects = vec![floor, red, green, blue];

    let mut camera = Camera::new(
        1000,
        800,
        4.0,
        PI / 2.0,
        view_transform(
            Tuple::point(0.0, 1.0, 0.0) + Tuple::vector(1.0, 0.1, 0.0) * 4.0,
            Tuple::point(0.0, 1.0, 0.0),
            Tuple::vector(0.0, 1.0, 0.0),
        ),
    );
    camera.aperture = 0.01;
    camera.blur_oversampling = 20;
    camera.oversampling = 3;
    camera.threads = 8;

    let image = camera.render(&mut world);
    let ppm = image.to_ppm();
    std::fs::write("examples/blur.ppm", ppm).unwrap();
}
