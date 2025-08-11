use pofk_algorithm::backtracking_algorithms::subset_generation::subset_generation;

#[test]
fn test_subset_generation_large() {
    let nums: Vec<u32> = (1..=5).collect();
    let subsets = subset_generation(&nums);
    assert_eq!(subsets.len(), 32);
    let nums = vec![1, 2, 3];
    let subsets = subset_generation(&nums);
    assert_eq!(subsets.len(), 8);
}
