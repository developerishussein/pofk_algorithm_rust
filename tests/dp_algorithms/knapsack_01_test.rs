use pofk_algorithm::dp_algorithms::knapsack_01::knapsack_01;

#[test]
fn test_knapsack_01_large() {
    let weights = vec![2, 3, 4, 5, 9, 7, 3, 6, 8, 5, 4, 7, 2, 1, 6, 8, 9, 3, 2, 5];
    let values = vec![3, 4, 5, 6, 10, 8, 4, 7, 9, 6, 5, 8, 3, 2, 7, 9, 10, 4, 3, 6];
    let capacity = 50;
    let result = knapsack_01(&weights, &values, capacity);
    assert!(result > 0);
    // Small case
    assert_eq!(knapsack_01(&[2, 3, 4, 5], &[3, 4, 5, 6], 5), 7);
}
