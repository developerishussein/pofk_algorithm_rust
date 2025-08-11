#[cfg(test)]
mod tests {
    use crate::list_algorithms::rotate_array_right::rotate_array_right;

    #[test]
    fn test_rotate_basic() {
        let mut arr = [1, 2, 3, 4, 5, 6, 7];
        rotate_array_right(&mut arr, 3);
        assert_eq!(arr, [5, 6, 7, 1, 2, 3, 4]);
    }

    #[test]
    fn test_rotate_empty() {
        let mut arr: [i32; 0] = [];
        rotate_array_right(&mut arr, 2);
        assert_eq!(arr, []);
    }

    #[test]
    fn test_rotate_single() {
        let mut arr = [42];
        rotate_array_right(&mut arr, 1);
        assert_eq!(arr, [42]);
    }

    #[test]
    fn test_rotate_k_zero() {
        let mut arr = [1, 2, 3];
        rotate_array_right(&mut arr, 0);
        assert_eq!(arr, [1, 2, 3]);
    }

    #[test]
    fn test_rotate_k_greater_than_len() {
        let mut arr = [1, 2, 3];
        rotate_array_right(&mut arr, 5);
        assert_eq!(arr, [2, 3, 1]);
    }

    #[test]
    fn test_rotate_k_equal_len() {
        let mut arr = [1, 2, 3];
        rotate_array_right(&mut arr, 3);
        assert_eq!(arr, [1, 2, 3]);
    }
}
