use rt_rust::cameras::Camera;
use rt_rust::colors::Color;
use rt_rust::floats::Float;
use rt_rust::lights::PointLight;
use rt_rust::objects::Object;
use rt_rust::transformations::{rotation_y, rotation_z, scaling, translation, view_transform};
use rt_rust::tuples::Tuple;
use rt_rust::worlds::World;
use std::f32::consts::PI;

fn main() {
    let hexagon = hexagon();

    let light = PointLight::new(Tuple::point(0.0, 50.0, 25.0), Color::new(1.0, 0.2, 1.0));
    let mut world = World::new();
    world.lights = vec![light];
    world.objects = vec![hexagon];

    let camera = Camera::new(
        500,
        300,
        PI / 3.0,
        view_transform(
            Tuple::point(3.0, 3.0, 3.0),
            Tuple::point(0.0, 0.0, 0.0),
            Tuple::vector(0.0, 1.0, 0.0),
        ),
    );

    let image = camera.render(&mut world, 5);
    let ppm = image.to_ppm();
    std::fs::write("examples/hexagon_group.ppm", ppm).unwrap();
}

fn hexagon_corner() -> Object {
    Object::new_sphere().with_transform(translation(0.0, 0.0, -1.0) * scaling(0.25, 0.25, 0.25))
}

fn hexagon_edge() -> Object {
    let mut cyl = Object::new_cylinder().with_transform(
        translation(0.0, 0.0, -1.0)
            * rotation_y(-PI / 6.0)
            * rotation_z(-PI / 2.0)
            * scaling(0.25, 1.0, 0.25),
    );
    cyl.as_mut_cylinder().truncate(0.0, 1.0, false);
    cyl
}

fn hexagon_side() -> Object {
    let mut g = Object::new_group();
    g.as_mut_group().add_child(hexagon_corner());
    g.as_mut_group().add_child(hexagon_edge());
    g
}

fn hexagon() -> Object {
    let mut hex = Object::new_group();
    for n in 0..6 {
        let rot = (n as Float) * PI / 3.0;
        let side = hexagon_side().with_transform(rotation_y(rot));
        hex.as_mut_group().add_child(side);
    }
    hex
}
