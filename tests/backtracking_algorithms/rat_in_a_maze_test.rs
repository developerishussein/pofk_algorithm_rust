use pofk_algorithms::backtracking_algorithms::rat_in_a_maze::rat_in_a_maze;

#[test]
fn test_rat_in_a_maze_large() {
    let maze = vec![vec![1,0,0,0], vec![1,1,0,1], vec![0,1,0,0], vec![1,1,1,1]];
    let paths = rat_in_a_maze(&maze);
    assert!(!paths.is_empty());
    let maze = vec![vec![1,0], vec![0,1]];
    let paths = rat_in_a_maze(&maze);
    assert!(paths.is_empty());
}
