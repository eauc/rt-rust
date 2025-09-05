use crate::coordinates::Coordinate;
use crate::spheres::Sphere;
use std::cmp;

#[derive(Debug, Clone, cmp::PartialEq)]
pub struct Intersection<'a> {
    t: Coordinate,
    object: &'a Sphere,
}

impl<'a> Intersection<'a> {
    pub fn new(t: Coordinate, object: &'a Sphere) -> Intersection<'a> {
        Intersection { t, object }
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

    #[test]
    fn an_intersection_encapsulates_t_and_object() {
        let s = Sphere::new();
        let i = Intersection::new(3.5, &s);
        assert_eq!(i.t, 3.5);
        assert_eq!(i.object, &s);
    }

    #[test]
    fn the_hit_when_all_intersections_have_positive_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs = vec![i2.clone(), i1.clone()];
        let i = hit(&xs);
        assert_eq!(i, Some(&i1));
    }

    #[test]
    fn the_hit_when_some_intersections_have_negative_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(1.0, &s);
        let xs = vec![i2.clone(), i1.clone()];
        let i = hit(&xs);
        assert_eq!(i, Some(&i2));
    }

    #[test]
    fn the_hit_when_all_intersections_have_negative_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(-2.0, &s);
        let i2 = Intersection::new(-1.0, &s);
        let xs = vec![i2.clone(), i1.clone()];
        let i = hit(&xs);
        assert_eq!(i, None);
    }
    // Scenario: The hit is always the lowest nonnegative intersection
    #[test]
    fn the_hit_is_always_the_lowest_nonnegative_intersection() {
        let s = Sphere::new();
        let i1 = Intersection::new(5.0, &s);
        let i2 = Intersection::new(7.0, &s);
        let i3 = Intersection::new(-3.0, &s);
        let i4 = Intersection::new(2.0, &s);
        let xs = vec![i1.clone(), i2.clone(), i3.clone(), i4.clone()];
        let i = hit(&xs);
        assert_eq!(i, Some(&i4));
    }
}
