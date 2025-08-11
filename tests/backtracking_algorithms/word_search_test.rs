use pofk_algorithms::backtracking_algorithms::word_search::word_search;

#[test]
fn test_word_search_large() {
    let board = vec![vec!['A','B','C','E'], vec!['S','F','C','S'], vec!['A','D','E','E']];
    assert!(word_search(&board, "ABCCED"));
    assert!(word_search(&board, "SEE"));
    assert!(!word_search(&board, "ABCB"));
}
