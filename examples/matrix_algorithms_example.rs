//! Example usage of matrix/grid algorithms
use pofk_algorithms::matrix_algorithms::*;

fn main() {
    // Flood Fill
    let mut grid = vec![vec![1,1,1], vec![1,1,0], vec![1,0,1]];
    flood_fill::flood_fill(&mut grid, 1, 1, 2);
    println!("Flood fill: {:?}", grid);

    // Island Count
    let grid = vec![vec![1,1,0,0,0], vec![1,1,0,0,0], vec![0,0,1,0,0], vec![0,0,0,1,1]];
    let count = island_count::island_count(&grid);
    println!("Island count: {}", count);

    // Shortest Path in Grid
    let grid = vec![vec![0,0,0], vec![1,1,0], vec![0,0,0]];
    let sp = shortest_path_grid::shortest_path_grid(&grid, (0,0), (2,2));
    println!("Shortest path: {:?}", sp);

    // Word Search
    let board = vec![vec!['A','B','C','E'], vec!['S','F','C','S'], vec!['A','D','E','E']];
    let found = word_search::word_search(&board, "ABCCED");
    println!("Word search (ABCCED): {}", found);

    // Path Sum
    let grid = vec![vec![5,4,8], vec![11,13,4], vec![7,2,1]];
    let has_sum = path_sum::path_sum(&grid, (0,0), (2,2), 27);
    println!("Path sum 27 exists: {}", has_sum);

    // Matrix Rotation
    let mut mat = vec![vec![1,2,3], vec![4,5,6], vec![7,8,9]];
    matrix_rotation::rotate_matrix(&mut mat);
    println!("Rotated matrix: {:?}", mat);

    // Spiral Traversal
    let mat = vec![vec![1,2,3], vec![4,5,6], vec![7,8,9]];
    let spiral = spiral_traversal::spiral_traversal(&mat);
    println!("Spiral traversal: {:?}", spiral);

    // Surrounded Regions
    let mut board = vec![vec!['X','X','X','X'], vec!['X','O','O','X'], vec!['X','X','O','X'], vec!['X','O','X','X']];
    surrounded_regions::solve_surrounded_regions(&mut board);
    println!("Surrounded regions solved: {:?}", board);
}
