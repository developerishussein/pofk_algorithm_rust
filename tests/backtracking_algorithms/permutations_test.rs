use pofk_algorithms::backtracking_algorithms::permutations::permutations;

#[test]
fn test_permutations_large() {
    let nums = vec![1, 2, 3, 4];
    let perms = permutations(&nums);
    assert_eq!(perms.len(), 24);
    let nums = vec![1, 2, 3];
    let perms = permutations(&nums);
    assert_eq!(perms.len(), 6);
}
