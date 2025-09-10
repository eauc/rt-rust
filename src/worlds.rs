use crate::colors::{BLACK, Color};
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

    fn reflected_color(
        &self,
        hit: &Intersection,
        comps: &IntersectionComputations,
        depth: u32,
    ) -> Color {
        if depth == 0 || hit.object.material().reflective == 0.0 {
            return BLACK;
        }
        let reflect_ray = Ray::new(comps.over_point, comps.reflectv);
        let color = self.color_at(&reflect_ray, depth - 1);
        color * hit.object.material().reflective
    }

    fn refracted_color(
        &self,
        hit: &Intersection,
        comps: &IntersectionComputations,
        depth: u32,
    ) -> Color {
        if depth == 0 || hit.object.material().transparency == 0.0 {
            return BLACK;
        }
        let n_ratio = comps.n1 / comps.n2;
        let cos_i = comps.eyev.dot(comps.normalv);
        let sin2_t = n_ratio.powi(2) * (1.0 - cos_i.powi(2));
        if sin2_t > 1.0 {
            return BLACK;
        }
        let cos_t = (1.0 - sin2_t).sqrt();
        let direction = comps.normalv * (n_ratio * cos_i - cos_t) - comps.eyev * n_ratio;
        let refract_ray = Ray::new(comps.under_point, direction);
        let color = self.color_at(&refract_ray, depth - 1);
        color * hit.object.material().transparency
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

    fn shade_hit(&self, hit: &Intersection, comps: &IntersectionComputations, depth: u32) -> Color {
        let is_shadowed = self.is_shadowed(comps.over_point);
        let surface = hit.object.material().lighting(
            hit.object,
            &self.light,
            comps.over_point,
            comps.eyev,
            comps.normalv,
            is_shadowed,
        );
        let reflected = self.reflected_color(hit, comps, depth);
        let refracted = self.refracted_color(hit, comps, depth);
        surface + reflected + refracted
    }

    pub fn color_at(&self, ray: &Ray, depth: u32) -> Color {
        let xs = self.intersect(ray);
        if let Some(hit) = intersections::hit(&xs) {
            let comps = hit.prepare_computations(&ray, &xs);
            self.shade_hit(&hit, &comps, depth)
        } else {
            crate::colors::BLACK
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::colors::BLACK;
    use crate::matrices::Matrix;
    use crate::patterns::tests::TestPattern;
    use crate::planes::Plane;
    use crate::spheres::Sphere;
    use crate::transformations::{scaling, translation};
    use std::sync::Arc;

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
        let comps = i.prepare_computations(&r, &vec![]);
        let c = w.shade_hit(&i, &comps, 1);
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
        let comps = i.prepare_computations(&r, &vec![]);
        let c = w.shade_hit(&i, &comps, 1);
        assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn the_color_when_a_ray_misses() {
        let (s1, s2) = default_world_objects();
        let w = default_world(&s1, &s2);
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 1.0, 0.0));
        let c = w.color_at(&r, 1);
        assert_eq!(c, Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn the_color_when_a_ray_hits() {
        let (s1, s2) = default_world_objects();
        let w = default_world(&s1, &s2);
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let c = w.color_at(&r, 1);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn the_color_with_an_intersection_behind_the_ray() {
        let (mut s1, mut s2) = default_world_objects();
        s1.material.ambient = 1.0;
        s2.material.ambient = 1.0;
        let w = default_world(&s1, &s2);
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.75), Tuple::vector(0.0, 0.0, -1.0));
        let c = w.color_at(&r, 1);
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
        let comps = i.prepare_computations(&r, &vec![]);
        let c = w.shade_hit(&i, &comps, 1);
        assert_eq!(c, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn the_reflected_color_for_a_nonreflective_material() {
        let (s1, mut s2) = default_world_objects();
        s2.material.ambient = 1.0;
        let w = default_world(&s1, &s2);
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let i = Intersection::new(1.0, &s2);
        let comps = i.prepare_computations(&r, &vec![]);
        let color = w.reflected_color(&i, &comps, 1);
        assert_eq!(color, BLACK);
    }

    #[test]
    fn the_reflected_color_for_a_reflective_material() {
        let (s1, s2) = default_world_objects();
        let mut w = default_world(&s1, &s2);
        let mut shape = Plane::new(translation(0.0, -1.0, 0.0));
        shape.material.reflective = 0.5;
        w.objects.push(&shape);
        let r = Ray::new(
            Tuple::point(0.0, 0.0, -3.0),
            Tuple::vector(0.0, -(2.0_f32).sqrt() / 2.0, (2.0_f32).sqrt() / 2.0),
        );
        let i = Intersection::new((2.0_f32).sqrt(), &shape);
        let comps = i.prepare_computations(&r, &vec![]);
        let color = w.reflected_color(&i, &comps, 1);
        assert_eq!(color, Color::new(0.19032222, 0.23791526, 0.14274));
    }

    #[test]
    fn the_reflected_color_at_the_maximum_recursive_depth() {
        let (s1, s2) = default_world_objects();
        let mut w = default_world(&s1, &s2);
        let mut shape = Plane::new(translation(0.0, -1.0, 0.0));
        shape.material.reflective = 0.5;
        w.objects.push(&shape);
        let r = Ray::new(
            Tuple::point(0.0, 0.0, -3.0),
            Tuple::vector(0.0, -(2.0_f32).sqrt() / 2.0, (2.0_f32).sqrt() / 2.0),
        );
        let i = Intersection::new((2.0_f32).sqrt(), &shape);
        let comps = i.prepare_computations(&r, &vec![]);
        let color = w.reflected_color(&i, &comps, 0);
        assert_eq!(color, BLACK);
    }

    #[test]
    fn shade_hit_with_a_reflective_material() {
        let (s1, s2) = default_world_objects();
        let mut w = default_world(&s1, &s2);
        let mut shape = Plane::new(translation(0.0, -1.0, 0.0));
        shape.material.reflective = 0.5;
        w.objects.push(&shape);
        let r = Ray::new(
            Tuple::point(0.0, 0.0, -3.0),
            Tuple::vector(0.0, -(2.0_f32).sqrt() / 2.0, (2.0_f32).sqrt() / 2.0),
        );
        let i = Intersection::new((2.0_f32).sqrt(), &shape);
        let comps = i.prepare_computations(&r, &vec![]);
        let color = w.shade_hit(&i, &comps, 1);
        assert_eq!(color, Color::new(0.8767573, 0.924340374, 0.8291743));
    }

    #[test]
    fn color_at_with_mutually_reflective_surfaces() {
        let mut w = World::new(
            PointLight::new(Tuple::point(0.0, 0.0, 0.0), Color::new(1.0, 1.0, 1.0)),
            vec![],
        );
        let mut lower = Plane::new(translation(0.0, -1.0, 0.0));
        lower.material.reflective = 1.0;
        w.objects.push(&lower);
        let mut upper = Plane::new(translation(0.0, 1.0, 0.0));
        upper.material.reflective = 1.0;
        w.objects.push(&upper);
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 1.0, 0.0));
        assert_eq!(
            w.color_at(&r, 10),
            Color::new(20.899998, 20.899998, 20.899998)
        );
    }

    #[test]
    fn the_refracted_color_with_an_opaque_surface() {
        let (s1, s2) = default_world_objects();
        let w = default_world(&s1, &s2);
        let shape = w.objects[0];
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = vec![Intersection::new(4.0, shape), Intersection::new(6.0, shape)];
        let comps = xs[0].prepare_computations(&r, &xs);
        let c = w.refracted_color(&xs[0], &comps, 5);
        assert_eq!(c, BLACK);
    }

    #[test]
    fn the_refracted_color_at_the_maximum_recursive_depth() {
        let (mut s1, s2) = default_world_objects();
        s1.material.transparency = 1.0;
        s1.material.refractive_index = 1.5;
        let w = default_world(&s1, &s2);
        let shape = w.objects[0];
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = vec![Intersection::new(4.0, shape), Intersection::new(6.0, shape)];
        let comps = xs[0].prepare_computations(&r, &xs);
        let c = w.refracted_color(&xs[0], &comps, 0);
        assert_eq!(c, BLACK);
    }

    #[test]
    fn the_refracted_color_under_total_internal_reflection() {
        let (mut s1, s2) = default_world_objects();
        s1.material.transparency = 1.0;
        s1.material.refractive_index = 1.5;
        let w = default_world(&s1, &s2);
        let shape = w.objects[0];
        let r = Ray::new(
            Tuple::point(0.0, 0.0, 2.0_f32.sqrt() / 2.0),
            Tuple::vector(0.0, 1.0, 0.0),
        );
        let xs = vec![
            Intersection::new(-(2.0_f32).sqrt() / 2.0, shape),
            Intersection::new((2.0_f32).sqrt() / 2.0, shape),
        ];
        let comps = xs[1].prepare_computations(&r, &xs);
        let c = w.refracted_color(&xs[1], &comps, 5);
        assert_eq!(c, BLACK);
    }

    #[test]
    fn the_refracted_color_with_a_refracted_ray() {
        let (mut s1, mut s2) = default_world_objects();
        s1.material.ambient = 1.0;
        s1.material.pattern = Some(Arc::new(TestPattern::new(Matrix::identity())));
        s2.material.transparency = 1.0;
        s2.material.refractive_index = 1.5;
        let w = default_world(&s1, &s2);
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.1), Tuple::vector(0.0, 1.0, 0.0));
        let xs = vec![
            Intersection::new(-0.9899, &s1),
            Intersection::new(-0.4899, &s2),
            Intersection::new(0.4899, &s2),
            Intersection::new(0.9899, &s1),
        ];
        let comps = xs[2].prepare_computations(&r, &xs);
        let c = w.refracted_color(&xs[2], &comps, 5);
        assert_eq!(c, Color::new(0.0, 0.99887455, 0.0472189175));
    }

    #[test]
    fn shade_hit_with_a_transparent_material() {
        let (s1, s2) = default_world_objects();
        let mut floor = Plane::new(translation(0.0, -1.0, 0.0));
        floor.material.transparency = 0.5;
        floor.material.refractive_index = 1.5;
        let mut ball = Sphere::new(translation(0.0, -3.5, -0.5));
        ball.material.color = Color::new(1.0, 0.0, 0.0);
        ball.material.ambient = 0.5;
        let mut w = default_world(&s1, &s2);
        w.objects.push(&floor);
        w.objects.push(&ball);
        let r = Ray::new(
            Tuple::point(0.0, 0.0, -3.0),
            Tuple::vector(0.0, -2.0_f32.sqrt() / 2.0, 2.0_f32.sqrt() / 2.0),
        );
        let i = Intersection::new(2.0_f32.sqrt(), &floor);
        let xs = vec![i];
        let comps = xs[0].prepare_computations(&r, &xs);
        let c = w.shade_hit(&xs[0], &comps, 5);
        assert_eq!(c, Color::new(0.9364251, 0.6864251, 0.6864251));
    }
}
