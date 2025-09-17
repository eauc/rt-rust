use rt_rust::cameras::Camera;
use rt_rust::colors::Color;
use rt_rust::lights::Light;
use rt_rust::objects::Object;
use rt_rust::shapes::csg::Operation;
use rt_rust::transformations::{rotation_x, rotation_z, scaling, view_transform};
use rt_rust::tuples::Tuple;
use rt_rust::worlds::World;
use std::f32::consts::PI;

fn main() {
    let mut cyl1 = Object::new_cylinder();
    cyl1.material.color = Color::new(0.2, 1.0, 0.5);
    let mut cyl2 = Object::new_cylinder().with_transform(rotation_x(PI / 2.0));
    cyl2.material.color = Color::new(0.2, 1.0, 0.5);
    let c1 = Object::new_csg(Operation::Union, cyl1, cyl2);
    let mut cyl3 = Object::new_cylinder().with_transform(rotation_z(PI / 2.0));
    cyl3.material.color = Color::new(0.2, 1.0, 0.5);
    let c2 = Object::new_csg(Operation::Union, c1, cyl3);
    let mut cube = Object::new_cube().with_transform(scaling(1.5, 1.5, 1.5));
    cube.material.color = Color::new(1.0, 0.2, 1.0);
    let mut sphere = Object::new_sphere().with_transform(scaling(2.1, 2.1, 2.1));
    sphere.material.color = Color::new(1.0, 1.0, 0.2);
    let c3 = Object::new_csg(Operation::Intersection, sphere, cube);
    let csg = Object::new_csg(Operation::Difference, c3, c2);

    let light = Light::new_point(Tuple::point(10.0, 10.0, 2.0), Color::new(1.0, 1.0, 1.0));
    let mut world = World::new();
    world.lights = vec![light];
    world.objects = vec![csg];

    let camera = Camera::new(
        500,
        300,
        1.0,
        PI / 3.0,
        view_transform(
            Tuple::point(10.0, 4.0, 10.0),
            Tuple::point(0.0, 0.0, 0.0),
            Tuple::vector(0.0, 1.0, 0.0),
        ),
    );

    let image = camera.render(&mut world);
    let ppm = image.to_ppm();
    std::fs::write("examples/csg.ppm", ppm).unwrap();
}
