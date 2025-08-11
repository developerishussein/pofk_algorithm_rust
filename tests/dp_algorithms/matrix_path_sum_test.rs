use pofk_algorithms::dp_algorithms::matrix_path_sum::matrix_path_sum;

#[test]
fn test_matrix_path_sum_large() {
    let matrix: Vec<Vec<u32>> = vec![vec![1; 100]; 100];
    assert_eq!(matrix_path_sum(&matrix), 199);
    let matrix = vec![vec![1,3,1], vec![1,5,1], vec![4,2,1]];
    assert_eq!(matrix_path_sum(&matrix), 7);
}
