//! N-Queens Problem (Generic, Production-Grade)
//!
//! Solves the N-Queens problem and returns all distinct solutions.
//!
//! # Type Parameters
//! * `T`: Integer type. Must implement `Copy` + `PartialEq` + `From<u8>` + `Add<Output = T>` + `Sub<Output = T>`.
//!
//! # Example
//! ```rust
//! use pofk_algorithm::backtracking_algorithms::n_queens::*;
//! let solutions = n_queens(4usize);
//! assert_eq!(solutions.len(), 2);
//! ```
use std::ops::{Add, Sub};

pub fn n_queens<T>(n: T) -> Vec<Vec<T>>
where
    T: Copy + PartialEq + From<u8> + Add<Output = T> + Sub<Output = T> + Into<usize>,
{
    let n_usize = n.into();
    let mut results = Vec::new();
    let mut board = vec![T::from(0u8); n_usize];
    fn is_valid<T>(board: &[T], row: usize, col: T) -> bool
    where
        T: Copy + PartialEq + Add<Output = T> + Sub<Output = T> + From<u8> + Into<usize>,
    {
        for i in 0..row {
            if board[i] == col
                || (board[i].into() as isize - col.into() as isize).abs() == (row as isize - i as isize).abs()
            {
                return false;
            }
        }
        true
    }
    fn solve<T>(row: usize, n: usize, board: &mut Vec<T>, results: &mut Vec<Vec<T>>)
    where
        T: Copy + PartialEq + Add<Output = T> + Sub<Output = T> + From<u8> + Into<usize>,
    {
        if row == n {
            results.push(board.clone());
            return;
        }
        for col in 0..n {
            let col_t = T::from(col as u8);
            if is_valid(board, row, col_t) {
                board[row] = col_t;
                solve(row + 1, n, board, results);
            }
        }
    }
    solve(0, n_usize, &mut board, &mut results);
    results
}
