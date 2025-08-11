//! Subset Generation (Generic, Production-Grade)
//!
//! Generates all possible subsets (the power set) of a given set.
//!
//! # Type Parameters
//! * `T`: Value type. Must implement `Clone`.
//!
//! # Example
//! ```rust
//! use pofk_algorithm::backtracking_algorithms::subset_generation::*;
//! let nums = vec![1, 2, 3];
//! let subsets = subset_generation(&nums);
//! assert_eq!(subsets.len(), 8);
//! ```
pub fn subset_generation<T: Clone>(nums: &[T]) -> Vec<Vec<T>> {
    let mut res = Vec::new();
    let mut subset = Vec::new();
    fn backtrack<T: Clone>(start: usize, nums: &[T], subset: &mut Vec<T>, res: &mut Vec<Vec<T>>) {
        res.push(subset.clone());
        for i in start..nums.len() {
            subset.push(nums[i].clone());
            backtrack(i + 1, nums, subset, res);
            subset.pop();
        }
    }
    backtrack(0, nums, &mut subset, &mut res);
    res
}
