use crate::bounds::Bounds;
use crate::floats::EPSILON;
use crate::intersections::Intersection;
use crate::objects::Object;
use crate::rays::Ray;
use crate::tuples::Tuple;

#[derive(Debug, Clone, PartialEq)]
pub struct SmoothTriangle {
    pub p1: Tuple,
    pub p2: Tuple,
    pub p3: Tuple,
    e1: Tuple,
    e2: Tuple,
    pub n1: Tuple,
    pub n2: Tuple,
    pub n3: Tuple,
}

impl SmoothTriangle {
    pub fn new(p1: Tuple, p2: Tuple, p3: Tuple, n1: Tuple, n2: Tuple, n3: Tuple) -> SmoothTriangle {
        let e1 = p2 - p1;
        let e2 = p3 - p1;
        SmoothTriangle {
            p1,
            p2,
            p3,
            e1,
            e2,
            n1,
            n2,
            n3,
        }
    }

    pub fn prepare_bounds(&mut self, bounds: &mut Bounds) {
        bounds.min = Tuple::point(
            self.p1.x().min(self.p2.x().min(self.p3.x())),
            self.p1.y().min(self.p2.y().min(self.p3.y())),
            self.p1.z().min(self.p2.z().min(self.p3.z())),
        );
        bounds.max = Tuple::point(
            self.p1.x().max(self.p2.x().max(self.p3.x())),
            self.p1.y().max(self.p2.y().max(self.p3.y())),
            self.p1.z().max(self.p2.z().max(self.p3.z())),
        );
    }

    pub fn local_intersect<'a>(&'a self, ray: &Ray, object: &'a Object) -> Vec<Intersection<'a>> {
        let dir_cross_e2 = ray.direction.cross(self.e2);
        let det = self.e1.dot(dir_cross_e2);
        if det.abs() < EPSILON {
            return vec![];
        }
        let f = 1.0 / det;
        let p1_to_origin = ray.origin - self.p1;
        let u = f * p1_to_origin.dot(dir_cross_e2);
        if !(0.0..=1.0).contains(&u) {
            return vec![];
        }
        let origin_cross_e1 = p1_to_origin.cross(self.e1);
        let v = f * ray.direction.dot(origin_cross_e1);
        if v < 0.0 || u + v > 1.0 {
            return vec![];
        }
        let t = f * self.e2.dot(origin_cross_e1);
        vec![Intersection::new_with_uv(t, object, u, v)]
    }

    pub fn local_normal_at(&self, _point: Tuple, hit: &Intersection) -> Tuple {
        self.n2 * hit.u + self.n3 * hit.v + self.n1 * (1.0 - hit.u - hit.v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn background() -> Object {
        let p1 = Tuple::point(0.0, 1.0, 0.0);
        let p2 = Tuple::point(-1.0, 0.0, 0.0);
        let p3 = Tuple::point(1.0, 0.0, 0.0);
        let n1 = Tuple::vector(0.0, 1.0, 0.0);
        let n2 = Tuple::vector(-1.0, 0.0, 0.0);
        let n3 = Tuple::vector(1.0, 0.0, 0.0);
        Object::new_smooth_triangle(p1, p2, p3, n1, n2, n3)
    }

    #[test]
    fn constructing_a_smooth_triangle() {
        let tri = background();
        assert_eq!(tri.as_smooth_triangle().p1, Tuple::point(0.0, 1.0, 0.0));
        assert_eq!(tri.as_smooth_triangle().p2, Tuple::point(-1.0, 0.0, 0.0));
        assert_eq!(tri.as_smooth_triangle().p3, Tuple::point(1.0, 0.0, 0.0));
        assert_eq!(tri.as_smooth_triangle().n1, Tuple::vector(0.0, 1.0, 0.0));
        assert_eq!(tri.as_smooth_triangle().n2, Tuple::vector(-1.0, 0.0, 0.0));
        assert_eq!(tri.as_smooth_triangle().n3, Tuple::vector(1.0, 0.0, 0.0));
    }

    #[test]
    fn an_intersection_with_a_smooth_triangle_stores_u_v() {
        let tri = background();
        let r = Ray::new(Tuple::point(-0.2, 0.3, -2.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = tri.as_smooth_triangle().local_intersect(&r, &tri);
        assert_eq!(xs[0].u, 0.45);
        assert_eq!(xs[0].v, 0.25);
    }

    #[test]
    fn a_smooth_triangle_interpolates_the_normal() {
        let tri = background();
        let r = Ray::new(Tuple::point(-0.2, 0.3, -2.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = tri.as_smooth_triangle().local_intersect(&r, &tri);
        let i = &xs[0];
        let n = tri.normal_at(Tuple::point(0.0, 0.0, 0.0), i);
        assert_eq!(n, Tuple::vector(-0.5547, 0.83205, 0.0));
    }

    #[test]
    fn preparing_the_normal_on_a_smooth_triangle() {
        let tri = background();
        let r = Ray::new(Tuple::point(-0.2, 0.3, -2.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = tri.as_smooth_triangle().local_intersect(&r, &tri);
        let i = &xs[0];
        let comps = i.prepare_computations(&r, &xs);
        assert_eq!(comps.normalv, Tuple::vector(-0.5547, 0.83205, 0.0));
    }
}
