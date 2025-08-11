//! Matrix Rotation (90 degrees, Production-Grade)
//!
//! Rotates a square matrix 90 degrees clockwise in-place.
//!
//! # Example
//! ```rust
//! use pofk_algorithm::matrix_algorithms::matrix_rotation::*;
//! let mut mat = vec![vec![1,2,3], vec![4,5,6], vec![7,8,9]];
//! rotate_matrix(&mut mat);
//! assert_eq!(mat, vec![vec![7,4,1], vec![8,5,2], vec![9,6,3]]);
//! ```
pub fn rotate_matrix<T: Clone>(matrix: &mut Vec<Vec<T>>) {
    let n = matrix.len();
    for layer in 0..n/2 {
        let first = layer;
        let last = n - 1 - layer;
        for i in first..last {
            let offset = i - first;
            let top = matrix[first][i].clone();
            matrix[first][i] = matrix[last-offset][first].clone();
            matrix[last-offset][first] = matrix[last][last-offset].clone();
            matrix[last][last-offset] = matrix[i][last].clone();
            matrix[i][last] = top;
        }
    }
}
