use pofk_algorithm::matrix_algorithms::spiral_traversal::*;

#[test]
fn test_spiral_traversal() {
    let mat = vec![vec![1,2,3], vec![4,5,6], vec![7,8,9]];
    assert_eq!(spiral_traversal(&mat), vec![1,2,3,6,9,8,7,4,5]);
}
