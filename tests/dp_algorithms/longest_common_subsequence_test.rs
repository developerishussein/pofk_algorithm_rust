use pofk_algorithms::dp_algorithms::longest_common_subsequence::longest_common_subsequence;

#[test]
fn test_lcs_large() {
    let a: Vec<u8> = (0..100).collect();
    let b: Vec<u8> = (50..150).collect();
    assert_eq!(longest_common_subsequence(&a, &b), 50);
    let a = b"AGGTAB".to_vec();
    let b = b"GXTXAYB".to_vec();
    assert_eq!(longest_common_subsequence(&a, &b), 4);
}
