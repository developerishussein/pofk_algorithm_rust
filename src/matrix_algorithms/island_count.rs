//! Island Count (DFS/BFS, Production-Grade)
//!
//! Counts the number of islands (connected regions of 1s) in a 2D grid.
//!
//! # Example
//! ```rust
//! use pofk_algorithm::matrix_algorithms::island_count::*;
//! let grid = vec![vec![1,1,0,0,0], vec![1,1,0,0,0], vec![0,0,1,0,0], vec![0,0,0,1,1]];
//! assert_eq!(island_count(&grid), 3);
//! ```
pub fn island_count(grid: &[Vec<u8>]) -> usize {
    let mut grid = grid.to_vec();
    let rows = grid.len();
    let cols = if rows > 0 { grid[0].len() } else { 0 };
    fn dfs(grid: &mut Vec<Vec<u8>>, r: usize, c: usize, rows: usize, cols: usize) {
        if r >= rows || c >= cols || grid[r][c] != 1 { return; }
        grid[r][c] = 0;
        if r > 0 { dfs(grid, r-1, c, rows, cols); }
        if c > 0 { dfs(grid, r, c-1, rows, cols); }
        if r+1 < rows { dfs(grid, r+1, c, rows, cols); }
        if c+1 < cols { dfs(grid, r, c+1, rows, cols); }
    }
    let mut count = 0;
    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == 1 {
                count += 1;
                dfs(&mut grid, r, c, rows, cols);
            }
        }
    }
    count
}
