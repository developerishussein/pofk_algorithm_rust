use pofk_algorithms::matrix_algorithms::flood_fill::*;

#[test]
fn test_flood_fill_basic() {
    let mut grid = vec![vec![1,1,1], vec![1,1,0], vec![1,0,1]];
    flood_fill(&mut grid, 1, 1, 2);
    assert_eq!(grid, vec![vec![2,2,2], vec![2,2,0], vec![2,0,1]]);
}

#[test]
fn test_flood_fill_noop() {
    let mut grid = vec![vec![0,0], vec![0,0]];
    flood_fill(&mut grid, 0, 0, 0);
    assert_eq!(grid, vec![vec![0,0], vec![0,0]]);
}
