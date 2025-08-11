//! Rat in a Maze (Generic, Production-Grade)
//!
//! Finds all paths for a rat to reach the destination in a maze.
//!
//! # Type Parameters
//! * `T`: Value type. Must implement `Clone` + `PartialEq` + `From<u8>`.
//!
//! # Example
//! ```rust
//! use pofk_algorithms::backtracking_algorithms::rat_in_a_maze::*;
//! let maze = vec![vec![1,0,0,0], vec![1,1,0,1], vec![0,1,0,0], vec![1,1,1,1]];
//! let paths = rat_in_a_maze(&maze);
//! assert!(!paths.is_empty());
//! ```
pub fn rat_in_a_maze<T>(maze: &[Vec<T>]) -> Vec<String>
where
    T: Clone + PartialEq + From<u8>,
{
    let n = maze.len();
    let mut res = Vec::new();
    let mut path = String::new();
    let mut visited = vec![vec![false; n]; n];
    fn backtrack<T>(x: usize, y: usize, maze: &[Vec<T>], visited: &mut [Vec<bool>], path: &mut String, res: &mut Vec<String>)
    where
        T: Clone + PartialEq + From<u8>,
    {
        let n = maze.len();
        if x == n - 1 && y == n - 1 {
            res.push(path.clone());
            return;
        }
        let dirs = [(1,0,'D'), (0,-1,'L'), (0,1,'R'), (-1,0,'U')];
        for &(dx, dy, dir) in &dirs {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx >= 0 && ny >= 0 && (nx as usize) < n && (ny as usize) < n
                && maze[nx as usize][ny as usize] == T::from(1u8)
                && !visited[nx as usize][ny as usize] {
                visited[nx as usize][ny as usize] = true;
                path.push(dir);
                backtrack(nx as usize, ny as usize, maze, visited, path, res);
                path.pop();
                visited[nx as usize][ny as usize] = false;
            }
        }
    }
    if n == 0 || maze[0][0] != T::from(1u8) {
        return res;
    }
    visited[0][0] = true;
    backtrack(0, 0, maze, &mut visited, &mut path, &mut res);
    res
}
