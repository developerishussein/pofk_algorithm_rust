use pofk_algorithm::backtracking_algorithms::n_queens::n_queens;

#[test]
fn test_n_queens_large() {
    let solutions = n_queens(8usize);
    assert_eq!(solutions.len(), 92);
    let solutions = n_queens(4usize);
    assert_eq!(solutions.len(), 2);
}
