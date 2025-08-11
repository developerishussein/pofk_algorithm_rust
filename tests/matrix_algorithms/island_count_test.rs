use pofk_algorithm::matrix_algorithms::island_count::*;

#[test]
fn test_island_count_basic() {
    let grid = vec![vec![1,1,0,0,0], vec![1,1,0,0,0], vec![0,0,1,0,0], vec![0,0,0,1,1]];
    assert_eq!(island_count(&grid), 3);
}

#[test]
fn test_island_count_empty() {
    let grid: Vec<Vec<u8>> = vec![];
    assert_eq!(island_count(&grid), 0);
}
