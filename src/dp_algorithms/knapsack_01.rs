//! 0/1 Knapsack Problem (Generic, Production-Grade)
//!
//! Solves the 0/1 Knapsack problem using dynamic programming.
//!
//! # Type Parameters
//! * `W`: Weight type. Must implement `Copy`, `PartialOrd`, `Sub<Output = W>`, `From<u8>`.
//! * `V`: Value type. Must implement `Copy`, `Add<Output = V>`, `Ord`, `Default`.
//!
//! # Example
//! ```rust
//! use pofk_algorithm::dp_algorithms::knapsack_01::*;
//! let weights = vec![2usize, 3, 4, 5];
//! let values = vec![3usize, 4, 5, 6];
//! let capacity = 5usize;
//! assert_eq!(knapsack_01(&weights, &values, capacity), 7);
//! ```
use std::ops::{Add, Sub};

pub fn knapsack_01<W, V>(weights: &[W], values: &[V], capacity: W) -> V
where
    W: Copy + PartialOrd + Sub<Output = W> + From<u8> + Into<usize>,
    V: Copy + Add<Output = V> + Ord + Default,
{
    let n = weights.len();
    let cap = <W as Into<usize>>::into(capacity);
    let mut dp = vec![vec![V::default(); cap + 1]; n + 1];
    for i in 1..=n {
        for w in 0..=cap {
            let weight_i_1 = <W as Into<usize>>::into(weights[i - 1]);
            if weight_i_1 <= w {
                let include = values[i - 1] + dp[i - 1][w - weight_i_1];
                let exclude = dp[i - 1][w];
                dp[i][w] = if include > exclude { include } else { exclude };
            } else {
                dp[i][w] = dp[i - 1][w];
            }
        }
    }
    dp[n][cap]
}
