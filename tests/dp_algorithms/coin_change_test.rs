use pofk_algorithm::dp_algorithms::coin_change::coin_change;

#[test]
fn test_coin_change_large() {
    let coins = vec![1, 2, 5, 10, 20, 50];
    let amount = 1000;
    let result = coin_change(&coins, amount);
    assert!(result > 0);
    // Impossible case
    let coins = vec![7, 13];
    let amount = 5;
    assert_eq!(coin_change(&coins, amount), -1);
}
