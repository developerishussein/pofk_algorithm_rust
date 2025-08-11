use pofk_algorithms::dp_algorithms::rod_cutting::rod_cutting;

#[test]
fn test_rod_cutting_large() {
    let prices: Vec<u32> = (1..=100).collect();
    let n = 100;
    let result = rod_cutting(&prices, n);
    assert!(result > 0);
    let prices = vec![1, 5, 8, 9, 10, 17, 17, 20];
    let n = 8;
    assert_eq!(rod_cutting(&prices, n), 22);
}
