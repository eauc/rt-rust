use crate::floats::{EPSILON, Float};
use crate::matrices::Matrix;
use crate::rays::Ray;
use crate::tuples::Tuple;

#[derive(Debug, Clone, PartialEq)]
pub struct Bounds {
    pub min: Tuple,
    pub max: Tuple,
}

impl Default for Bounds {
    fn default() -> Self {
        Bounds {
            min: Tuple::point(-1.0, -1.0, -1.0),
            max: Tuple::point(1.0, 1.0, 1.0),
        }
    }
}

impl Bounds {
    pub fn intersect(&self, ray: &Ray) -> bool {
        let (xtmin, xtmax) = check_axis(
            ray.origin.x(),
            ray.direction.x(),
            self.min.x(),
            self.max.x(),
        );
        let (ytmin, ytmax) = check_axis(
            ray.origin.y(),
            ray.direction.y(),
            self.min.y(),
            self.max.y(),
        );
        let (ztmin, ztmax) = check_axis(
            ray.origin.z(),
            ray.direction.z(),
            self.min.z(),
            self.max.z(),
        );
        let tmin = xtmin.max(ytmin).max(ztmin);
        let tmax = xtmax.min(ytmax).min(ztmax);
        tmin <= tmax
    }

    pub fn transform(&self, transform: &Matrix<4>) -> Bounds {
        let min = self.min;
        let max = self.max;
        let (min_x, min_y, min_z) = (min.x(), min.y(), min.z());
        let (max_x, max_y, max_z) = (max.x(), max.y(), max.z());
        let corners = (
            *transform * Tuple::point(min_x, min_y, min_z),
            *transform * Tuple::point(min_x, min_y, max_z),
            *transform * Tuple::point(min_x, max_y, min_z),
            *transform * Tuple::point(min_x, max_y, max_z),
            *transform * Tuple::point(max_x, min_y, min_z),
            *transform * Tuple::point(max_x, min_y, max_z),
            *transform * Tuple::point(max_x, max_y, min_z),
            *transform * Tuple::point(max_x, max_y, max_z),
        );
        let min_x = corners
            .0
            .x()
            .min(corners.1.x())
            .min(corners.2.x())
            .min(corners.3.x())
            .min(corners.4.x())
            .min(corners.5.x())
            .min(corners.6.x())
            .min(corners.7.x());
        let min_y = corners
            .0
            .y()
            .min(corners.1.y())
            .min(corners.2.y())
            .min(corners.3.y())
            .min(corners.4.y())
            .min(corners.5.y())
            .min(corners.6.y())
            .min(corners.7.y());
        let min_z = corners
            .0
            .z()
            .min(corners.1.z())
            .min(corners.2.z())
            .min(corners.3.z())
            .min(corners.4.z())
            .min(corners.5.z())
            .min(corners.6.z())
            .min(corners.7.z());
        let max_x = corners
            .0
            .x()
            .max(corners.1.x())
            .max(corners.2.x())
            .max(corners.3.x())
            .max(corners.4.x())
            .max(corners.5.x())
            .max(corners.6.x())
            .max(corners.7.x());
        let max_y = corners
            .0
            .y()
            .max(corners.1.y())
            .max(corners.2.y())
            .max(corners.3.y())
            .max(corners.4.y())
            .max(corners.5.y())
            .max(corners.6.y())
            .max(corners.7.y());
        let max_z = corners
            .0
            .z()
            .max(corners.1.z())
            .max(corners.2.z())
            .max(corners.3.z())
            .max(corners.4.z())
            .max(corners.5.z())
            .max(corners.6.z())
            .max(corners.7.z());
        Bounds {
            min: Tuple::point(min_x, min_y, min_z),
            max: Tuple::point(max_x, max_y, max_z),
        }
    }

    pub fn merge(&mut self, other: &Bounds) {
        self.min = Tuple::point(
            self.min.x().min(other.min.x()),
            self.min.y().min(other.min.y()),
            self.min.z().min(other.min.z()),
        );
        self.max = Tuple::point(
            self.max.x().max(other.max.x()),
            self.max.y().max(other.max.y()),
            self.max.z().max(other.max.z()),
        );
    }
}

fn check_axis(origin: Float, direction: Float, min: Float, max: Float) -> (Float, Float) {
    let tmin_numerator = min - origin - EPSILON;
    let tmax_numerator = max - origin + EPSILON;

    let (tmin, tmax) = if direction.abs() >= EPSILON {
        (tmin_numerator / direction, tmax_numerator / direction)
    } else {
        (
            tmin_numerator * Float::INFINITY,
            tmax_numerator * Float::INFINITY,
        )
    };
    if tmin > tmax {
        (tmax, tmin)
    } else {
        (tmin, tmax)
    }
}
