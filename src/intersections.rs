use crate::coordinates::{Coordinate, EPSILON};
use crate::rays::Ray;
use crate::shapes::{Shape, normal_at};
use crate::tuples::Tuple;

pub struct Intersection<'a> {
    pub t: Coordinate,
    pub object: &'a dyn Shape,
}

pub struct IntersectionComputations {
    pub point: Tuple,
    pub over_point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub inside: bool,
}

impl<'a> Intersection<'a> {
    pub fn new(t: Coordinate, object: &'a dyn Shape) -> Intersection<'a> {
        Intersection { t, object }
    }

    pub fn prepare_computations(&'a self, ray: &Ray) -> IntersectionComputations {
        let point = ray.position(self.t);
        let eyev = -ray.direction;
        let normalv = normal_at(self.object, point);
        let inside = normalv.dot(eyev) < 0.0;
        let normalv = if inside { -normalv } else { normalv };
        let over_point = point + normalv * EPSILON;
        IntersectionComputations {
            point,
            over_point,
            eyev,
            normalv,
            inside,
        }
    }
}

pub fn hit<'a>(xs: &'a Vec<Intersection<'a>>) -> Option<&'a Intersection<'a>> {
    xs.iter()
        .filter(|i| i.t >= 0.0)
        .min_by(|i1, i2| i1.t.total_cmp(&i2.t))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spheres::Sphere;
    use crate::transformations::translation;

    #[test]
    fn an_intersection_encapsulates_t_and_object() {
        let s = Sphere::default();
        let i = Intersection::new(3.5, &s);
        assert_eq!(i.t, 3.5);
    }

    #[test]
    fn the_hit_when_all_intersections_have_positive_t() {
        let s = Sphere::default();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs = vec![i2, i1];
        let i = hit(&xs).unwrap();
        assert_eq!(i.t, 1.0);
    }

    #[test]
    fn the_hit_when_some_intersections_have_negative_t() {
        let s = Sphere::default();
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(1.0, &s);
        let xs = vec![i2, i1];
        let i = hit(&xs).unwrap();
        assert_eq!(i.t, 1.0);
    }

    #[test]
    fn the_hit_when_all_intersections_have_negative_t() {
        let s = Sphere::default();
        let i1 = Intersection::new(-2.0, &s);
        let i2 = Intersection::new(-1.0, &s);
        let xs = vec![i2, i1];
        let i = hit(&xs);
        assert!(i.is_none());
    }

    #[test]
    fn the_hit_is_always_the_lowest_nonnegative_intersection() {
        let s = Sphere::default();
        let i1 = Intersection::new(5.0, &s);
        let i2 = Intersection::new(7.0, &s);
        let i3 = Intersection::new(-3.0, &s);
        let i4 = Intersection::new(2.0, &s);
        let xs = vec![i1, i2, i3, i4];
        let i = hit(&xs).unwrap();
        assert_eq!(i.t, 2.0);
    }

    #[test]
    fn precomputing_the_state_of_an_intersection() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = Sphere::default();
        let i = Intersection::new(4.0, &shape);
        let comps = i.prepare_computations(&r);
        assert_eq!(comps.point, Tuple::point(0.0, 0.0, -1.0));
        assert_eq!(comps.eyev, Tuple::vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, Tuple::vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_outside() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = Sphere::default();
        let i = Intersection::new(4.0, &shape);
        let comps = i.prepare_computations(&r);
        assert_eq!(comps.inside, false);
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_inside() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = Sphere::default();
        let i = Intersection::new(1.0, &shape);
        let comps = i.prepare_computations(&r);
        assert_eq!(comps.point, Tuple::point(0.0, 0.0, 1.0));
        assert_eq!(comps.eyev, Tuple::vector(0.0, 0.0, -1.0));
        assert_eq!(comps.inside, true);
        assert_eq!(comps.normalv, Tuple::vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn the_hit_should_offset_the_point() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = Sphere::new(translation(0.0, 0.0, 1.0));
        let i = Intersection::new(5.0, &shape);
        let comps = i.prepare_computations(&r);
        assert!(comps.over_point.z() < -EPSILON / 2.0);
        assert!(comps.point.z() > comps.over_point.z());
    }
}
