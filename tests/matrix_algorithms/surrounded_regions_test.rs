use pofk_algorithm::matrix_algorithms::surrounded_regions::*;

#[test]
fn test_surrounded_regions() {
    let mut board = vec![vec!['X','X','X','X'], vec!['X','O','O','X'], vec!['X','X','O','X'], vec!['X','O','X','X']];
    solve_surrounded_regions(&mut board);
    assert_eq!(board, vec![vec!['X','X','X','X'], vec!['X','X','X','X'], vec![ 'X','X','X','X'], vec!['X','O','X','X']]);
}
