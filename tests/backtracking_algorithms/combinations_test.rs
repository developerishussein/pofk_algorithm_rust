use pofk_algorithms::backtracking_algorithms::combinations::combinations;

#[test]
fn test_combinations_large() {
    let nums = vec![1, 2, 3, 4, 5];
    let combs = combinations(&nums, 3);
    assert_eq!(combs.len(), 10);
    let nums = vec![1, 2, 3, 4];
    let combs = combinations(&nums, 2);
    assert_eq!(combs.len(), 6);
}
