//! Sudoku Solver (Production-Grade)
//!
//! Solves a given 9x9 Sudoku puzzle in-place.
//!
//! # Example
//! ```rust
//! use pofk_algorithms::backtracking_algorithms::sudoku_solver::*;
//! let mut board = [
//!     ['5','3','.','.','7','.','.','.','.'],
//!     ['6','.','.','1','9','5','.','.','.'],
//!     ['.','9','8','.','.','.','.','6','.'],
//!     ['8','.','.','.','6','.','.','.','3'],
//!     ['4','.','.','8','.','3','.','.','1'],
//!     ['7','.','.','.','2','.','.','.','6'],
//!     ['.','6','.','.','.','.','2','8','.'],
//!     ['.','.','.','4','1','9','.','.','5'],
//!     ['.','.','.','.','8','.','.','7','9']
//! ];
//! solve_sudoku(&mut board);
//! assert_eq!(board[0][0], '5');
//! ```
pub fn solve_sudoku(board: &mut [[char; 9]; 9]) -> bool {
    fn is_valid(board: &[[char; 9]; 9], row: usize, col: usize, c: char) -> bool {
        for i in 0..9 {
            if board[row][i] == c || board[i][col] == c {
                return false;
            }
            let box_row = 3 * (row / 3) + i / 3;
            let box_col = 3 * (col / 3) + i % 3;
            if board[box_row][box_col] == c {
                return false;
            }
        }
        true
    }
    for row in 0..9 {
        for col in 0..9 {
            if board[row][col] == '.' {
                for c in '1'..='9' {
                    if is_valid(board, row, col, c) {
                        board[row][col] = c;
                        if solve_sudoku(board) {
                            return true;
                        }
                        board[row][col] = '.';
                    }
                }
                return false;
            }
        }
    }
    true
}
