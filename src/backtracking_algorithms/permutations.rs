//! Permutations (Generic, Production-Grade)
//!
//! Generates all possible permutations of a given set.
//!
//! # Type Parameters
//! * `T`: Value type. Must implement `Clone`.
//!
//! # Example
//! ```rust
//! use pofk_algorithm::backtracking_algorithms::permutations::*;
//! let nums = vec![1, 2, 3];
//! let perms = permutations(&nums);
//! assert_eq!(perms.len(), 6);
//! ```
pub fn permutations<T: Clone>(nums: &[T]) -> Vec<Vec<T>> {
    let mut res = Vec::new();
    let mut nums = nums.to_vec();
    fn backtrack<T: Clone>(start: usize, nums: &mut Vec<T>, res: &mut Vec<Vec<T>>) {
        if start == nums.len() {
            res.push(nums.clone());
            return;
        }
        for i in start..nums.len() {
            nums.swap(start, i);
            backtrack(start + 1, nums, res);
            nums.swap(start, i);
        }
    }
    backtrack(0, &mut nums, &mut res);
    res
}
