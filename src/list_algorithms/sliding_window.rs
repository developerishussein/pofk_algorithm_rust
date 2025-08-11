//! 🪟 Sliding Window (Generic)
//!
//! Computes the maximum sum of any contiguous subarray of size `k`.
//!
//! # Type Parameters
//! * `T`: The element type. Must implement `Copy + std::ops::Add<Output = T> + PartialOrd + Default + Sub<Output = T>`.
//!
//! # Arguments
//! * `slice` - The slice to search.
//! * `k` - The window size.
//!
//! # Returns
//! * `Option<T>` - The maximum sum, or None if the slice is too small.
//!
//! # Example
//! ```rust
//! use pofk_algorithm::list_algorithms::sliding_window::sliding_window_max_sum;
//! let arr = [2, 1, 5, 1, 3, 2];
//! assert_eq!(sliding_window_max_sum(&arr, 3), Some(9));
//! ```
pub fn sliding_window_max_sum<T>(slice: &[T], k: usize) -> Option<T>
where
    T: Copy + std::ops::Add<Output = T> + PartialOrd + Default + std::ops::Sub<Output = T>,
{
    if slice.len() < k || k == 0 { return None; }
    let mut window_sum = slice[..k].iter().copied().fold(T::default(), |a, b| a + b);
    let mut max_sum = window_sum;
    for i in k..slice.len() {
        window_sum = window_sum + slice[i] - slice[i - k];
        if window_sum > max_sum {
            max_sum = window_sum;
        }
    }
    Some(max_sum)
}
