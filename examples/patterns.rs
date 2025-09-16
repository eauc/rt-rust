use rt_rust::cameras::Camera;
use rt_rust::colors::Color;
use rt_rust::lights::PointLight;
use rt_rust::objects::Object;
use rt_rust::patterns::Pattern;
use rt_rust::transformations::{rotation_y, scaling, translation, view_transform};
use rt_rust::tuples::Tuple;
use rt_rust::worlds::World;
use std::f32::consts::PI;

fn main() {
    let mut floor = Object::new_plane();
    floor.material.pattern = Some(Pattern::new_ring(
        Color::new(0.1, 1.0, 0.1),
        Color::new(0.1, 0.1, 1.0),
    ));

    let gradient = Pattern::new_gradient(Color::new(1.0, 0.0, 0.0), Color::new(0.0, 0.0, 1.0))
        .with_transform(
            translation(-1.0, 0.0, 0.0) * rotation_y(PI / 4.0) * scaling(2.0, 1.0, 1.0),
        );
    let mut middle = Object::new_sphere().with_transform(translation(0.0, 1.5, 0.0));
    middle.material.pattern = Some(gradient);

    let stripes = Pattern::new_stripe(Color::new(1.0, 1.0, 0.0), Color::new(0.2, 0.6, 1.0))
        .with_transform(scaling(0.5, 1.0, 1.0));
    let mut left =
        Object::new_sphere().with_transform(scaling(0.5, 0.5, 0.5) * translation(5.0, 1.5, 0.0));
    left.material.pattern = Some(stripes);

    let checkers = Pattern::new_checker(Color::new(1.0, 0.2, 0.6), Color::new(0.0, 1.0, 0.0));
    let mut right =
        Object::new_sphere().with_transform(scaling(0.6, 0.6, 0.6) * translation(0.0, 1.5, 4.0));
    right.material.pattern = Some(checkers);

    let light = PointLight::new(Tuple::point(10.0, 10.0, 0.0), Color::new(1.0, 1.0, 1.0));
    let mut world = World::new(light);
    world.objects = vec![floor, middle, left, right];

    let camera = Camera::new(
        500,
        400,
        PI / 3.0,
        view_transform(
            Tuple::point(5.0, 3.0, 5.0),
            Tuple::point(0.0, 0.0, 0.0),
            Tuple::vector(0.0, 1.0, 0.0),
        ),
    );

    let image = camera.render(&mut world, 0);
    let ppm = image.to_ppm();
    std::fs::write("examples/patterns.ppm", ppm).unwrap();
}
