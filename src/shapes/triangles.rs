use crate::bounds::Bounds;
use crate::floats::EPSILON;
use crate::intersections::Intersection;
use crate::objects::Object;
use crate::rays::Ray;
use crate::tuples::Tuple;

pub struct Triangle {
    pub p1: Tuple,
    pub p2: Tuple,
    pub p3: Tuple,
    pub e1: Tuple,
    pub e2: Tuple,
    pub normal: Tuple,
}

impl Triangle {
    pub fn new(p1: Tuple, p2: Tuple, p3: Tuple) -> Triangle {
        let e1 = p2 - p1;
        let e2 = p3 - p1;
        let normal = e2.cross(e1).normalize();
        Triangle {
            p1,
            p2,
            p3,
            e1,
            e2,
            normal,
        }
    }

    pub fn prepare_bounds(&self, bounds: &mut Bounds) {
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
        if u < 0.0 || u > 1.0 {
            return vec![];
        }
        let origin_cross_e1 = p1_to_origin.cross(self.e1);
        let v = f * ray.direction.dot(origin_cross_e1);
        if v < 0.0 || u + v > 1.0 {
            return vec![];
        }
        let t = f * self.e2.dot(origin_cross_e1);
        vec![Intersection::new(t, object)]
    }

    pub fn local_normal_at(&self, _local_point: Tuple) -> Tuple {
        self.normal
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructing_a_triangle() {
        let p1 = Tuple::point(0.0, 1.0, 0.0);
        let p2 = Tuple::point(-1.0, 0.0, 0.0);
        let p3 = Tuple::point(1.0, 0.0, 0.0);
        let t = Triangle::new(p1, p2, p3);
        assert_eq!(t.p1, p1);
        assert_eq!(t.p2, p2);
        assert_eq!(t.p3, p3);
        assert_eq!(t.e1, Tuple::vector(-1.0, -1.0, 0.0));
        assert_eq!(t.e2, Tuple::vector(1.0, -1.0, 0.0));
        assert_eq!(t.normal, Tuple::vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn finding_the_normal_on_a_triangle() {
        let p1 = Tuple::point(0.0, 1.0, 0.0);
        let p2 = Tuple::point(-1.0, 0.0, 0.0);
        let p3 = Tuple::point(1.0, 0.0, 0.0);
        let t = Triangle::new(p1, p2, p3);
        let n1 = t.local_normal_at(Tuple::point(0.0, 0.5, 0.0));
        let n2 = t.local_normal_at(Tuple::point(-0.5, 0.75, 0.0));
        let n3 = t.local_normal_at(Tuple::point(0.5, 0.25, 0.0));
        assert_eq!(n1, t.normal);
        assert_eq!(n2, t.normal);
        assert_eq!(n3, t.normal);
    }

    #[test]
    fn intersecting_a_ray_parallel_to_the_triangle() {
        let t = Object::new_triangle(
            Tuple::point(0.0, 1.0, 0.0),
            Tuple::point(-1.0, 0.0, 0.0),
            Tuple::point(1.0, 0.0, 0.0),
        );
        let r = Ray::new(Tuple::point(0.0, -1.0, -2.0), Tuple::vector(0.0, 1.0, 0.0));
        let xs = t.as_triangle().local_intersect(&r, &t);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn ray_misses_p1_p3_edge() {
        let t = Object::new_triangle(
            Tuple::point(0.0, 1.0, 0.0),
            Tuple::point(-1.0, 0.0, 0.0),
            Tuple::point(1.0, 0.0, 0.0),
        );
        let r = Ray::new(Tuple::point(1.0, 1.0, -2.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = t.as_triangle().local_intersect(&r, &t);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn ray_misses_p1_p2_edge() {
        let t = Object::new_triangle(
            Tuple::point(0.0, 1.0, 0.0),
            Tuple::point(-1.0, 0.0, 0.0),
            Tuple::point(1.0, 0.0, 0.0),
        );
        let r = Ray::new(Tuple::point(-1.0, 1.0, -2.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = t.as_triangle().local_intersect(&r, &t);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn ray_misses_p2_p3_edge() {
        let t = Object::new_triangle(
            Tuple::point(0.0, 1.0, 0.0),
            Tuple::point(-1.0, 0.0, 0.0),
            Tuple::point(1.0, 0.0, 0.0),
        );
        let r = Ray::new(Tuple::point(0.0, -1.0, -2.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = t.as_triangle().local_intersect(&r, &t);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn ray_hits_a_triangle() {
        let t = Object::new_triangle(
            Tuple::point(0.0, 1.0, 0.0),
            Tuple::point(-1.0, 0.0, 0.0),
            Tuple::point(1.0, 0.0, 0.0),
        );
        let r = Ray::new(Tuple::point(0.0, 0.5, -2.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = t.as_triangle().local_intersect(&r, &t);
        assert_eq!(xs.iter().map(|x| x.t).collect::<Vec<_>>(), vec![2.0]);
    }
}
