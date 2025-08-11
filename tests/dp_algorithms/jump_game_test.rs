use pofk_algorithms::dp_algorithms::jump_game::jump_game;

#[test]
fn test_jump_game_large() {
    let mut nums = vec![100; 1000];
    nums[999] = 0;
    assert!(jump_game(&nums));
    // Negative case
    let mut nums = vec![1; 1000];
    nums[999] = 0;
    nums[500] = 0;
    assert!(!jump_game(&nums));
}
