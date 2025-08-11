#[cfg(test)]
mod tests {
    use crate::set_algorithms::set_difference::set_difference;
    #[test]
    fn test_set_difference() {
        let a = [1, 2, 3, 4];
        let b = [2, 4];
        let mut diff = set_difference(&a, &b);
        diff.sort();
        assert_eq!(diff, vec![1, 3]);
        let a = [1, 2];
        let b: [i32; 0] = [];
        let mut diff = set_difference(&a, &b);
        diff.sort();
        assert_eq!(diff, vec![1, 2]);
    }
}
