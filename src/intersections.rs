use crate::coordinates::{Coordinate, EPSILON};
use crate::rays::Ray;
use crate::shapes::{Shape, normal_at};
use crate::tuples::Tuple;
use std::ptr;

pub struct Intersection<'a> {
    pub t: Coordinate,
    pub object: &'a dyn Shape,
}

pub struct IntersectionComputations {
    pub point: Tuple,
    pub over_point: Tuple,
    pub under_point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub inside: bool,
    pub reflectv: Tuple,
    pub n1: Coordinate,
    pub n2: Coordinate,
}

impl<'a> Intersection<'a> {
    pub fn new(t: Coordinate, object: &'a dyn Shape) -> Intersection<'a> {
        Intersection { t, object }
    }

    pub fn prepare_computations(
        &'a self,
        ray: &Ray,
        xs: &'a Vec<Intersection<'a>>,
    ) -> IntersectionComputations {
        let point = ray.position(self.t);
        let eyev = -ray.direction;
        let normalv = normal_at(self.object, point);
        let inside = normalv.dot(eyev) < 0.0;
        let normalv = if inside { -normalv } else { normalv };
        let over_point = point + normalv * EPSILON;
        let under_point = point - normalv * EPSILON;
        let reflectv = ray.direction.reflect(normalv);
        let (n1, n2) = self.find_refraction_indices(xs);
        IntersectionComputations {
            point,
            over_point,
            under_point,
            eyev,
            normalv,
            inside,
            reflectv,
            n1,
            n2,
        }
    }

    fn find_refraction_indices(&self, xs: &Vec<Intersection>) -> (Coordinate, Coordinate) {
        let mut containers: Vec<&dyn Shape> = vec![];
        let mut n1 = 1.0;
        let mut n2 = 1.0;
        for x in xs {
            if x.t == self.t && containers.len() > 0 {
                n1 = containers.last().unwrap().material().refractive_index;
            }
            let i = containers.iter().position(|c| ptr::eq(*c, x.object));
            if let Some(i) = i {
                containers.remove(i);
            } else {
                containers.push(x.object);
            }
            if x.t == self.t {
                if containers.len() > 0 {
                    n2 = containers.last().unwrap().material().refractive_index;
                }
                break;
            }
        }
        (n1, n2)
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
    use crate::planes::Plane;
    use crate::spheres::Sphere;
    use crate::transformations::{scaling, translation};

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
        let comps = i.prepare_computations(&r, &vec![]);
        assert_eq!(comps.point, Tuple::point(0.0, 0.0, -1.0));
        assert_eq!(comps.eyev, Tuple::vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, Tuple::vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_outside() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = Sphere::default();
        let i = Intersection::new(4.0, &shape);
        let comps = i.prepare_computations(&r, &vec![]);
        assert_eq!(comps.inside, false);
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_inside() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = Sphere::default();
        let i = Intersection::new(1.0, &shape);
        let comps = i.prepare_computations(&r, &vec![]);
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
        let comps = i.prepare_computations(&r, &vec![]);
        assert!(comps.over_point.z() < -EPSILON / 2.0);
        assert!(comps.point.z() > comps.over_point.z());
    }

    #[test]
    fn precomputing_the_reflection_vector() {
        let shape = Plane::default();
        let position = Tuple::point(0.0, 1.0, 0.0);
        let eyev = Tuple::vector(0.0, -(2.0_f32).sqrt() / 2.0, (2.0_f32).sqrt() / 2.0);
        let r = Ray::new(position, eyev);
        let i = Intersection::new(1.0, &shape);
        let comps = i.prepare_computations(&r, &vec![]);
        assert_eq!(
            comps.reflectv,
            Tuple::vector(0.0, 2.0_f32.sqrt() / 2.0, 2.0_f32.sqrt() / 2.0)
        );
    }

    #[test]
    fn the_under_point_is_offset_below_the_surface() {
        let r = Ray::new(Tuple::point(0.0,0.0,-5.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = Sphere::glass(translation(0.0, 0.0, 1.0));
        let i = Intersection::new(5.0, &shape);
        let xs = vec![i];
        let comps = xs[0].prepare_computations(&r, &xs);
        assert!(comps.under_point.z() > EPSILON / 2.0);
        assert!(comps.point.z() < comps.under_point.z());
    }

    #[test]
    fn finding_n1_and_n2_at_various_intersections() {
        let mut a = Sphere::glass(scaling(2.0, 2.0, 2.0));
        a.material.refractive_index = 1.5;
        let mut b = Sphere::glass(translation(0.0, 0.0, -0.25));
        b.material.refractive_index = 2.0;
        let mut c = Sphere::glass(translation(0.0, 0.0, 0.25));
        c.material.refractive_index = 2.5;
        let r = Ray::new(Tuple::point(0.0, 0.0, -4.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = vec![
            Intersection::new(2.0, &a),
            Intersection::new(2.75, &b),
            Intersection::new(3.25, &c),
            Intersection::new(4.75, &b),
            Intersection::new(5.25, &c),
            Intersection::new(6.0, &a),
        ];
        assert_eq!(
            xs.iter()
                .map(|x| x.prepare_computations(&r, &xs))
                .map(|comps| (comps.n1, comps.n2))
                .collect::<Vec<_>>(),
            vec![
                (1.0, 1.5),
                (1.5, 2.0),
                (2.0, 2.5),
                (2.5, 2.5),
                (2.5, 1.5),
                (1.5, 1.0),
            ]
        )
    }
}
