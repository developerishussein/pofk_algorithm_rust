use std::ops::Add;

/// Jump Game (Generic, Production-Grade)
///
/// Determines if you can reach the last index.
///
/// # Type Parameters
/// * `T`: Value type. Must implement `Copy` + `Add<Output = T>` + `PartialOrd` + `From<u8>`.
///
/// # Example
/// ```rust
/// use pofk_algorithms::dp_algorithms::jump_game::*;
/// let nums = vec![2,3,1,1,4];
/// assert!(jump_game(&nums));
/// ```
pub fn jump_game<T>(nums: &[T]) -> bool
where
    T: Copy + Add<Output = T> + PartialOrd + From<u8>,
{
    let mut max_reach = T::from(0u8);
    for (i, &num) in nums.iter().enumerate() {
        if T::from(i as u8) > max_reach {
            return false;
        }
        let candidate = T::from(i as u8) + num;
        if candidate > max_reach {
            max_reach = candidate;
        }
    }
    true
}
