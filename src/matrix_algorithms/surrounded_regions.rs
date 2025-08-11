//! Surrounded Regions (Production-Grade)
//!
//! Captures all regions surrounded by 'X' in a 2D board of 'X' and 'O'.
//!
//! # Example
//! ```rust
//! use pofk_algorithms::matrix_algorithms::surrounded_regions::*;
//! let mut board = vec![vec!['X','X','X','X'], vec!['X','O','O','X'], vec!['X','X','O','X'], vec!['X','O','X','X']];
//! solve_surrounded_regions(&mut board);
//! assert_eq!(board, vec![vec!['X','X','X','X'], vec!['X','X','X','X'], vec!['X','X','X','X'], vec!['X','O','X','X']]);
//! ```
pub fn solve_surrounded_regions(board: &mut Vec<Vec<char>>) {
    let rows = board.len();
    let cols = if rows > 0 { board[0].len() } else { 0 };
    fn dfs(board: &mut Vec<Vec<char>>, r: usize, c: usize, rows: usize, cols: usize) {
        if r >= rows || c >= cols || board[r][c] != 'O' { return; }
        board[r][c] = '#';
        if r > 0 { dfs(board, r-1, c, rows, cols); }
        if c > 0 { dfs(board, r, c-1, rows, cols); }
        if r+1 < rows { dfs(board, r+1, c, rows, cols); }
        if c+1 < cols { dfs(board, r, c+1, rows, cols); }
    }
    for r in 0..rows {
        if board[r][0] == 'O' { dfs(board, r, 0, rows, cols); }
        if board[r][cols-1] == 'O' { dfs(board, r, cols-1, rows, cols); }
    }
    for c in 0..cols {
        if board[0][c] == 'O' { dfs(board, 0, c, rows, cols); }
        if board[rows-1][c] == 'O' { dfs(board, rows-1, c, rows, cols); }
    }
    for r in 0..rows {
        for c in 0..cols {
            if board[r][c] == 'O' { board[r][c] = 'X'; }
            if board[r][c] == '#' { board[r][c] = 'O'; }
        }
    }
}
