use crate::colors::Color;
use crate::intersections;
use crate::intersections::Intersection;
use crate::intersections::IntersectionComputations;
use crate::lights::PointLight;
use crate::rays::Ray;
use crate::spheres::Sphere;
use crate::transformations;
use crate::tuples::Tuple;

pub struct World {
    pub light: PointLight,
    pub objects: Vec<Sphere>,
}

impl World {
    pub fn default() -> World {
        let mut s1 = Sphere::new();
        s1.material.color = Color(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;
        let mut s2 = Sphere::new();
        s2.transform = transformations::scaling(0.5, 0.5, 0.5);
        World {
            light: PointLight::new(Tuple::point(-10.0, 10.0, -10.0), Color(1.0, 1.0, 1.0)),
            objects: vec![s1, s2],
        }
    }

    fn intersect<'a>(&'a self, ray: &Ray) -> Vec<Intersection<'a>> {
        let mut intersections = self
            .objects
            .iter()
            .flat_map(|s| s.intersect(ray))
            .collect::<Vec<Intersection>>();
        intersections.sort_by(|i1, i2| i1.t.partial_cmp(&i2.t).unwrap());
        intersections
    }

    fn shade_hit(&self, comps: &IntersectionComputations) -> Color {
        comps
            .object
            .material
            .lighting(&self.light, comps.point, comps.eyev, comps.normalv)
    }

    pub fn color_at(&self, ray: &Ray) -> Color {
        let xs = self.intersect(ray);
        if let Some(hit) = intersections::hit(&xs) {
            let comps = hit.prepare_computations(&ray);
            self.shade_hit(&comps)
        } else {
            crate::colors::BLACK
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_default_world() {
        let light = PointLight::new(Tuple::point(-10.0, 10.0, -10.0), Color(1.0, 1.0, 1.0));
        let mut s1 = Sphere::new();
        s1.material.color = Color(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;
        let mut s2 = Sphere::new();
        s2.transform = transformations::scaling(0.5, 0.5, 0.5);
        let w = World::default();
        assert_eq!(w.light, light);
        assert_eq!(w.objects, vec![s1, s2]);
    }

    #[test]
    fn intersect_a_world_with_a_ray() {
        let w = World::default();
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
        let w = World::default();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = &w.objects[0];
        let i = Intersection::new(4.0, shape);
        let comps = i.prepare_computations(&r);
        let c = w.shade_hit(&comps);
        assert_eq!(c, Color(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let mut w = World::default();
        w.light = PointLight::new(Tuple::point(0.0, 0.25, 0.0), Color(1.0, 1.0, 1.0));
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = &w.objects[1];
        let i = Intersection::new(0.5, shape);
        let comps = i.prepare_computations(&r);
        let c = w.shade_hit(&comps);
        assert_eq!(c, Color(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn the_color_when_a_ray_misses() {
        let w = World::default();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 1.0, 0.0));
        let c = w.color_at(&r);
        assert_eq!(c, Color(0.0, 0.0, 0.0));
    }

    #[test]
    fn the_color_when_a_ray_hits() {
        let w = World::default();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let c = w.color_at(&r);
        assert_eq!(c, Color(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn the_color_with_an_intersection_behind_the_ray() {
        let mut w = World::default();
        let outer = &mut w.objects[0];
        outer.material.ambient = 1.0;
        let inner = &mut w.objects[1];
        inner.material.ambient = 1.0;
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.75), Tuple::vector(0.0, 0.0, -1.0));
        let c = w.color_at(&r);
        assert_eq!(c, w.objects[1].material.color);
    }
}
