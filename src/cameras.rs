use crate::canvas::Canvas;
use crate::floats::{Float, rand};
use crate::matrices::Matrix;
use crate::rays::Ray;
use crate::tuples::Tuple;
use crate::worlds::World;
use indicatif::ProgressBar;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    focal_length: Float,
    pub aperture: Float,
    hsize: usize,
    vsize: usize,
    half_width: Float,
    half_height: Float,
    pixel_size: Float,
    pub blur_oversampling: usize,
    pub oversampling: usize,
    pub render_depth: usize,
    pub threads: usize,
    transform_inv: Matrix<4>,
}

impl Camera {
    pub fn new(
        hsize: usize,
        vsize: usize,
        focal_length: Float,
        field_of_view: Float,
        transform: Matrix<4>,
    ) -> Camera {
        let half_view = focal_length * (field_of_view / 2.0).tan();
        let aspect = hsize as Float / vsize as Float;
        let (half_width, half_height) = if aspect >= 1.0 {
            (half_view, half_view / aspect)
        } else {
            (half_view * aspect, half_view)
        };
        Camera {
            focal_length,
            aperture: 0.0,
            hsize,
            vsize,
            half_width,
            half_height,
            pixel_size: half_width * 2.0 / hsize as Float,
            blur_oversampling: 1,
            oversampling: 2,
            render_depth: 5,
            threads: 1,
            transform_inv: transform.inverse(),
        }
    }

    fn rays_for_coordinates(&self, x_offset: Float, y_offset: Float) -> Vec<Ray> {
        let lens_x = self.half_width - x_offset;
        let lens_y = self.half_height - y_offset;
        let pixel = self.transform_inv * Tuple::point(lens_x, lens_y, -self.focal_length);
        let mut rays = vec![];
        let aperture = self.focal_length * self.aperture;
        for _ in 0..self.blur_oversampling {
            let lens_origin = Tuple::point(0.0, 0.0, 0.0)
                + if self.blur_oversampling > 1 {
                    Tuple::vector(rand(aperture), rand(aperture), 0.0)
                } else {
                    Tuple::vector(0.0, 0.0, 0.0)
                };
            let origin = self.transform_inv * lens_origin;
            let direction = (pixel - origin).normalize();
            rays.push(Ray::new(origin, direction));
        }
        rays
    }
    fn rays_for_pixel(&self, x: usize, y: usize) -> Vec<Ray> {
        let mut rays = Vec::new();
        let offset = 1.0 / self.oversampling as Float;
        let start_offset = offset / 2.0;
        for dx in 0..self.oversampling {
            for dy in 0..self.oversampling {
                let x_offset = (x as Float + start_offset + dx as Float * offset) * self.pixel_size;
                let y_offset = (y as Float + start_offset + dy as Float * offset) * self.pixel_size;
                rays.extend(self.rays_for_coordinates(x_offset, y_offset));
            }
        }
        rays
    }

    pub fn render(self, world: &mut World) -> Canvas {
        let mut world = world.clone();
        world.prepare();
        let world = Arc::new(world);
        let image = Arc::new(Mutex::new(Canvas::new(self.hsize, self.vsize)));
        let mut handles = Vec::new();
        let chunk_size = self.vsize / self.threads;
        let pb = Arc::new(Mutex::new(ProgressBar::new(self.vsize as u64)));
        for i in 0..self.threads {
            let pb = Arc::clone(&pb);
            let world = Arc::clone(&world);
            let image = Arc::clone(&image);
            let handle = thread::spawn(move || {
                for y in chunk_size * i..chunk_size * (i + 1) {
                    for x in 0..self.hsize {
                        let rays = self.rays_for_pixel(x, y);
                        let color = rays
                            .iter()
                            .map(|ray| world.color_at(&ray, self.render_depth))
                            .reduce(|a, b| a + b)
                            .unwrap()
                            * (1.0 / rays.len() as Float);
                        image.lock().unwrap().write_pixel(x, y, color);
                    }
                    pb.lock().unwrap().inc(1);
                }
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }
        pb.lock().unwrap().finish();
        match Arc::try_unwrap(image) {
            Ok(image) => image.into_inner().unwrap(),
            Err(_) => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::colors::Color;
    use crate::transformations::{rotation_y, translation, view_transform};
    use crate::worlds::tests::default_world;
    use std::f32::consts::PI;

    #[test]
    fn constructing_a_camera() {
        let hsize = 160;
        let vsize = 120;
        let field_of_view = PI / 2.0;
        let c = Camera::new(hsize, vsize, 1.0, field_of_view, Matrix::identity());
        assert_eq!(c.hsize, 160);
        assert_eq!(c.vsize, 120);
    }

    #[test]
    fn the_pixel_size_for_a_horizontal_canvas() {
        let c = Camera::new(200, 125, 1.0, PI / 2.0, Matrix::identity());
        assert_eq!(c.pixel_size, 0.01);
    }

    #[test]
    fn the_pixel_size_for_a_vertical_canvas() {
        let c = Camera::new(125, 200, 1.0, PI / 2.0, Matrix::identity());
        assert_eq!(c.pixel_size, 0.01);
    }

    #[test]
    fn constructing_a_ray_through_the_center_of_the_canvas() {
        let mut c = Camera::new(201, 101, 1.0, PI / 2.0, Matrix::identity());
        c.oversampling = 1;
        let rs = c.rays_for_pixel(100, 50);
        let r = &rs[0];
        assert_eq!(r.origin, Tuple::point(0.0, 0.0, 0.0));
        assert_eq!(r.direction, Tuple::vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn constructing_a_ray_through_a_corner_of_the_canvas() {
        let mut c = Camera::new(201, 101, 1.0, PI / 2.0, Matrix::identity());
        c.oversampling = 1;
        let rs = c.rays_for_pixel(0, 0);
        let r = &rs[0];
        assert_eq!(r.origin, Tuple::point(0.0, 0.0, 0.0));
        assert_eq!(r.direction, Tuple::vector(0.66519, 0.33259, -0.66851));
    }

    #[test]
    fn constructing_a_ray_when_the_camera_is_transformed() {
        let mut c = Camera::new(
            201,
            101,
            1.0,
            PI / 2.0,
            rotation_y(PI / 4.0) * translation(0.0, -2.0, 5.0),
        );
        c.oversampling = 1;
        let rs = c.rays_for_pixel(100, 50);
        let r = &rs[0];
        assert_eq!(r.origin, Tuple::point(0.0, 2.0, -5.0));
        assert_eq!(
            r.direction,
            Tuple::vector(2.0_f32.sqrt() / 2.0, 0.0, -2.0_f32.sqrt() / 2.0)
        );
    }

    #[test]
    fn rendering_a_world_with_a_camera() {
        let mut w = default_world();
        let from = Tuple::point(0.0, 0.0, -5.0);
        let to = Tuple::point(0.0, 0.0, 0.0);
        let up = Tuple::vector(0.0, 1.0, 0.0);
        let mut c = Camera::new(11, 11, 1.0, PI / 2.0, view_transform(from, to, up));
        c.oversampling = 1;
        c.render_depth = 1;
        let image = c.render(&mut w);
        assert_eq!(image.pixel_at(5, 5), Color::new(0.38066, 0.47583, 0.2855));
    }
}
