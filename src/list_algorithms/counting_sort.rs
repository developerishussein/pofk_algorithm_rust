//! 🔢 Counting Sort (Generic for Unsigned Integers)
//!
//! Sorts a mutable slice of unsigned integers in ascending order using the counting sort algorithm.
//!
//! # Arguments
//! * `slice` - The mutable slice to sort.
//!
//! # Panics
//! Panics if the slice contains values greater than `max_value`.
//!
//! # Example
//! ```rust
//! use pofk_algorithms::list_algorithms::counting_sort::counting_sort;
//! let mut arr = [4u32, 2, 2, 8, 3, 3, 1];
//! counting_sort(&mut arr, 8);
//! assert_eq!(arr, [1, 2, 2, 3, 3, 4, 8]);
//! ```
//!
//! # Supported Types
//! Only works for unsigned integer types: u8, u16, u32, u64, usize.
use num_traits::{PrimInt, Unsigned, FromPrimitive, ToPrimitive};

pub fn counting_sort<T>(slice: &mut [T], max_value: T)
where
    T: PrimInt + Unsigned + FromPrimitive + ToPrimitive,
{
    let max_usize = max_value.to_usize().expect("max_value too large");
    let mut count = vec![0usize; max_usize + 1];
    for &item in slice.iter() {
        let idx = item.to_usize().expect("Value too large for usize");
        count[idx] += 1;
    }
    let mut i = 0;
    for (num, &c) in count.iter().enumerate() {
        for _ in 0..c {
            slice[i] = T::from_usize(num).expect("Conversion failed");
            i += 1;
        }
    }
}
