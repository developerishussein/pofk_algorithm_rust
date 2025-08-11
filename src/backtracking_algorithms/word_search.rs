//! Word Search (Production-Grade)
//!
//! Determines if a word exists in a 2D board.
//!
//! # Example
//! ```rust
//! use pofk_algorithms::backtracking_algorithms::word_search::*;
//! let board = vec![vec!['A','B','C','E'], vec!['S','F','C','S'], vec!['A','D','E','E']];
//! assert!(word_search(&board, "ABCCED"));
//! ```
pub fn word_search(board: &[Vec<char>], word: &str) -> bool {
    let (m, n) = (board.len(), board[0].len());
    let word: Vec<char> = word.chars().collect();
    let mut visited = vec![vec![false; n]; m];
    fn dfs(board: &[Vec<char>], word: &[char], i: usize, j: usize, k: usize, visited: &mut [Vec<bool>]) -> bool {
        if k == word.len() { return true; }
        if i >= board.len() || j >= board[0].len() || visited[i][j] || board[i][j] != word[k] {
            return false;
        }
        visited[i][j] = true;
        let dirs = [(0,1),(1,0),(0,-1),(-1,0)];
        for &(di, dj) in &dirs {
            let ni = i as isize + di;
            let nj = j as isize + dj;
            if ni >= 0 && nj >= 0 && (ni as usize) < board.len() && (nj as usize) < board[0].len() {
                if dfs(board, word, ni as usize, nj as usize, k+1, visited) {
                    return true;
                }
            }
        }
        visited[i][j] = false;
        false
    }
    for i in 0..m {
        for j in 0..n {
            if dfs(board, &word, i, j, 0, &mut visited) {
                return true;
            }
        }
    }
    false
}
