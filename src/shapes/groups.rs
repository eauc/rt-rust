use crate::bounds::Bounds;
use crate::floats::Float;
use crate::intersections::Intersection;
use crate::matrices::Matrix;
use crate::objects::Object;
use crate::rays::Ray;
use crate::tuples::Tuple;

#[derive(Debug, Clone, PartialEq)]
pub struct Group {
    pub children: Vec<Object>,
}

impl Group {
    pub fn new() -> Group {
        Group { children: vec![] }
    }

    pub fn add_child(&mut self, object: Object) {
        self.children.push(object);
    }

    pub fn prepare_bounds(&mut self, bounds: &mut Bounds) {
        bounds.min = Tuple::point(Float::INFINITY, Float::INFINITY, Float::INFINITY);
        bounds.max = Tuple::point(-Float::INFINITY, -Float::INFINITY, -Float::INFINITY);
        for c in &mut self.children {
            c.prepare_bounds();
            let transformed_bounds = c.bounds.transform(&c.transform);
            // println!("bounds: {:#?}\ntransformed_bounds: {:#?}\n", c.bounds, transformed_bounds);
            bounds.merge(&transformed_bounds);
        }
        // println!("new bounds: {:#?}\n", bounds);
    }

    pub fn prepare_transform(&mut self, world_to_object: &Matrix<4>, object_to_world: &Matrix<4>) {
        for c in &mut self.children {
            c.world_to_object = c.transform_inverse * *world_to_object;
            c.object_to_world = *object_to_world * c.transform_inverse.transpose();
            c.prepare_transform();
        }
    }

    pub fn includes(&self, object: &Object) -> bool {
        self.children.iter().any(|c| c.includes(object))
    }

    pub fn local_intersect<'b>(&'b self, ray: &Ray, object: &'b Object) -> Vec<Intersection<'b>> {
        if !object.bounds.intersect(ray) {
            return vec![];
        }
        let mut xs = self
            .children
            .iter()
            .flat_map(|c| c.intersect(ray))
            .collect::<Vec<_>>();
        xs.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        xs
    }

    pub fn local_normal_at(&self, _local_point: Tuple) -> Tuple {
        panic!("We should never call local_normal_at on a group");
    }
}

impl Default for Group {
    fn default() -> Group {
        Group::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transformations::{scaling, translation};
    use crate::tuples::Tuple;

    #[test]
    fn intersecting_a_ray_with_an_empty_group() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let g = Object::new_group();
        let xs = g.as_group().local_intersect(&r, &g);
        assert!(xs.is_empty());
    }

    #[test]
    fn intersecting_a_ray_with_a_nonempty_group() {
        let mut g = Object::new_group();
        let s1 = Object::new_sphere();
        let s2 = Object::new_sphere().with_transform(translation(0.0, 0.0, -3.0));
        let s3 = Object::new_sphere().with_transform(translation(5.0, 0.0, 0.0));
        g.as_mut_group().add_child(s1);
        g.as_mut_group().add_child(s2);
        g.as_mut_group().add_child(s3);
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = g.as_group().local_intersect(&r, &g);
        assert_eq!(
            xs.iter().map(|x| x.t).collect::<Vec<_>>(),
            vec![1.0, 3.0, 4.0, 6.0]
        );
    }

    #[test]
    fn intersecting_a_transformed_group() {
        let mut g = Object::new_group().with_transform(scaling(2.0, 2.0, 2.0));
        let s = Object::new_sphere().with_transform(translation(5.0, 0.0, 0.0));
        g.as_mut_group().add_child(s);
        g.prepare();
        let r = Ray::new(Tuple::point(10.0, 0.0, -10.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = g.intersect(&r);
        assert_eq!(xs.len(), 2);
    }
}
