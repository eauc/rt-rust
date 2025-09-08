use crate::colors::Color;
use crate::intersections::{self, Intersection, IntersectionComputations};
use crate::lights::PointLight;
use crate::rays::Ray;
use crate::shapes::{Shape, intersect};
use crate::tuples::Tuple;

pub struct World<'a> {
    pub light: PointLight,
    pub objects: Vec<&'a dyn Shape>,
}

impl<'a> World<'a> {
    pub fn new(light: PointLight, objects: Vec<&'a dyn Shape>) -> World<'a> {
        World { light, objects }
    }

    fn intersect(&'a self, ray: &Ray) -> Vec<Intersection<'a>> {
        let mut intersections = self
            .objects
            .iter()
            .flat_map(|s| intersect(*s, ray))
            .collect::<Vec<Intersection>>();
        intersections.sort_by(|i1, i2| i1.t.partial_cmp(&i2.t).unwrap());
        intersections
    }

    fn is_shadowed(&self, point: Tuple) -> bool {
        let v = self.light.position - point;
        let distance = v.magnitude();
        let direction = v.normalize();
        let r = Ray::new(point, direction);

        let xs = self.intersect(&r);
        if let Some(hit) = intersections::hit(&xs)
            && hit.t < distance
        {
            true
        } else {
            false
        }
    }

    fn shade_hit(&self, hit: &Intersection, comps: &IntersectionComputations) -> Color {
        let is_shadowed = self.is_shadowed(comps.over_point);
        hit.object.material().lighting(
            &self.light,
            comps.over_point,
            comps.eyev,
            comps.normalv,
            is_shadowed,
        )
    }

    pub fn color_at(&self, ray: &Ray) -> Color {
        let xs = self.intersect(ray);
        if let Some(hit) = intersections::hit(&xs) {
            let comps = hit.prepare_computations(&ray);
            self.shade_hit(&hit, &comps)
        } else {
            crate::colors::BLACK
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::spheres::Sphere;
    use crate::transformations::{scaling, translation};

    pub fn default_world_objects() -> (Sphere, Sphere) {
        let mut s1 = Sphere::default();
        s1.material.color = Color::new(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;
        let s2 = Sphere::new(scaling(0.5, 0.5, 0.5));

        (s1, s2)
    }
    pub fn default_world<'a>(s1: &'a Sphere, s2: &'a Sphere) -> World<'a> {
        World {
            light: PointLight::new(Tuple::point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0)),
            objects: vec![s1, s2],
        }
    }

    #[test]
    fn intersect_a_world_with_a_ray() {
        let (s1, s2) = default_world_objects();
        let w = default_world(&s1, &s2);
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = w.intersect(&r);
        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 4.5);
        assert_eq!(xs[2].t, 5.5);
        assert_eq!(xs[3].t, 6.0);
    }

    #[test]
    fn shading_an_intersection() {
        let (s1, s2) = default_world_objects();
        let w = default_world(&s1, &s2);
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = w.objects[0];
        let i = Intersection::new(4.0, shape);
        let comps = i.prepare_computations(&r);
        let c = w.shade_hit(&i, &comps);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let (s1, s2) = default_world_objects();
        let mut w = default_world(&s1, &s2);
        w.light = PointLight::new(Tuple::point(0.0, 0.25, 0.0), Color::new(1.0, 1.0, 1.0));
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = w.objects[1];
        let i = Intersection::new(0.5, shape);
        let comps = i.prepare_computations(&r);
        let c = w.shade_hit(&i, &comps);
        assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn the_color_when_a_ray_misses() {
        let (s1, s2) = default_world_objects();
        let w = default_world(&s1, &s2);
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 1.0, 0.0));
        let c = w.color_at(&r);
        assert_eq!(c, Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn the_color_when_a_ray_hits() {
        let (s1, s2) = default_world_objects();
        let w = default_world(&s1, &s2);
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let c = w.color_at(&r);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn the_color_with_an_intersection_behind_the_ray() {
        let (mut s1, mut s2) = default_world_objects();
        s1.material.ambient = 1.0;
        s2.material.ambient = 1.0;
        let w = default_world(&s1, &s2);
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.75), Tuple::vector(0.0, 0.0, -1.0));
        let c = w.color_at(&r);
        assert_eq!(c, w.objects[1].material().color);
    }

    #[test]
    fn there_is_no_shadow_when_nothing_is_collinear_with_point_and_light() {
        let (s1, s2) = default_world_objects();
        let w = default_world(&s1, &s2);
        let p = Tuple::point(0.0, 10.0, 0.0);
        assert_eq!(w.is_shadowed(p), false);
    }

    #[test]
    fn there_is_a_shadow_when_an_object_is_between_the_point_and_the_light() {
        let (s1, s2) = default_world_objects();
        let w = default_world(&s1, &s2);
        let p = Tuple::point(10.0, -10.0, 10.0);
        assert_eq!(w.is_shadowed(p), true);
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_light() {
        let (s1, s2) = default_world_objects();
        let w = default_world(&s1, &s2);
        let p = Tuple::point(-20.0, 20.0, -20.);
        assert_eq!(w.is_shadowed(p), false);
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_point() {
        let (s1, s2) = default_world_objects();
        let w = default_world(&s1, &s2);
        let p = Tuple::point(-2.0, 2.0, -2.0);
        assert_eq!(w.is_shadowed(p), false);
    }

    #[test]
    fn shade_hit_is_given_an_intersection_in_shadow() {
        let light = PointLight::new(Tuple::point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let s1 = Sphere::default();
        let s2 = Sphere::new(translation(0.0, 0.0, 10.0));
        let w = World::new(light, vec![&s1, &s2]);
        let r = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
        let i = Intersection::new(4.0, w.objects[1]);
        let comps = i.prepare_computations(&r);
        let c = w.shade_hit(&i, &comps);
        assert_eq!(c, Color::new(0.1, 0.1, 0.1));
    }
}
