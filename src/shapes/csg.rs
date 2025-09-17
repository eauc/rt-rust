use crate::bounds::Bounds;
use crate::floats::Float;
use crate::intersections::Intersection;
use crate::matrices::Matrix;
use crate::objects::Object;
use crate::rays::Ray;
use crate::tuples::Tuple;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operation {
    Difference,
    Intersection,
    Union,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Csg {
    operation: Operation,
    children: Vec<Object>,
}

impl Csg {
    pub fn new(operation: Operation, left: Object, right: Object) -> Csg {
        Csg {
            operation,
            children: vec![left, right],
        }
    }

    pub fn prepare_bounds(&mut self, bounds: &mut Bounds) {
        bounds.min = Tuple::point(Float::INFINITY, Float::INFINITY, Float::INFINITY);
        bounds.max = Tuple::point(-Float::INFINITY, -Float::INFINITY, -Float::INFINITY);
        for c in &mut self.children {
            c.prepare_bounds();
            let transformed_bounds = c.bounds.transform(&c.transform);
            bounds.merge(&transformed_bounds);
        }
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

    pub fn local_intersect<'a>(&'a self, ray: &Ray, _object: &'a Object) -> Vec<Intersection<'a>> {
        let mut xs = self
            .children
            .iter()
            .flat_map(|c| c.intersect(ray))
            .collect::<Vec<_>>();
        xs.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        self.filter_intersections(xs)
    }

    pub fn local_normal_at(&self, _point: Tuple) -> Tuple {
        panic!("We should never call local_normal_at on a group");
    }

    fn filter_intersections<'a>(&self, xs: Vec<Intersection<'a>>) -> Vec<Intersection<'a>> {
        let left = &self.children[0];
        let mut inl = false;
        let mut inr = false;
        let mut result = Vec::with_capacity(xs.len());
        for x in xs {
            let lhit = left.includes(x.object);
            if intersection_allowed(self.operation, lhit, inl, inr) {
                result.push(x);
            }
            if lhit {
                inl = !inl;
            } else {
                inr = !inr;
            }
        }
        result
    }
}

fn intersection_allowed(op: Operation, lhit: bool, inl: bool, inr: bool) -> bool {
    match op {
        Operation::Difference => (lhit && !inr) || (!lhit && inl),
        Operation::Intersection => (lhit && inr) || (!lhit && inl),
        Operation::Union => (lhit && !inr) || (!lhit && !inl),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transformations::translation;

    #[test]
    fn csg_is_created_with_an_operation_and_two_shapes() {
        let s1 = Object::new_sphere();
        let s2 = Object::new_cube();
        let c = Object::new_csg(Operation::Union, s1, s2);
        assert_eq!(c.as_csg().operation, Operation::Union);
        assert_eq!(c.as_csg().children.len(), 2);
    }

    #[test]
    fn evaluating_the_rule_of_a_csg_shape() {
        let tests = vec![
            (Operation::Union, true, true, true, false),
            (Operation::Union, true, true, false, true),
            (Operation::Union, true, false, true, false),
            (Operation::Union, true, false, false, true),
            (Operation::Union, false, true, true, false),
            (Operation::Union, false, false, true, true),
            (Operation::Union, false, false, false, true),
            (Operation::Intersection, true, true, true, true),
            (Operation::Intersection, true, true, false, false),
            (Operation::Intersection, true, false, true, true),
            (Operation::Intersection, true, false, false, false),
            (Operation::Intersection, false, true, true, true),
            (Operation::Intersection, false, true, false, true),
            (Operation::Intersection, false, false, true, false),
            (Operation::Intersection, false, false, false, false),
            (Operation::Difference, true, true, true, false),
            (Operation::Difference, true, true, false, true),
            (Operation::Difference, true, false, true, false),
            (Operation::Difference, true, false, false, true),
            (Operation::Difference, false, true, true, true),
            (Operation::Difference, false, true, false, true),
            (Operation::Difference, false, false, true, false),
            (Operation::Difference, false, false, false, false),
        ];
        for (op, lhit, inl, inr, result) in tests {
            assert_eq!(intersection_allowed(op, lhit, inl, inr), result,);
        }
    }

    #[test]
    fn filtering_a_list_of_intersections() {
        let tests = vec![
            (Operation::Union, vec![1.0, 4.0]),
            (Operation::Intersection, vec![2.0, 3.0]),
            (Operation::Difference, vec![1.0, 2.0]),
        ];
        for (op, expected) in tests {
            let s1 = Object::new_sphere();
            let s2 = Object::new_cube();
            let c = Object::new_csg(op, s1, s2);
            let xs = vec![
                Intersection::new(1.0, &c.as_csg().children[0]),
                Intersection::new(2.0, &c.as_csg().children[1]),
                Intersection::new(3.0, &c.as_csg().children[0]),
                Intersection::new(4.0, &c.as_csg().children[1]),
            ];
            let result = c.as_csg().filter_intersections(xs);
            assert_eq!(result.iter().map(|x| x.t).collect::<Vec<_>>(), expected);
        }
    }

    #[test]
    fn a_ray_misses_a_csg_object() {
        let s1 = Object::new_sphere();
        let s2 = Object::new_cube();
        let c = Object::new_csg(Operation::Union, s1, s2);
        let r = Ray::new(Tuple::point(0.0, 2.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = c.as_csg().local_intersect(&r, &c);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn a_ray_hits_a_csg_object() {
        let s1 = Object::new_sphere();
        let s2 = Object::new_sphere().with_transform(translation(0.0, 0.0, 0.5));
        let c = Object::new_csg(Operation::Union, s1, s2);
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = c.as_csg().local_intersect(&r, &c);
        assert_eq!(xs.iter().map(|x| x.t).collect::<Vec<_>>(), vec![4.0, 6.5]);
    }
}
