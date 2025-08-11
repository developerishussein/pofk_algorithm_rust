use pofk_algorithm::backtracking_algorithms::combination_sum::combination_sum;

#[test]
fn test_combination_sum_large() {
    let candidates = vec![2,3,6,7];
    let target = 7;
    let result = combination_sum(&candidates, target);
    assert!(result.contains(&vec![2,2,3]));
    assert!(result.contains(&vec![7]));
    let candidates = vec![2,3,5];
    let target = 8;
    let result = combination_sum(&candidates, target);
    assert!(result.contains(&vec![2,2,2,2]));
}
