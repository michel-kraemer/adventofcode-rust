//! Gaussian elimination to solve a system of linear equations. The code is
//! based on the JavaScript implementation found in the Arcane Algorithm Archive
//! (see <https://www.algorithm-archive.org/> and
//! <https://github.com/algorithm-archivists/algorithm-archive)>, licensed under
//! the MIT license by James Schloss et. al.

use std::mem;

fn gaussian_elimination<const R: usize, const C: usize>(matrix: &mut [[f64; C]; R]) {
    for i in 0..C - 1 {
        // find largest element in current column
        let mut pivot = i;
        for j in i + 1..R {
            if matrix[j][i].abs() > matrix[pivot][i] {
                pivot = j;
            }
        }

        if matrix[pivot][i] == 0.0 {
            panic!("The matrix is singular");
        }

        if i != pivot {
            // swap rows
            for k in i..C {
                let (a, b) = matrix.split_at_mut(pivot);
                mem::swap(&mut a[i][k], &mut b[0][k]);
            }
        }

        // set elements below pivot to 0
        for j in i + 1..R {
            let scale = matrix[j][i] / matrix[i][i];
            for k in i + 1..C {
                matrix[j][k] -= matrix[i][k] * scale;
            }
            matrix[j][i] = 0.0;
        }
    }
}

fn gauss_jordan_elimination<const R: usize, const C: usize>(matrix: &mut [[f64; C]; R]) {
    for i in 0..C - 1 {
        if matrix[i][i] != 0.0 {
            for j in (i..C).rev() {
                matrix[i][j] /= matrix[i][i];
            }

            for j in 0..i {
                for k in (i..C).rev() {
                    matrix[j][k] -= matrix[j][i] * matrix[i][k];
                }
            }
        }
    }
}

fn back_substitution<const R: usize, const C: usize>(matrix: &mut [[f64; C]; R]) -> [f64; R] {
    let mut result = [0.0; R];
    for i in (0..R).rev() {
        let mut sum = 0.0;
        for (j, c) in result.iter().enumerate().take(C - 1).skip(i + 1) {
            sum += c * matrix[i][j];
        }
        result[i] = (matrix[i][C - 1] - sum) / matrix[i][i];
    }
    result
}

/// Solve a system of equations
pub fn solve<const R: usize, const C: usize>(matrix: &mut [[f64; C]; R]) -> [f64; R] {
    gaussian_elimination(matrix);
    gauss_jordan_elimination(matrix);
    back_substitution(matrix)
}
