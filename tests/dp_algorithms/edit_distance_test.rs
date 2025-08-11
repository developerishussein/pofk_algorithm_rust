use pofk_algorithm::dp_algorithms::edit_distance::edit_distance;

#[test]
fn test_edit_distance_large() {
    let a: Vec<char> = "kitten".chars().collect();
    let b: Vec<char> = "sitting".chars().collect();
    assert_eq!(edit_distance(&a, &b), 3);
    let a: Vec<char> = "a".repeat(100).chars().collect();
    let b: Vec<char> = "a".repeat(99).chars().chain("b".chars()).collect();
    assert_eq!(edit_distance(&a, &b), 1);
}
