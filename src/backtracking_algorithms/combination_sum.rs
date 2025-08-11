//! Combination Sum (Generic, Production-Grade)
//!
//! Finds all unique combinations in candidates where the candidate numbers sum to target.
//!
//! # Type Parameters
//! * `T`: Value type. Must implement `Clone` + `Ord` + `Add<Output = T>` + `PartialOrd` + `From<u8>`.
//!
//! # Example
//! ```rust
//! use pofk_algorithm::backtracking_algorithms::combination_sum::*;
//! let candidates = vec![2,3,6,7];
//! let target = 7;
//! let result = combination_sum(&candidates, target);
//! assert_eq!(result, vec![vec![2,2,3], vec![7]]);
//! ```
use std::ops::{Add, Sub};

pub fn combination_sum<T>(candidates: &[T], target: T) -> Vec<Vec<T>>
where
    T: Clone + Ord + Add<Output = T> + Sub<Output = T> + PartialOrd + From<u8>,
{
    let mut res = Vec::new();
    let mut comb = Vec::new();
    fn backtrack<T>(candidates: &[T], target: T, start: usize, comb: &mut Vec<T>, res: &mut Vec<Vec<T>>)
    where
        T: Clone + Ord + Add<Output = T> + Sub<Output = T> + PartialOrd + From<u8>,
    {
        if target == T::from(0u8) {
            res.push(comb.clone());
            return;
        }
        for i in start..candidates.len() {
            if candidates[i] > target {
                continue;
            }
            comb.push(candidates[i].clone());
            backtrack(candidates, target.clone() - candidates[i].clone(), i, comb, res);
            comb.pop();
        }
    }
    let mut sorted = candidates.to_vec();
    sorted.sort();
    backtrack(&sorted, target, 0, &mut comb, &mut res);
    res
}
