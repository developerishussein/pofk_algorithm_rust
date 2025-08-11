//! Spiral Traversal of Matrix (Production-Grade)
//!
//! Returns the elements of a matrix in spiral order.
//!
//! # Example
//! ```rust
//! use pofk_algorithm::matrix_algorithms::spiral_traversal::*;
//! let mat = vec![vec![1,2,3], vec![4,5,6], vec![7,8,9]];
//! assert_eq!(spiral_traversal(&mat), vec![1,2,3,6,9,8,7,4,5]);
//! ```
pub fn spiral_traversal<T: Clone>(matrix: &[Vec<T>]) -> Vec<T> {
    let mut res = Vec::new();
    if matrix.is_empty() { return res; }
    let (mut top, mut bottom, mut left, mut right) = (0, matrix.len()-1, 0, matrix[0].len()-1);
    while top <= bottom && left <= right {
        for c in left..=right { res.push(matrix[top][c].clone()); }
        if top == bottom { break; }
        for r in top+1..=bottom { res.push(matrix[r][right].clone()); }
        if left == right { break; }
        for c in (left..right).rev() { res.push(matrix[bottom][c].clone()); }
        for r in (top+1..bottom).rev() { res.push(matrix[r][left].clone()); }
        top += 1; bottom -= 1; left += 1; right -= 1;
    }
    res
}
