use pofk_algorithms::matrix_algorithms::path_sum::*;

#[test]
fn test_path_sum_true() {
    let grid = vec![vec![5,4,8], vec![11,13,4], vec![7,2,1]];
    assert!(path_sum(&grid, (0,0), (2,2), 27));
}

#[test]
fn test_path_sum_false() {
    let grid = vec![vec![5,4,8], vec![11,13,4], vec![7,2,1]];
    assert!(!path_sum(&grid, (0,0), (2,2), 10));
}
