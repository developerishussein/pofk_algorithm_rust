//! Flood Fill (Generic, Production-Grade)
//!
//! Performs a flood fill on a 2D grid.
//!
//! # Type Parameters
//! * `T`: Value type. Must implement `Clone` + `PartialEq`.
//!
//! # Example
//! ```rust
//! use pofk_algorithms::matrix_algorithms::flood_fill::*;
//! let mut grid = vec![vec![1,1,1], vec![1,1,0], vec![1,0,1]];
//! flood_fill(&mut grid, 1, 1, 2);
//! assert_eq!(grid[1][1], 2);
//! ```
pub fn flood_fill<T: Clone + PartialEq>(grid: &mut Vec<Vec<T>>, sr: usize, sc: usize, new_val: T) {
    let rows = grid.len();
    let cols = if rows > 0 { grid[0].len() } else { 0 };
    let orig = grid[sr][sc].clone();
    if orig == new_val { return; }
    fn dfs<T: Clone + PartialEq>(grid: &mut Vec<Vec<T>>, r: usize, c: usize, orig: &T, new_val: &T, rows: usize, cols: usize) {
        if r >= rows || c >= cols || grid[r][c] != *orig { return; }
        grid[r][c] = new_val.clone();
        if r > 0 { dfs(grid, r-1, c, orig, new_val, rows, cols); }
        if c > 0 { dfs(grid, r, c-1, orig, new_val, rows, cols); }
        if r+1 < rows { dfs(grid, r+1, c, orig, new_val, rows, cols); }
        if c+1 < cols { dfs(grid, r, c+1, orig, new_val, rows, cols); }
    }
    dfs(grid, sr, sc, &orig, &new_val, rows, cols);
}
