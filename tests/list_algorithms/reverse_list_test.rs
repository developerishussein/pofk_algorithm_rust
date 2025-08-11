#[cfg(test)]
mod tests {
    use crate::list_algorithms::reverse_list::reverse_list;

    #[test]
    fn test_reverse_list_basic() {
        let mut arr = [1, 2, 3, 4, 5];
        reverse_list(&mut arr);
        assert_eq!(arr, [5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_reverse_list_empty() {
        let mut arr: [i32; 0] = [];
        reverse_list(&mut arr);
        assert_eq!(arr, []);
    }

    #[test]
    fn test_reverse_list_single() {
        let mut arr = [42];
        reverse_list(&mut arr);
        assert_eq!(arr, [42]);
    }

    #[test]
    fn test_reverse_list_even() {
        let mut arr = [1, 2, 3, 4];
        reverse_list(&mut arr);
        assert_eq!(arr, [4, 3, 2, 1]);
    }

    #[test]
    fn test_reverse_list_odd() {
        let mut arr = [1, 2, 3, 4, 5];
        reverse_list(&mut arr);
        assert_eq!(arr, [5, 4, 3, 2, 1]);
    }
}
