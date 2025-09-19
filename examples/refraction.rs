use rt_rust::cameras::Camera;
use rt_rust::colors::{Color, BLACK, WHITE};
use rt_rust::floats::PI;
use rt_rust::lights::Light;
use rt_rust::matrices::Matrix;
use rt_rust::objects::Object;
use rt_rust::patterns::Pattern;
use rt_rust::transformations::{scaling, translation, view_transform};
use rt_rust::tuples::Tuple;
use rt_rust::worlds::World;

fn main() {
    let mut floor = Object::new_plane().with_transform(translation(0.0, -2.0, 0.0));
    floor.material.pattern = Some(Pattern::new_checker(BLACK, WHITE));

    let mut glass = Object::new_sphere()
        .made_of_glass()
        .with_transform(Matrix::identity());
    glass.material.ambient = 0.0;
    glass.material.diffuse = 0.2;
    glass.material.specular = 0.9;
    glass.material.shininess = 300.0;
    glass.material.refractive_index = 1.5;
    glass.material.transparency = 0.9;
    glass.material.reflective = 0.9;
    let mut air = Object::new_sphere().with_transform(scaling(0.6, 0.6, 0.6));
    air.material.ambient = 0.0;
    air.material.diffuse = 0.0;
    air.material.specular = 0.0;
    air.material.reflective = 0.9;
    air.material.transparency = 1.0;
    air.material.refractive_index = 1.0;

    let light = Light::new_point(Tuple::point(0.0, 100.0, 100.0), Color::new(1.0, 1.0, 1.0));
    let mut world = World::new();
    world.lights = vec![light];
    world.objects = vec![floor, glass, air];

    let mut camera = Camera::new(
        1000,
        800,
        1.0,
        PI / 3.0,
        view_transform(
            Tuple::point(0.0, 6.0, 0.0),
            Tuple::point(0.0, 0.0, 0.0),
            Tuple::vector(0.0, 0.0, 1.0),
        ),
    );
    camera.oversampling = 4;
    camera.render_depth = 10;
    camera.threads = 8;

    let image = camera.render(&mut world);
    let ppm = image.to_ppm();
    std::fs::write("examples/refraction.ppm", ppm).unwrap();
}
