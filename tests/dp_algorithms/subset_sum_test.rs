use pofk_algorithm::dp_algorithms::subset_sum::subset_sum;

#[test]
fn test_subset_sum_large() {
    let set: Vec<u32> = (1..=30).collect();
    let sum = 465; // sum of 1..=30
    assert!(subset_sum(&set, sum));
    // Negative case
    assert!(!subset_sum(&set, 1000));
}
