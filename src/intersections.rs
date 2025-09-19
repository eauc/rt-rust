use crate::floats::{Float, EPSILON};
use crate::objects::Object;
use crate::rays::Ray;
use crate::tuples::Tuple;
use std::ptr;

pub struct Intersection<'a> {
    pub t: Float,
    pub u: Float,
    pub v: Float,
    pub object: &'a Object,
}

pub struct IntersectionComputations {
    pub point: Tuple,
    pub over_point: Tuple,
    pub under_point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub inside: bool,
    pub reflectv: Tuple,
    pub n1: Float,
    pub n2: Float,
}

impl<'a> Intersection<'a> {
    pub fn new(t: Float, object: &'a Object) -> Intersection<'a> {
        Intersection {
            t,
            u: 0.0,
            v: 0.0,
            object,
        }
    }

    pub fn new_with_uv(t: Float, object: &'a Object, u: Float, v: Float) -> Intersection<'a> {
        Intersection { t, u, v, object }
    }

    pub fn prepare_computations(
        &'a self,
        ray: &Ray,
        xs: &'a Vec<Intersection<'a>>,
    ) -> IntersectionComputations {
        let point = ray.position(self.t);
        let eyev = -ray.direction;
        let normalv = self.object.normal_at(point, self);
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

    fn find_refraction_indices(&self, xs: &Vec<Intersection>) -> (Float, Float) {
        let mut containers: Vec<&Object> = Vec::with_capacity(xs.len());
        let mut n1 = 1.0;
        let mut n2 = 1.0;
        for x in xs {
            if x.t == self.t && !containers.is_empty() {
                n1 = containers.last().unwrap().material.refractive_index;
            }
            let i = containers.iter().position(|c| ptr::eq(*c, x.object));
            if let Some(i) = i {
                containers.remove(i);
            } else {
                containers.push(x.object);
            }
            if x.t == self.t {
                if !containers.is_empty() {
                    n2 = containers.last().unwrap().material.refractive_index;
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

pub fn schlick(comps: &IntersectionComputations) -> Float {
    let mut cos = comps.eyev.dot(comps.normalv);
    if comps.n1 > comps.n2 {
        let n = comps.n1 / comps.n2;
        let sin2_t = n.powi(2) * (1.0 - cos.powi(2));
        if sin2_t > 1.0 {
            return 1.0;
        }
        let cos_t = (1.0 - sin2_t).sqrt();
        cos = cos_t;
    }
    let r0 = ((comps.n1 - comps.n2) / (comps.n1 + comps.n2)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cos).powi(5)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::floats::{equals, SQRT_2};
    use crate::matrices::Matrix;
    use crate::transformations::{scaling, translation};

    #[test]
    fn an_intersection_encapsulates_t_and_object() {
        let s = Object::new_sphere();
        let i = Intersection::new(3.5, &s);
        assert_eq!(i.t, 3.5);
    }

    #[test]
    fn the_hit_when_all_intersections_have_positive_t() {
        let s = Object::new_sphere();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs = vec![i2, i1];
        let i = hit(&xs).unwrap();
        assert_eq!(i.t, 1.0);
    }

    #[test]
    fn the_hit_when_some_intersections_have_negative_t() {
        let s = Object::new_sphere();
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(1.0, &s);
        let xs = vec![i2, i1];
        let i = hit(&xs).unwrap();
        assert_eq!(i.t, 1.0);
    }

    #[test]
    fn the_hit_when_all_intersections_have_negative_t() {
        let s = Object::new_sphere();
        let i1 = Intersection::new(-2.0, &s);
        let i2 = Intersection::new(-1.0, &s);
        let xs = vec![i2, i1];
        let i = hit(&xs);
        assert!(i.is_none());
    }

    #[test]
    fn the_hit_is_always_the_lowest_nonnegative_intersection() {
        let s = Object::new_sphere();
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
        let object = Object::new_sphere();
        let i = Intersection::new(4.0, &object);
        let comps = i.prepare_computations(&r, &vec![]);
        assert_eq!(comps.point, Tuple::point(0.0, 0.0, -1.0));
        assert_eq!(comps.eyev, Tuple::vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, Tuple::vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_outside() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let object = Object::new_sphere();
        let i = Intersection::new(4.0, &object);
        let comps = i.prepare_computations(&r, &vec![]);
        assert_eq!(comps.inside, false);
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_inside() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let object = Object::new_sphere();
        let i = Intersection::new(1.0, &object);
        let comps = i.prepare_computations(&r, &vec![]);
        assert_eq!(comps.point, Tuple::point(0.0, 0.0, 1.0));
        assert_eq!(comps.eyev, Tuple::vector(0.0, 0.0, -1.0));
        assert_eq!(comps.inside, true);
        assert_eq!(comps.normalv, Tuple::vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn the_hit_should_offset_the_point() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let object = Object::new_sphere().with_transform(translation(0.0, 0.0, 1.0));
        let i = Intersection::new(5.0, &object);
        let comps = i.prepare_computations(&r, &vec![]);
        assert!(comps.over_point.z() < -EPSILON / 2.0);
        assert!(comps.point.z() > comps.over_point.z());
    }

    #[test]
    fn precomputing_the_reflection_vector() {
        let object = Object::new_plane();
        let position = Tuple::point(0.0, 1.0, 0.0);
        let eyev = Tuple::vector(0.0, -SQRT_2 / 2.0, SQRT_2 / 2.0);
        let r = Ray::new(position, eyev);
        let i = Intersection::new(1.0, &object);
        let comps = i.prepare_computations(&r, &vec![]);
        assert_eq!(
            comps.reflectv,
            Tuple::vector(0.0, SQRT_2 / 2.0, SQRT_2 / 2.0)
        );
    }

    #[test]
    fn the_under_point_is_offset_below_the_surface() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let object = Object::new_sphere()
            .made_of_glass()
            .with_transform(translation(0.0, 0.0, 1.0));
        let i = Intersection::new(5.0, &object);
        let xs = vec![i];
        let comps = xs[0].prepare_computations(&r, &xs);
        assert!(comps.under_point.z() > EPSILON / 2.0);
        assert!(comps.point.z() < comps.under_point.z());
    }

    #[test]
    fn finding_n1_and_n2_at_various_intersections() {
        let mut a = Object::new_sphere()
            .made_of_glass()
            .with_transform(scaling(2.0, 2.0, 2.0));
        a.material.refractive_index = 1.5;
        let mut b = Object::new_sphere()
            .made_of_glass()
            .with_transform(translation(0.0, 0.0, -0.25));
        b.material.refractive_index = 2.0;
        let mut c = Object::new_sphere()
            .made_of_glass()
            .with_transform(translation(0.0, 0.0, 0.25));
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

    #[test]
    fn the_schlick_approximation_under_total_internal_reflection() {
        let object = Object::new_sphere()
            .made_of_glass()
            .with_transform(Matrix::identity());
        let r = Ray::new(
            Tuple::point(0.0, 0.0, SQRT_2 / 2.0),
            Tuple::vector(0.0, 1.0, 0.0),
        );
        let xs = vec![
            Intersection::new(-SQRT_2 / 2.0, &object),
            Intersection::new(SQRT_2 / 2.0, &object),
        ];
        let comps = xs[1].prepare_computations(&r, &xs);
        let reflectance = schlick(&comps);
        assert_eq!(reflectance, 1.0);
    }

    #[test]
    fn the_schlick_approximation_with_a_perpendicular_viewing_angle() {
        let object = Object::new_sphere()
            .made_of_glass()
            .with_transform(Matrix::identity());
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 1.0, 0.0));
        let xs = vec![
            Intersection::new(-1.0, &object),
            Intersection::new(1.0, &object),
        ];
        let comps = xs[1].prepare_computations(&r, &xs);
        let reflectance = schlick(&comps);
        assert!(equals(reflectance, 0.04));
    }

    #[test]
    fn the_schlick_approximation_with_small_angle_and_n2_gt_n1() {
        let object = Object::new_sphere()
            .made_of_glass()
            .with_transform(Matrix::identity());
        let r = Ray::new(Tuple::point(0.0, 0.99, -2.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = vec![Intersection::new(1.8589, &object)];
        let comps = xs[0].prepare_computations(&r, &xs);
        let reflectance = schlick(&comps);
        assert!(equals(reflectance, 0.48873067));
    }

    #[test]
    fn an_intersection_can_encapsulate_u_and_v() {
        let s = Object::new_triangle(
            Tuple::point(0.0, 1.0, 0.0),
            Tuple::point(-1.0, 0.0, 0.0),
            Tuple::point(1.0, 0.0, 0.0),
        );
        let i = Intersection::new_with_uv(3.5, &s, 0.2, 0.4);
        assert_eq!(i.u, 0.2);
        assert_eq!(i.v, 0.4);
    }
}
