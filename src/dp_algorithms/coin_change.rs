//! Coin Change (Generic, Production-Grade)
//!
//! Finds the minimum number of coins needed to make up a given amount.
//!
//! # Type Parameters
//! * `T`: Value type. Must implement `Copy` + `Ord` + `Sub<Output = T>` + `From<u8>` + `Default`.
//!
//! # Example
//! ```rust
//! use pofk_algorithm::dp_algorithms::coin_change::*;
//! let coins = vec![1usize, 2, 5];
//! let amount = 11usize;
//! assert_eq!(coin_change(&coins, amount), 3); // 11 = 5+5+1
//! ```
use std::ops::Sub;

pub fn coin_change<T>(coins: &[T], amount: T) -> i32
where
    T: Copy + Ord + Sub<Output = T> + From<u8> + Default + Into<usize>,
{
    let amount_usize = <T as Into<usize>>::into(amount);
    let mut dp = vec![i32::MAX - 1; amount_usize + 1];
    dp[0] = 0;
    for i in 1..=amount_usize {
        for &coin in coins {
            let coin_usize = <T as Into<usize>>::into(coin);
            if coin_usize <= i {
                dp[i] = dp[i].min(dp[i - coin_usize] + 1);
            }
        }
    }
    if dp[amount_usize] == i32::MAX - 1 { -1 } else { dp[amount_usize] }
}
