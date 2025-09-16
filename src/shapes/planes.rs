use crate::bounds::Bounds;
use crate::floats::{EPSILON, Float};
use crate::floats::equals;
use crate::intersections::Intersection;
use crate::objects::Object;
use crate::rays::Ray;
use crate::tuples::Tuple;

pub struct Plane;

impl Plane {
    pub fn new() -> Plane {
        Plane
    }

    pub fn prepare_bounds(&mut self, bounds: &mut Bounds) {
        bounds.min = Tuple::point(-Float::INFINITY, -EPSILON, -Float::INFINITY);
        bounds.max = Tuple::point(Float::INFINITY, EPSILON, Float::INFINITY);
    }

    pub fn local_intersect<'a>(&'a self, ray: &Ray, object: &'a Object) -> Vec<Intersection<'a>> {
        if equals(ray.direction.y(), 0.0) {
            return vec![];
        }
        let t = -ray.origin.y() / ray.direction.y();
        vec![Intersection::new(t, object)]
    }

    pub fn local_normal_at(&self, _point: Tuple) -> Tuple {
        Tuple::vector(0.0, 1.0, 0.0)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn the_normal_of_a_plane_is_constant_everywhere() {
        let p = Plane::new();
        let n1 = p.local_normal_at(Tuple::point(0.0, 0.0, 0.0));
        let n2 = p.local_normal_at(Tuple::point(10.0, 0.0, -10.0));
        let n3 = p.local_normal_at(Tuple::point(-5.0, 0.0, 150.0));
        assert_eq!(n1, Tuple::vector(0.0, 1.0, 0.0));
        assert_eq!(n2, Tuple::vector(0.0, 1.0, 0.0));
        assert_eq!(n3, Tuple::vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn intersect_with_a_ray_parallel_to_the_plane() {
        let r = Ray::new(Tuple::point(0.0, 10.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let p = Object::new_plane();
        let xs = p.as_plane().local_intersect(&r, &p);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn intersect_with_a_ray_coplanar_to_the_plane() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let p = Object::new_plane();
        let xs = p.as_plane().local_intersect(&r, &p);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_above() {
        let r = Ray::new(Tuple::point(0.0, 1.0, 0.0), Tuple::vector(0.0, -1.0, 0.0));
        let p = Object::new_plane();
        let xs = p.as_plane().local_intersect(&r, &p);
        assert_eq!(xs.iter().map(|x| x.t).collect::<Vec<_>>(), vec![1.0]);
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_below() {
        let r = Ray::new(Tuple::point(0.0, -1.0, 0.0), Tuple::vector(0.0, 1.0, 0.0));
        let p = Object::new_plane();
        let xs = p.as_plane().local_intersect(&r, &p);
        assert_eq!(xs.iter().map(|x| x.t).collect::<Vec<_>>(), vec![1.0]);
    }
}
