#[cfg(test)]
mod tests {
    use crate::set_algorithms::find_intersection::find_intersection;
    #[test]
    fn test_find_intersection() {
        let a = [1, 2, 3];
        let b = [2, 3, 4];
        let mut inter = find_intersection(&a, &b);
        inter.sort();
        assert_eq!(inter, vec![2, 3]);
        let a = [1, 2];
        let b = [3, 4];
        let inter = find_intersection(&a, &b);
        assert!(inter.is_empty());
    }
}
