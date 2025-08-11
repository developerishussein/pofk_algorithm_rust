use pofk_algorithm::matrix_algorithms::word_search::*;

#[test]
fn test_word_search_found() {
    let board = vec![vec!['A','B','C','E'], vec!['S','F','C','S'], vec!['A','D','E','E']];
    assert!(word_search(&board, "ABCCED"));
}

#[test]
fn test_word_search_not_found() {
    let board = vec![vec!['A','B','C','E'], vec!['S','F','C','S'], vec!['A','D','E','E']];
    assert!(!word_search(&board, "ABCB"));
}
