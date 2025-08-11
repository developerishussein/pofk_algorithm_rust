//! Shortest Path in Grid (BFS, Production-Grade)
//!
//! Finds the shortest path from (sr, sc) to (er, ec) in a 2D grid of 0s (open) and 1s (blocked).
//!
//! # Example
//! ```rust
//! use pofk_algorithms::matrix_algorithms::shortest_path_grid::*;
//! let grid = vec![vec![0,0,0], vec![1,1,0], vec![0,0,0]];
//! assert_eq!(shortest_path_grid(&grid, (0,0), (2,2)), Some(4));
//! ```
use std::collections::VecDeque;
pub fn shortest_path_grid(grid: &[Vec<u8>], start: (usize, usize), end: (usize, usize)) -> Option<usize> {
    let rows = grid.len();
    let cols = if rows > 0 { grid[0].len() } else { 0 };
    let mut visited = vec![vec![false; cols]; rows];
    let mut queue = VecDeque::new();
    queue.push_back((start.0, start.1, 0));
    visited[start.0][start.1] = true;
    let dirs = [(-1,0),(1,0),(0,-1),(0,1)];
    while let Some((r, c, d)) = queue.pop_front() {
        if (r, c) == end { return Some(d); }
        for (dr, dc) in dirs.iter() {
            let nr = r as isize + dr;
            let nc = c as isize + dc;
            if nr >= 0 && nr < rows as isize && nc >= 0 && nc < cols as isize {
                let (nr, nc) = (nr as usize, nc as usize);
                if !visited[nr][nc] && grid[nr][nc] == 0 {
                    visited[nr][nc] = true;
                    queue.push_back((nr, nc, d+1));
                }
            }
        }
    }
    None
}
