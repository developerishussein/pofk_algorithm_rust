//! Word Search in Grid (DFS, Production-Grade)
//!
//! Checks if a word exists in a 2D grid of characters.
//!
//! # Example
//! ```rust
//! use pofk_algorithm::matrix_algorithms::word_search::*;
//! let board = vec![vec!['A','B','C','E'], vec!['S','F','C','S'], vec!['A','D','E','E']];
//! assert!(word_search(&board, "ABCCED"));
//! assert!(!word_search(&board, "ABCB"));
//! ```
pub fn word_search(board: &[Vec<char>], word: &str) -> bool {
    let rows = board.len();
    let cols = if rows > 0 { board[0].len() } else { 0 };
    let word: Vec<char> = word.chars().collect();
    let mut visited = vec![vec![false; cols]; rows];
    fn dfs(board: &[Vec<char>], word: &[char], visited: &mut Vec<Vec<bool>>, i: usize, r: usize, c: usize) -> bool {
        if i == word.len() { return true; }
        if r >= board.len() || c >= board[0].len() || visited[r][c] || board[r][c] != word[i] { return false; }
        visited[r][c] = true;
        let dirs = [(-1,0),(1,0),(0,-1),(0,1)];
        for (dr, dc) in dirs.iter() {
            let nr = r as isize + dr;
            let nc = c as isize + dc;
            if nr >= 0 && nr < board.len() as isize && nc >= 0 && nc < board[0].len() as isize {
                if dfs(board, word, visited, i+1, nr as usize, nc as usize) {
                    visited[r][c] = false;
                    return true;
                }
            }
        }
        visited[r][c] = false;
        false
    }
    for r in 0..rows {
        for c in 0..cols {
            if dfs(board, &word, &mut visited, 0, r, c) {
                return true;
            }
        }
    }
    false
}
