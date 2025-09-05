use crate::coordinates::Coordinate;
use crate::coordinates::equals;
use crate::tuples::Tuple;
use std::cmp;
use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct Matrix<const M: usize>([[Coordinate; M]; M]);

impl<const M: usize> Matrix<M> {
    pub fn new(data: [[Coordinate; M]; M]) -> Matrix<M> {
        Matrix(data)
    }

    fn identity() -> Matrix<M> {
        let mut data = [[0.0; M]; M];
        for i in 0..M {
            data[i][i] = 1.0;
        }
        Matrix(data)
    }

    fn transpose(&self) -> Matrix<M> {
        let mut data = [[0.0; M]; M];
        for i in 0..M {
            for j in 0..M {
                data[j][i] = self.0[i][j];
            }
        }
        Matrix(data)
    }
}

impl Matrix<2> {
    fn determinant(&self) -> Coordinate {
        self.0[0][0] * self.0[1][1] - self.0[0][1] * self.0[1][0]
    }
}

impl Matrix<3> {
    fn submatrix(self, row: usize, column: usize) -> Matrix<2> {
        let mut data = [[0.0; 2]; 2];
        for i in 0..3 {
            if i != row {
                let r = if i < row { i } else { i - 1 };
                for j in 0..3 {
                    if j != column {
                        let c = if j < column { j } else { j - 1 };
                        data[r][c] = self.0[i][j];
                    }
                }
            }
        }
        return Matrix(data);
    }

    fn minor(&self, row: usize, column: usize) -> Coordinate {
        self.submatrix(row, column).determinant()
    }

    fn cofactor(&self, row: usize, column: usize) -> Coordinate {
        let minor = self.minor(row, column);
        if (row + column) % 2 == 0 {
            minor
        } else {
            -minor
        }
    }

    fn determinant(&self) -> Coordinate {
        self.0[0][0] * self.cofactor(0, 0)
            + self.0[0][1] * self.cofactor(0, 1)
            + self.0[0][2] * self.cofactor(0, 2)
    }
}

impl Matrix<4> {
    fn submatrix(self, row: usize, column: usize) -> Matrix<3> {
        let mut data = [[0.0; 3]; 3];
        for i in 0..4 {
            if i != row {
                let r = if i < row { i } else { i - 1 };
                for j in 0..4 {
                    if j != column {
                        let c = if j < column { j } else { j - 1 };
                        data[r][c] = self.0[i][j];
                    }
                }
            }
        }
        return Matrix(data);
    }

    fn minor(&self, row: usize, column: usize) -> Coordinate {
        self.submatrix(row, column).determinant()
    }

    fn cofactor(&self, row: usize, column: usize) -> Coordinate {
        let minor = self.minor(row, column);
        if (row + column) % 2 == 0 {
            minor
        } else {
            -minor
        }
    }

    fn determinant(&self) -> Coordinate {
        self.0[0][0] * self.cofactor(0, 0)
            + self.0[0][1] * self.cofactor(0, 1)
            + self.0[0][2] * self.cofactor(0, 2)
            + self.0[0][3] * self.cofactor(0, 3)
    }

    fn is_invertible(&self) -> bool {
        !equals(self.determinant(), 0.0)
    }

    pub fn inverse(&self) -> Matrix<4> {
        if !self.is_invertible() {
            panic!("Matrix is not invertible");
        }
        let det = self.determinant();
        let mut result = Matrix::new([[0.0; 4]; 4]);
        for i in 0..4 {
            for j in 0..4 {
                result[(j, i)] = self.cofactor(i, j) / det;
            }
        }
        result
    }
}

impl<const M: usize> ops::Index<(usize, usize)> for Matrix<M> {
    type Output = Coordinate;

    fn index(&self, index: (usize, usize)) -> &Coordinate {
        &self.0[index.0][index.1]
    }
}

impl<const M: usize> ops::IndexMut<(usize, usize)> for Matrix<M> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Coordinate {
        &mut self.0[index.0][index.1]
    }
}

impl<const M: usize> cmp::PartialEq for Matrix<M> {
    fn eq(&self, other: &Matrix<M>) -> bool {
        let eq = true;
        for i in 0..M {
            for j in 0..M {
                if !equals(self[(i, j)], other[(i, j)]) {
                    return false;
                }
            }
        }
        eq
    }
}

impl<const M: usize> ops::Mul for Matrix<M> {
    type Output = Matrix<M>;
    fn mul(self, other: Matrix<M>) -> Matrix<M> {
        let mut result = Matrix::new([[0.0; M]; M]);
        for i in 0..M {
            for j in 0..M {
                for k in 0..M {
                    result[(i, j)] += self[(i, k)] * other[(k, j)];
                }
            }
        }
        result
    }
}

impl ops::Mul<Tuple> for Matrix<4> {
    type Output = Tuple;

    fn mul(self, other: Tuple) -> Tuple {
        Tuple(
            self[(0, 0)] * other.x()
                + self[(0, 1)] * other.y()
                + self[(0, 2)] * other.z()
                + self[(0, 3)] * other.w(),
            self[(1, 0)] * other.x()
                + self[(1, 1)] * other.y()
                + self[(1, 2)] * other.z()
                + self[(1, 3)] * other.w(),
            self[(2, 0)] * other.x()
                + self[(2, 1)] * other.y()
                + self[(2, 2)] * other.z()
                + self[(2, 3)] * other.w(),
            self[(3, 0)] * other.x()
                + self[(3, 1)] * other.y()
                + self[(3, 2)] * other.z()
                + self[(3, 3)] * other.w(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructing_and_inspecting_a_4x4_matrix() {
        let m = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ]);

        assert_eq!(m[(0, 0)], 1.0);
        assert_eq!(m[(0, 3)], 4.0);
        assert_eq!(m[(1, 0)], 5.5);
        assert_eq!(m[(1, 2)], 7.5);
        assert_eq!(m[(2, 2)], 11.0);
        assert_eq!(m[(3, 0)], 13.5);
        assert_eq!(m[(3, 2)], 15.5);
    }

    #[test]
    fn a_2x2_matrix_ought_to_be_representable() {
        let m = Matrix::new([[1.0, 2.0], [3.0, 4.0]]);
        assert_eq!(m[(0, 0)], 1.0);
        assert_eq!(m[(0, 1)], 2.0);
        assert_eq!(m[(1, 0)], 3.0);
        assert_eq!(m[(1, 1)], 4.0);
    }

    #[test]
    fn a_3x3_matrix_ought_to_be_representable() {
        let m = Matrix::new([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
        assert_eq!(m[(0, 0)], 1.0);
        assert_eq!(m[(1, 1)], 5.0);
        assert_eq!(m[(2, 2)], 9.0);
    }

    #[test]
    fn matrix_equality_with_identical_matrices() {
        let a = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let b = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        assert_eq!(a, b);
    }

    #[test]
    fn matrix_equality_with_different_matrices() {
        let a = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let b = Matrix::new([
            [2.0, 3.0, 4.0, 5.0],
            [6.0, 7.0, 8.0, 9.0],
            [8.0, 7.0, 6.0, 5.0],
            [4.0, 3.0, 2.0, 1.0],
        ]);
        assert_ne!(a, b);
    }

    #[test]
    fn multiplying_two_matrices() {
        let a = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let b = Matrix::new([
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ]);
        assert_eq!(
            a * b,
            Matrix::new([
                [20.0, 22.0, 50.0, 48.0],
                [44.0, 54.0, 114.0, 108.0],
                [40.0, 58.0, 110.0, 102.0],
                [16.0, 26.0, 46.0, 42.0],
            ])
        );
    }

    #[test]
    fn a_matrix_multiplied_by_a_tuple() {
        let a = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let b = Tuple(1.0, 2.0, 3.0, 1.0);
        assert_eq!(a * b, Tuple(18.0, 24.0, 33.0, 1.0));
    }

    #[test]
    fn multiplying_a_matrix_by_the_identity_matrix() {
        let a = Matrix::new([
            [0.0, 1.0, 2.0, 4.0],
            [1.0, 2.0, 4.0, 8.0],
            [2.0, 4.0, 8.0, 16.0],
            [4.0, 8.0, 16.0, 32.0],
        ]);
        assert_eq!(a * Matrix::identity(), a);
    }

    #[test]
    fn multiplying_the_identity_matrix_by_a_tuple() {
        let a = Matrix::identity();
        let b = Tuple(1.0, 2.0, 3.0, 4.0);
        assert_eq!(a * b, b);
    }

    // Scenario: Transposing a matrix
    #[test]
    fn transposing_a_matrix() {
        let a = Matrix::new([
            [0.0, 9.0, 1.0, 0.0],
            [9.0, 8.0, 8.0, 0.0],
            [3.0, 0.0, 5.0, 5.0],
            [0.0, 8.0, 3.0, 8.0],
        ]);
        assert_eq!(
            a.transpose(),
            Matrix::new([
                [0.0, 9.0, 3.0, 0.0],
                [9.0, 8.0, 0.0, 8.0],
                [1.0, 8.0, 5.0, 3.0],
                [0.0, 0.0, 5.0, 8.0],
            ])
        );
    }

    #[test]
    fn transposing_the_identity_matrix() {
        let a = Matrix::<4>::identity();
        assert_eq!(a.transpose(), a);
    }

    #[test]
    fn calculating_the_determinant_of_a_2x2_matrix() {
        let a = Matrix::new([[1.0, 5.0], [-3.0, 2.0]]);
        assert_eq!(a.determinant(), 17.0);
    }

    #[test]
    fn a_submatrix_of_a_3x3_matrix_is_a_2x2_matrix() {
        let a = Matrix::new([[1.0, 5.0, 0.0], [-3.0, 2.0, 7.0], [0.0, 6.0, -3.0]]);
        assert_eq!(a.submatrix(0, 2), Matrix::new([[-3.0, 2.0], [0.0, 6.0]]));
    }

    #[test]
    fn a_submatrix_of_a_4x4_matrix_is_a_3x3_matrix() {
        let a = Matrix::new([
            [-6.0, 1.0, 1.0, 6.0],
            [-8.0, 5.0, 8.0, 6.0],
            [-1.0, 0.0, 8.0, 2.0],
            [-7.0, 1.0, -1.0, 1.0],
        ]);
        assert_eq!(
            a.submatrix(2, 1),
            Matrix::new([[-6.0, 1.0, 6.0], [-8.0, 8.0, 6.0], [-7.0, -1.0, 1.0]])
        );
    }

    #[test]
    fn calculating_a_minor_of_a_3x3_matrix() {
        let a = Matrix::new([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);
        assert_eq!(a.submatrix(1, 0).determinant(), 25.0);
        assert_eq!(a.minor(1, 0), 25.0);
    }

    #[test]
    fn calculating_a_cofactor_of_a_3x3_matrix() {
        let a = Matrix::new([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);
        assert_eq!(a.minor(0, 0), -12.0);
        assert_eq!(a.cofactor(0, 0), -12.0);
        assert_eq!(a.minor(1, 0), 25.0);
        assert_eq!(a.cofactor(1, 0), -25.0);
    }

    #[test]
    fn calculating_the_determinant_of_a_3x3_matrix() {
        let a = Matrix::new([[1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0]]);
        assert_eq!(a.cofactor(0, 0), 56.0);
        assert_eq!(a.cofactor(0, 1), 12.0);
        assert_eq!(a.cofactor(0, 2), -46.0);
        assert_eq!(a.determinant(), -196.0);
    }

    #[test]
    fn calculating_the_determinant_of_a_4x4_matrix() {
        let a = Matrix::new([
            [-2.0, -8.0, 3.0, 5.0],
            [-3.0, 1.0, 7.0, 3.0],
            [1.0, 2.0, -9.0, 6.0],
            [-6.0, 7.0, 7.0, -9.0],
        ]);
        assert_eq!(a.cofactor(0, 0), 690.0);
        assert_eq!(a.cofactor(0, 1), 447.0);
        assert_eq!(a.cofactor(0, 2), 210.0);
        assert_eq!(a.cofactor(0, 3), 51.0);
        assert_eq!(a.determinant(), -4071.0);
    }

    #[test]
    fn invertible_matrix_is_invertible() {
        let a = Matrix::new([
            [6.0, 4.0, 4.0, 4.0],
            [5.0, 5.0, 7.0, 6.0],
            [4.0, -9.0, 3.0, -7.0],
            [9.0, 1.0, 7.0, -6.0],
        ]);
        assert_eq!(a.determinant(), -2120.0);
        assert_eq!(a.is_invertible(), true);
    }

    #[test]
    fn noninvertible_matrix_is_not_invertible() {
        let a = Matrix::new([
            [-4.0, 2.0, -2.0, -3.0],
            [9.0, 6.0, 2.0, 6.0],
            [0.0, -5.0, 1.0, -5.0],
            [0.0, 0.0, 0.0, 0.0],
        ]);
        assert_eq!(a.determinant(), 0.0);
        assert_eq!(a.is_invertible(), false);
    }

    //  Scenario: Calculating the inverse of a matrix
    #[test]
    fn calculating_the_inverse_of_a_matrix() {
        let a = Matrix::new([
            [-5.0, 2.0, 6.0, -8.0],
            [1.0, -5.0, 1.0, 8.0],
            [7.0, 7.0, -6.0, -7.0],
            [1.0, -3.0, 7.0, 4.0],
        ]);
        let b = a.inverse();

        assert_eq!(a.determinant(), 532.0);
        assert_eq!(a.cofactor(2, 3), -160.0);
        assert_eq!(b[(3, 2)], -160.0 / 532.0);
        assert_eq!(a.cofactor(3, 2), 105.0);
        assert_eq!(b[(2, 3)], 105.0 / 532.0);
        assert_eq!(
            b,
            Matrix::new([
                [0.21804512, 0.45112783, 0.24060151, -0.04511278],
                [-0.8082707, -1.456767, -0.44360903, 0.5206767],
                [-0.078947365, -0.2236842, -0.05263158, 0.19736843],
                [-0.52255636, -0.81390977, -0.30075186, 0.30639],
            ])
        );
    }

    #[test]
    fn calculating_the_inverse_of_a_second_matrix() {
        let a = Matrix::new([
            [8.0, -5.0, 9.0, 2.0],
            [7.0, 5.0, 6.0, 1.0],
            [-6.0, 0.0, 9.0, 6.0],
            [-3.0, 0.0, -9.0, -4.0],
        ]);
        let b = a.inverse();
        assert_eq!(
            b,
            Matrix::new([
                [-0.15385, -0.15385, -0.28205, -0.53846],
                [-0.07692, 0.12308, 0.02564, 0.03077],
                [0.35897, 0.35897, 0.43590, 0.92308],
                [-0.69231, -0.69231, -0.76923, -1.92308],
            ])
        );
    }

    #[test]
    fn calculating_the_inverse_of_a_third_matrix() {
        let a = Matrix::new([
            [9.0, 3.0, 0.0, 9.0],
            [-5.0, -2.0, -6.0, -3.0],
            [-4.0, 9.0, 6.0, 4.0],
            [-7.0, 6.0, 6.0, 2.0],
        ]);
        let b = a.inverse();
        assert_eq!(
            b,
            Matrix::new([
                [-0.04074, -0.07778, 0.14444, -0.22222],
                [-0.07778, 0.03333, 0.36667, -0.33333],
                [-0.02901, -0.14630, -0.10926, 0.12963],
                [0.17778, 0.06667, -0.26667, 0.33333],
            ])
        );
    }

    #[test]
    fn multiplying_a_product_by_its_inverse() {
        let a = Matrix::new([
            [3.0, -9.0, 7.0, 3.0],
            [3.0, -8.0, 2.0, -9.0],
            [-4.0, 4.0, 4.0, 1.0],
            [-6.0, 5.0, -1.0, 1.0],
        ]);
        let b = Matrix::new([
            [8.0, 2.0, 2.0, 2.0],
            [3.0, -1.0, 7.0, 0.0],
            [7.0, 0.0, 5.0, 4.0],
            [6.0, -2.0, 0.0, 5.0],
        ]);
        let c = a * b;
        assert_eq!(c * b.inverse(), a);
    }
}
