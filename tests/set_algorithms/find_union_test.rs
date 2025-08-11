#[cfg(test)]
mod tests {
    use crate::set_algorithms::find_union::find_union;
    #[test]
    fn test_find_union() {
        let a = [1, 2, 3];
        let b = [3, 4, 5];
        let mut union = find_union(&a, &b);
        union.sort();
        assert_eq!(union, vec![1, 2, 3, 4, 5]);
        let a: [i32; 0] = [];
        let b = [1, 2];
        let mut union = find_union(&a, &b);
        union.sort();
        assert_eq!(union, vec![1, 2]);
    }
}
