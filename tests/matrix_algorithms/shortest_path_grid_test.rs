use pofk_algorithms::matrix_algorithms::shortest_path_grid::*;

#[test]
fn test_shortest_path_basic() {
    let grid = vec![vec![0,0,0], vec![1,1,0], vec![0,0,0]];
    assert_eq!(shortest_path_grid(&grid, (0,0), (2,2)), Some(4));
}

#[test]
fn test_shortest_path_blocked() {
    let grid = vec![vec![0,1], vec![1,0]];
    assert_eq!(shortest_path_grid(&grid, (0,0), (1,1)), None);
}
