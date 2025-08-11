use pofk_algorithms::matrix_algorithms::matrix_rotation::*;

#[test]
fn test_matrix_rotation() {
    let mut mat = vec![vec![1,2,3], vec![4,5,6], vec![7,8,9]];
    rotate_matrix(&mut mat);
    assert_eq!(mat, vec![vec![7,4,1], vec![8,5,2], vec![9,6,3]]);
}
