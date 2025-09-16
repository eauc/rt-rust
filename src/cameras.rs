use crate::canvas::Canvas;
use crate::floats::Float;
use crate::matrices::Matrix;
use crate::rays::Ray;
use crate::tuples::Tuple;
use crate::worlds::World;

pub struct Camera {
    hsize: usize,
    vsize: usize,
    half_width: Float,
    half_height: Float,
    pixel_size: Float,
    transform_inv: Matrix<4>,
}

impl Camera {
    pub fn new(
        hsize: usize,
        vsize: usize,
        field_of_view: Float,
        transform: Matrix<4>,
    ) -> Camera {
        let half_view = (field_of_view / 2.0).tan();
        let aspect = hsize as Float / vsize as Float;
        let (half_width, half_height) = if aspect >= 1.0 {
            (half_view, half_view / aspect)
        } else {
            (half_view * aspect, half_view)
        };
        Camera {
            hsize,
            vsize,
            half_width,
            half_height,
            pixel_size: half_width * 2.0 / hsize as Float,
            transform_inv: transform.inverse(),
        }
    }

    fn ray_for_pixel(&self, x: usize, y: usize) -> Ray {
        let x_offset = (x as Float + 0.5) * self.pixel_size;
        let y_offset = (y as Float + 0.5) * self.pixel_size;
        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;
        let pixel = self.transform_inv * Tuple::point(world_x, world_y, -1.0);
        let origin = self.transform_inv * Tuple::point(0.0, 0.0, 0.0);
        let direction = (pixel - origin).normalize();
        return Ray::new(origin, direction);
    }

    pub fn render(&self, world: &mut World, depth: u32) -> Canvas {
        world.prepare();
        let mut image = Canvas::new(self.hsize, self.vsize);
        for y in 0..self.vsize {
            for x in 0..self.hsize {
                let ray = self.ray_for_pixel(x, y);
                let color = world.color_at(&ray, depth);
                image.write_pixel(x, y, color);
            }
        }
        image
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
        let c = Camera::new(hsize, vsize, field_of_view, Matrix::identity());
        assert_eq!(c.hsize, 160);
        assert_eq!(c.vsize, 120);
    }

    #[test]
    fn the_pixel_size_for_a_horizontal_canvas() {
        let c = Camera::new(200, 125, PI / 2.0, Matrix::identity());
        assert_eq!(c.pixel_size, 0.01);
    }

    #[test]
    fn the_pixel_size_for_a_vertical_canvas() {
        let c = Camera::new(125, 200, PI / 2.0, Matrix::identity());
        assert_eq!(c.pixel_size, 0.01);
    }

    #[test]
    fn constructing_a_ray_through_the_center_of_the_canvas() {
        let c = Camera::new(201, 101, PI / 2.0, Matrix::identity());
        let r = c.ray_for_pixel(100, 50);
        assert_eq!(r.origin, Tuple::point(0.0, 0.0, 0.0));
        assert_eq!(r.direction, Tuple::vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn constructing_a_ray_through_a_corner_of_the_canvas() {
        let c = Camera::new(201, 101, PI / 2.0, Matrix::identity());
        let r = c.ray_for_pixel(0, 0);
        assert_eq!(r.origin, Tuple::point(0.0, 0.0, 0.0));
        assert_eq!(r.direction, Tuple::vector(0.66519, 0.33259, -0.66851));
    }

    #[test]
    fn constructing_a_ray_when_the_camera_is_transformed() {
        let c = Camera::new(
            201,
            101,
            PI / 2.0,
            rotation_y(PI / 4.0) * translation(0.0, -2.0, 5.0),
        );
        let r = c.ray_for_pixel(100, 50);
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
        let c = Camera::new(11, 11, PI / 2.0, view_transform(from, to, up));
        let image = c.render(&mut w, 1);
        assert_eq!(image.pixel_at(5, 5), Color::new(0.38066, 0.47583, 0.2855));
    }
}
