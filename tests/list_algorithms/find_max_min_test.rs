#[cfg(test)]
mod tests {
    use crate::list_algorithms::find_max_min::find_max_min;

    #[test]
    fn test_find_max_min_basic() {
        let arr = [3, 1, 4, 1, 5, 9];
        assert_eq!(find_max_min(&arr), Some((1, 9)));
    }

    #[test]
    fn test_find_max_min_empty() {
        let arr: [i32; 0] = [];
        assert_eq!(find_max_min(&arr), None);
    }

    #[test]
    fn test_find_max_min_single() {
        let arr = [42];
        assert_eq!(find_max_min(&arr), Some((42, 42)));
    }

    #[test]
    fn test_find_max_min_negative() {
        let arr = [-3, -1, -4, -2];
        assert_eq!(find_max_min(&arr), Some((-4, -1)));
    }

    #[test]
    fn test_find_max_min_duplicates() {
        let arr = [2, 2, 2, 2];
        assert_eq!(find_max_min(&arr), Some((2, 2)));
    }
}
