//! Path Sum in Matrix (DFS, Production-Grade)
//!
//! Checks if there is a path from (sr, sc) to (er, ec) with a given sum.
//!
//! # Example
//! ```rust
//! use pofk_algorithms::matrix_algorithms::path_sum::*;
//! let grid = vec![vec![5,4,8], vec![11,13,4], vec![7,2,1]];
//! assert!(path_sum(&grid, (0,0), (2,2), 27));
//! assert!(!path_sum(&grid, (0,0), (2,2), 10));
//! ```
pub fn path_sum(grid: &[Vec<i32>], start: (usize, usize), end: (usize, usize), sum: i32) -> bool {
    let rows = grid.len();
    let cols = if rows > 0 { grid[0].len() } else { 0 };
    fn dfs(grid: &[Vec<i32>], r: usize, c: usize, end: (usize, usize), sum: i32, acc: i32, rows: usize, cols: usize, visited: &mut Vec<Vec<bool>>) -> bool {
        if r >= rows || c >= cols || visited[r][c] { return false; }
        let acc = acc + grid[r][c];
        if (r, c) == end { return acc == sum; }
        visited[r][c] = true;
        let dirs = [(-1,0),(1,0),(0,-1),(0,1)];
        for (dr, dc) in dirs.iter() {
            let nr = r as isize + dr;
            let nc = c as isize + dc;
            if nr >= 0 && nr < rows as isize && nc >= 0 && nc < cols as isize {
                if dfs(grid, nr as usize, nc as usize, end, sum, acc, rows, cols, visited) {
                    visited[r][c] = false;
                    return true;
                }
            }
        }
        visited[r][c] = false;
        false
    }
    let mut visited = vec![vec![false; cols]; rows];
    dfs(grid, start.0, start.1, end, sum, 0, rows, cols, &mut visited)
}
