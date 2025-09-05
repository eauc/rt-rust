use crate::coordinates::Coordinate;
use crate::matrices::Matrix;

fn translation(x: Coordinate, y: Coordinate, z: Coordinate) -> Matrix<4> {
    Matrix::new([
        [1.0, 0.0, 0.0, x],
        [0.0, 1.0, 0.0, y],
        [0.0, 0.0, 1.0, z],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

fn scaling(x: Coordinate, y: Coordinate, z: Coordinate) -> Matrix<4> {
    Matrix::new([
        [x, 0.0, 0.0, 0.0],
        [0.0, y, 0.0, 0.0],
        [0.0, 0.0, z, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

fn rotation_x(r: Coordinate) -> Matrix<4> {
    Matrix::new([
        [1.0, 0.0, 0.0, 0.0],
        [0.0, r.cos(), -r.sin(), 0.0],
        [0.0, r.sin(), r.cos(), 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

fn rotation_y(r: Coordinate) -> Matrix<4> {
    Matrix::new([
        [r.cos(), 0.0, r.sin(), 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [-r.sin(), 0.0, r.cos(), 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

fn rotation_z(r: Coordinate) -> Matrix<4> {
    Matrix::new([
        [r.cos(), -r.sin(), 0.0, 0.0],
        [r.sin(), r.cos(), 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

fn shearing(
    xy: Coordinate,
    xz: Coordinate,
    yx: Coordinate,
    yz: Coordinate,
    zx: Coordinate,
    zy: Coordinate,
) -> Matrix<4> {
    Matrix::new([
        [1.0, xy, xz, 0.0],
        [yx, 1.0, yz, 0.0],
        [zx, zy, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tuples::Tuple;

    #[test]
    fn multiplying_by_a_translation_matrix() {
        let transform = translation(5.0, -3.0, 2.0);
        let p = Tuple::point(-3.0, 4.0, 5.0);
        assert_eq!(transform * p, Tuple::point(2.0, 1.0, 7.0));
    }

    #[test]
    fn multiplying_by_the_inverse_of_a_translation_matrix() {
        let transform = translation(5.0, -3.0, 2.0);
        let inv = transform.inverse();
        let p = Tuple::point(-3.0, 4.0, 5.0);
        assert_eq!(inv * p, Tuple::point(-8.0, 7.0, 3.0));
    }

    #[test]
    fn translation_does_not_affect_vectors() {
        let transform = translation(5.0, -3.0, 2.0);
        let v = Tuple::vector(-3.0, 4.0, 5.0);
        assert_eq!(transform * v, v);
    }

    #[test]
    fn a_scaling_matrix_applied_to_a_point() {
        let transform = scaling(2.0, 3.0, 4.0);
        let p = Tuple::point(-4.0, 6.0, 8.0);
        assert_eq!(transform * p, Tuple::point(-8.0, 18.0, 32.0));
    }

    #[test]
    fn a_scaling_matrix_applied_to_a_vector() {
        let transform = scaling(2.0, 3.0, 4.0);
        let v = Tuple::vector(-4.0, 6.0, 8.0);
        assert_eq!(transform * v, Tuple::vector(-8.0, 18.0, 32.0));
    }

    #[test]
    fn multiplying_by_the_inverse_of_a_scaling_matrix() {
        let transform = scaling(2.0, 3.0, 4.0);
        let inv = transform.inverse();
        let v = Tuple::vector(-4.0, 6.0, 8.0);
        assert_eq!(inv * v, Tuple::vector(-2.0, 2.0, 2.0));
    }

    #[test]
    fn reflection_is_scaling_by_a_negative_value() {
        let transform = scaling(-1.0, 1.0, 1.0);
        let p = Tuple::point(2.0, 3.0, 4.0);
        assert_eq!(transform * p, Tuple::point(-2.0, 3.0, 4.0));
    }

    #[test]
    fn rotating_a_point_around_the_x_axis() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(std::f32::consts::PI / 4.0);
        let full_quarter = rotation_x(std::f32::consts::PI / 2.0);
        assert_eq!(
            half_quarter * p,
            Tuple::point(
                0.0,
                std::f32::consts::SQRT_2 / 2.0,
                std::f32::consts::SQRT_2 / 2.0
            )
        );
        assert_eq!(full_quarter * p, Tuple::point(0.0, 0.0, 1.0));
    }

    #[test]
    fn the_inverse_of_an_x_rotation_rotates_in_the_opposite_direction() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(std::f32::consts::PI / 4.0);
        let inv = half_quarter.inverse();
        assert_eq!(
            inv * p,
            Tuple::point(
                0.0,
                std::f32::consts::SQRT_2 / 2.0,
                -std::f32::consts::SQRT_2 / 2.0
            )
        );
    }

    #[test]
    fn rotating_a_point_around_the_y_axis() {
        let p = Tuple::point(0.0, 0.0, 1.0);
        let half_quarter = rotation_y(std::f32::consts::PI / 4.0);
        let full_quarter = rotation_y(std::f32::consts::PI / 2.0);
        assert_eq!(
            half_quarter * p,
            Tuple::point(
                std::f32::consts::SQRT_2 / 2.0,
                0.0,
                std::f32::consts::SQRT_2 / 2.0
            )
        );
        assert_eq!(full_quarter * p, Tuple::point(1.0, 0.0, 0.0));
    }

    #[test]
    fn rotating_a_point_around_the_z_axis() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = rotation_z(std::f32::consts::PI / 4.0);
        let full_quarter = rotation_z(std::f32::consts::PI / 2.0);
        assert_eq!(
            half_quarter * p,
            Tuple::point(
                -std::f32::consts::SQRT_2 / 2.0,
                std::f32::consts::SQRT_2 / 2.0,
                0.0
            )
        );
        assert_eq!(full_quarter * p, Tuple::point(-1.0, 0.0, 0.0));
    }

    #[test]
    fn a_shearing_transformation_moves_x_in_proportion_to_z() {
        let transform = shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);
        assert_eq!(transform * p, Tuple::point(6.0, 3.0, 4.0));
    }

    #[test]
    fn a_shearing_transformation_moves_y_in_proportion_to_x() {
        let transform = shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);
        assert_eq!(transform * p, Tuple::point(2.0, 5.0, 4.0));
    }

    #[test]
    fn a_shearing_transformation_moves_y_in_proportion_to_z() {
        let transform = shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);
        assert_eq!(transform * p, Tuple::point(2.0, 7.0, 4.0));
    }

    #[test]
    fn a_shearing_transformation_moves_z_in_proportion_to_x() {
        let transform = shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);
        assert_eq!(transform * p, Tuple::point(2.0, 3.0, 6.0));
    }
    #[test]
    fn a_shearing_transformation_moves_z_in_proportion_to_y() {
        let transform = shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = Tuple::point(2.0, 3.0, 4.0);
        assert_eq!(transform * p, Tuple::point(2.0, 3.0, 7.0));
    }

    #[test]
    fn individual_transformations_are_applied_in_sequence() {
        let p = Tuple::point(1.0, 0.0, 1.0);
        let a = rotation_x(std::f32::consts::PI / 2.0);
        let b = scaling(5.0, 5.0, 5.0);
        let c = translation(10.0, 5.0, 7.0);
        let p2 = a * p;
        assert_eq!(p2, Tuple::point(1.0, -1.0, 0.0));
        let p3 = b * p2;
        assert_eq!(p3, Tuple::point(5.0, -5.0, 0.0));
        let p4 = c * p3;
        assert_eq!(p4, Tuple::point(15.0, 0.0, 7.0));
    }

    #[test]
    fn chained_transformations_must_be_applied_in_reverse_order() {
        let p = Tuple::point(1.0, 0.0, 1.0);
        let a = rotation_x(std::f32::consts::PI / 2.0);
        let b = scaling(5.0, 5.0, 5.0);
        let c = translation(10.0, 5.0, 7.0);
        let t = c * b * a;
        assert_eq!(t * p, Tuple::point(15.0, 0.0, 7.0));
    }
}
