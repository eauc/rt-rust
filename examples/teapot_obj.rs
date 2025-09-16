use rt_rust::cameras::Camera;
use rt_rust::colors::Color;
use rt_rust::lights::PointLight;
use rt_rust::obj_files::parse_obj_file_path;
use rt_rust::transformations::view_transform;
use rt_rust::tuples::Tuple;
use rt_rust::worlds::World;
use std::f32::consts::PI;

fn main() {
    let teapot_low_obj = parse_obj_file_path("examples/teapot_low.obj");
    let teapot_obj = parse_obj_file_path("examples/teapot.obj");

    let light = PointLight::new(Tuple::point(100.0, 100.0, 100.0), Color::new(1.0, 1.0, 1.0));
    let mut world_low = World::new(light);
    world_low.objects = vec![teapot_low_obj.default_group];
    let mut world = World::new(light);
    world.objects = vec![teapot_obj.default_group];

    let camera = Camera::new(
        800,
        600,
        PI / 3.0,
        view_transform(
            Tuple::point(25.0, 25.0, 25.0),
            Tuple::point(0.0, 0.0, 10.0),
            Tuple::vector(0.0, 0.0, 1.0),
        ),
    );

    let image_low = camera.render(&mut world_low, 5);
    let image = camera.render(&mut world, 5);
    std::fs::write("examples/teapot_low.ppm", image_low.to_ppm()).unwrap();
    std::fs::write("examples/teapot.ppm", image.to_ppm()).unwrap();
}
