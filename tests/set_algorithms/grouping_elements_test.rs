#[cfg(test)]
mod tests {
    use crate::set_algorithms::grouping_elements::grouping_elements;
    use std::collections::HashMap;

    #[test]
    fn test_grouping_elements_basic() {
        let arr = ["apple", "apricot", "banana", "blueberry"];
        let groups = grouping_elements(&arr, |s| s.chars().next().unwrap());
        assert_eq!(groups[&'a'], vec!["apple", "apricot"]);
        assert_eq!(groups[&'b'], vec!["banana", "blueberry"]);
    }

    #[test]
    fn test_grouping_elements_numbers() {
        let arr = [1, 2, 3, 4, 5, 6];
        let groups = grouping_elements(&arr, |&n| n % 2);
        assert_eq!(groups[&0], vec![2, 4, 6]);
        assert_eq!(groups[&1], vec![1, 3, 5]);
    }

    #[test]
    fn test_grouping_elements_empty() {
        let arr: [i32; 0] = [];
        let groups: HashMap<i32, Vec<i32>> = grouping_elements(&arr, |&n| n);
        assert!(groups.is_empty());
    }
}
