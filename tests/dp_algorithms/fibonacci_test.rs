use pofk_algorithm::dp_algorithms::fibonacci::fibonacci;

#[test]
fn test_fibonacci_large() {
    // Test several large Fibonacci numbers
    assert_eq!(fibonacci::<u64>(40), 102334155);
    assert_eq!(fibonacci::<u64>(50), 12586269025);
    assert_eq!(fibonacci::<u64>(60), 1548008755920);
    // Test base cases
    assert_eq!(fibonacci::<u64>(0), 0);
    assert_eq!(fibonacci::<u64>(1), 1);
}
