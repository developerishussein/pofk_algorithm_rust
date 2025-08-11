use pofk_algorithms::dp_algorithms::house_robber::house_robber;

#[test]
fn test_house_robber_large() {
    let nums: Vec<u32> = (1..=1000).collect();
    let result = house_robber(&nums);
    assert!(result > 0);
    let nums = vec![2,7,9,3,1];
    assert_eq!(house_robber(&nums), 12);
}
