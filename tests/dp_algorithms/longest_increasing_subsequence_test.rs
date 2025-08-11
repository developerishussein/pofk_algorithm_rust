use pofk_algorithms::dp_algorithms::longest_increasing_subsequence::longest_increasing_subsequence;

#[test]
fn test_lis_large() {
    let arr: Vec<u32> = (1..=1000).collect();
    assert_eq!(longest_increasing_subsequence(&arr), 1000);
    let arr = vec![10, 22, 9, 33, 21, 50, 41, 60, 80];
    assert_eq!(longest_increasing_subsequence(&arr), 6);
}
