//! Fibonacci with Memoization (Generic, Production-Grade)
//!
//! Computes the nth Fibonacci number using memoization for high performance.
//!
//! # Type Parameters
//! * `T`: Integer type. Must implement `Copy`, `From<u8>`, `Add<Output = T>`, `Sub<Output = T>`, `PartialEq`, `PartialOrd`, and `Default`.
//!
//! # Example
//! ```rust
//! use pofk_algorithm::dp_algorithms::fibonacci::*;
//! assert_eq!(fibonacci::<u64>(10), 55);
//! ```
use std::collections::HashMap;
use std::ops::{Add, Sub};

pub fn fibonacci<T>(n: T) -> T
where
    T: Copy + From<u8> + Add<Output = T> + Sub<Output = T> + PartialEq + PartialOrd + Default + Eq + std::hash::Hash,
{
    fn fib<T>(n: T, memo: &mut HashMap<T, T>) -> T
    where
        T: Copy + From<u8> + Add<Output = T> + Sub<Output = T> + PartialEq + PartialOrd + Default + Eq + std::hash::Hash,
    {
        if n == T::from(0u8) {
            return T::from(0u8);
        }
        if n == T::from(1u8) {
            return T::from(1u8);
        }
        if let Some(&val) = memo.get(&n) {
            return val;
        }
        let val = fib(n - T::from(1u8), memo) + fib(n - T::from(2u8), memo);
        memo.insert(n, val);
        val
    }
    let mut memo = HashMap::new();
    fib(n, &mut memo)
}
