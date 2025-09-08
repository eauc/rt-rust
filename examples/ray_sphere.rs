use rt_rust::canvas::Canvas;
use rt_rust::colors::Color;
use rt_rust::rays::Ray;
use rt_rust::spheres::Sphere;
use rt_rust::tuples::Tuple;
use rt_rust::intersections;

fn main() {
    let ray_origin = Tuple::point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let canvas_pixel = 400;
    let pixel_size = wall_size / canvas_pixel as f32;
    let half = wall_size / 2.0;
    let mut canvas = Canvas::new(canvas_pixel, canvas_pixel);
    let red = Color::new(1.0, 0.0, 0.0);
    let sphere = Sphere::default();
    for y in 0..canvas_pixel {
        let world_y = half - pixel_size * y as f32;
        for x in 0..canvas_pixel {
            let world_x = -half + pixel_size * x as f32;
            let position = Tuple::point(world_x, world_y, wall_z);
            let ray = Ray::new(ray_origin, (position - ray_origin).normalize());
            let xs = sphere.intersect(&ray);
            if let Some(_) = intersections::hit(&xs) {
                canvas.write_pixel(x, y, red);
            }
        }
    }
    let ppm = canvas.to_ppm();
    std::fs::write("examples/ray_sphere.ppm", ppm).unwrap();
}
