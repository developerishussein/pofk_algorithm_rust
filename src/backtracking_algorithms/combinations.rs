//! Combinations (Generic, Production-Grade)
//!
//! Generates all possible combinations of k elements from a set.
//!
//! # Type Parameters
//! * `T`: Value type. Must implement `Clone`.
//!
//! # Example
//! ```rust
//! use pofk_algorithms::backtracking_algorithms::combinations::*;
//! let nums = vec![1, 2, 3, 4];
//! let combs = combinations(&nums, 2);
//! assert_eq!(combs.len(), 6);
//! ```
pub fn combinations<T: Clone>(nums: &[T], k: usize) -> Vec<Vec<T>> {
    let mut res = Vec::new();
    let mut comb = Vec::new();
    fn backtrack<T: Clone>(start: usize, nums: &[T], k: usize, comb: &mut Vec<T>, res: &mut Vec<Vec<T>>) {
        if comb.len() == k {
            res.push(comb.clone());
            return;
        }
        for i in start..nums.len() {
            comb.push(nums[i].clone());
            backtrack(i + 1, nums, k, comb, res);
            comb.pop();
        }
    }
    backtrack(0, nums, k, &mut comb, &mut res);
    res
}
