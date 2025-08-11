use pofk_algorithms::dp_algorithms::partition_equal_subset_sum::partition_equal_subset_sum;

#[test]
fn test_partition_equal_subset_sum_large() {
    let nums = vec![1, 5, 11, 5, 1, 5, 11, 5, 1, 5, 11, 5];
    assert!(partition_equal_subset_sum(&nums));
    // Negative case
    let nums = vec![1, 2, 3, 5];
    assert!(!partition_equal_subset_sum(&nums));
}
