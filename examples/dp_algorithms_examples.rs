//! Examples for all dp_algorithms
use pofk_algorithm::dp_algorithms::fibonacci::fibonacci;
use pofk_algorithm::dp_algorithms::knapsack_01::knapsack_01;
use pofk_algorithm::dp_algorithms::longest_increasing_subsequence::longest_increasing_subsequence;
use pofk_algorithm::dp_algorithms::longest_common_subsequence::longest_common_subsequence;
use pofk_algorithm::dp_algorithms::edit_distance::edit_distance;
use pofk_algorithm::dp_algorithms::matrix_path_sum::matrix_path_sum;
use pofk_algorithm::dp_algorithms::coin_change::coin_change;
use pofk_algorithm::dp_algorithms::subset_sum::subset_sum;
use pofk_algorithm::dp_algorithms::partition_equal_subset_sum::partition_equal_subset_sum;
use pofk_algorithm::dp_algorithms::house_robber::house_robber;
use pofk_algorithm::dp_algorithms::jump_game::jump_game;
use pofk_algorithm::dp_algorithms::palindromic_substrings::palindromic_substrings;
use pofk_algorithm::dp_algorithms::rod_cutting::rod_cutting;

fn main() {
    // Fibonacci
    println!("Fibonacci(20): {}", fibonacci::<usize>(20));

    // 0/1 Knapsack
    let weights = vec![2usize, 3, 4, 5];
    let values = vec![3usize, 4, 5, 6];
    let capacity = 5usize;
    println!("Knapsack 0/1: {}", knapsack_01(&weights, &values, capacity));

    // Longest Increasing Subsequence
    let arr = vec![10, 9, 2, 5, 3, 7, 101, 18];
    println!("LIS: {}", longest_increasing_subsequence(&arr));

    // Longest Common Subsequence
    let a = b"AGGTAB".to_vec();
    let b = b"GXTXAYB".to_vec();
    println!("LCS: {}", longest_common_subsequence(&a, &b));

    // Edit Distance
    let a: Vec<char> = "kitten".chars().collect();
    let b: Vec<char> = "sitting".chars().collect();
    println!("Edit Distance: {}", edit_distance(&a, &b));

    // Matrix Path Sum
    let matrix = vec![vec![1,3,1], vec![1,5,1], vec![4,2,1]];
    println!("Matrix Path Sum: {}", matrix_path_sum(&matrix));

    // Coin Change
    let coins = vec![1usize, 2, 5];
    let amount = 11usize;
    println!("Coin Change: {}", coin_change(&coins, amount));

    // Subset Sum
    let set = vec![3usize, 34, 4, 12, 5, 2];
    println!("Subset Sum (sum=9): {}", subset_sum(&set, 9));

    // Partition Equal Subset Sum
    let nums = vec![1usize, 5, 11, 5];
    println!("Partition Equal Subset Sum: {}", partition_equal_subset_sum(&nums));

    // House Robber
    let nums = vec![2,7,9,3,1];
    println!("House Robber: {}", house_robber(&nums));

    // Jump Game
    let nums = vec![2,3,1,1,4];
    println!("Jump Game: {}", jump_game(&nums));

    // Palindromic Substrings
    let s: Vec<char> = "aabaa".chars().collect();
    println!("Palindromic Substrings: {}", palindromic_substrings(&s));

    // Rod Cutting
    let prices = vec![1, 5, 8, 9, 10, 17, 17, 20];
    let n = 8;
    println!("Rod Cutting: {}", rod_cutting(&prices, n));
}
