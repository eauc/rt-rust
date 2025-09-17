use rt_rust::cameras::Camera;
use rt_rust::colors::{Color, WHITE};
use rt_rust::lights::Light;
use rt_rust::materials::Material;
use rt_rust::objects::Object;
use rt_rust::transformations::{rotation_x, scaling, translation, view_transform};
use rt_rust::tuples::Tuple;
use rt_rust::worlds::World;

fn main() {
    let mut white_material = Material::default();
    white_material.color = Color::new(1.0, 1.0, 1.0);
    white_material.diffuse = 0.7;
    white_material.ambient = 0.1;
    white_material.specular = 0.0;
    white_material.reflective = 0.1;
    let mut blue_material = white_material.clone();
    blue_material.color = Color::new(0.537, 0.831, 0.914);
    let mut red_material = white_material.clone();
    red_material.color = Color::new(0.941, 0.322, 0.388);
    let mut purple_material = white_material.clone();
    purple_material.color = Color::new(0.373, 0.404, 0.550);

    let standard_transform = scaling(0.5, 0.5, 0.5) * translation(1.0, -1.0, 1.0);
    let large_object = scaling(3.5, 3.5, 3.5) * standard_transform;
    let medium_object = scaling(3.0, 3.0, 3.0) * standard_transform;
    let small_object = scaling(2.0, 2.0, 2.0) * standard_transform;

    let mut bg = Object::new_plane()
        .with_transform(translation(0.0, 0.0, 500.0) * rotation_x(1.5707963267948966));
    bg.material.color = WHITE;
    bg.material.ambient = 1.0;
    bg.material.diffuse = 0.0;
    bg.material.specular = 0.0;

    let mut sphere = Object::new_sphere().with_transform(large_object);
    sphere.material.color = Color::new(0.373, 0.404, 0.550);
    sphere.material.diffuse = 0.2;
    sphere.material.ambient = 0.0;
    sphere.material.specular = 1.0;
    sphere.material.shininess = 200.0;
    sphere.material.reflective = 0.7;
    sphere.material.transparency = 0.7;
    sphere.material.refractive_index = 1.5;

    let mut c1 = Object::new_cube().with_transform(translation(4.0, 0.0, 0.0) * medium_object);
    c1.material = white_material;
    let mut c2 = Object::new_cube().with_transform(translation(8.5, 1.5, -0.5) * large_object);
    c2.material = blue_material;
    let mut c3 = Object::new_cube().with_transform(translation(0.0, 0.0, 4.0) * large_object);
    c3.material = red_material;
    let mut c4 = Object::new_cube().with_transform(translation(4.0, 0.0, 4.0) * small_object);
    c4.material = white_material;
    let mut c5 = Object::new_cube().with_transform(translation(7.5, 0.5, 4.0) * medium_object);
    c5.material = purple_material;
    let mut c6 = Object::new_cube().with_transform(translation(-0.25, 0.25, 8.0) * medium_object);
    c6.material = white_material;
    let mut c7 = Object::new_cube().with_transform(translation(4.0, 1.0, 7.5) * large_object);
    c7.material = blue_material;
    let mut c8 = Object::new_cube().with_transform(translation(10.0, 2.0, 7.5) * medium_object);
    c8.material = red_material;
    let mut c9 = Object::new_cube().with_transform(translation(8.0, 2.0, 12.0) * small_object);
    c9.material = white_material;
    let mut c10 = Object::new_cube().with_transform(translation(20.0, 1.0, 9.0) * small_object);
    c10.material = white_material;
    let mut c11 = Object::new_cube().with_transform(translation(-0.5, -5.0, 0.25) * large_object);
    c11.material = blue_material;
    let mut c12 = Object::new_cube().with_transform(translation(4.0, -4.0, 0.0) * large_object);
    c12.material = red_material;
    let mut c13 = Object::new_cube().with_transform(translation(8.5, -4.0, 0.0) * large_object);
    c13.material = white_material;
    let mut c14 = Object::new_cube().with_transform(translation(0.0, -4.0, 4.0) * large_object);
    c14.material = white_material;
    let mut c15 = Object::new_cube().with_transform(translation(-0.5, -4.5, 8.0) * large_object);
    c15.material = purple_material;
    let mut c16 = Object::new_cube().with_transform(translation(0.0, -8.0, 4.0) * large_object);
    c16.material = white_material;
    let mut c17 = Object::new_cube().with_transform(translation(-0.5, -8.5, 8.0) * large_object);
    c17.material = white_material;

    let light = Light::new_point(Tuple::point(50.0, 100.0, -50.0), WHITE);
    let light2 = Light::new_point(Tuple::point(-400.0, 50.0, -10.0), Color::new(0.2, 0.2, 0.2));

    let mut world = World::new();
    world.lights = vec![light, light2];
    world.objects = vec![
        bg, sphere, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15, c16, c17,
    ];

    let mut camera = Camera::new(
        1000,
        1000,
        1.0,
        0.785,
        view_transform(
            Tuple::point(-6.0, 6.0, -10.0),
            Tuple::point(6.0, 0.0, 6.0),
            Tuple::vector(-0.45, 1.0, 0.0),
        ),
    );
    camera.threads = 8;
    camera.oversampling = 3;

    let image = camera.render(&mut world);
    let ppm = image.to_ppm();
    std::fs::write("examples/cover.ppm", ppm).unwrap();
}
