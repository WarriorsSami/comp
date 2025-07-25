#[cfg(test)]
mod tests {
    use comp_derive::comp;

    #[test]
    fn test_list_comprehension() {
        let result = comp![x * x for x in 0..=10 if x % 2 == 0 if x % 3 == 0].collect::<Vec<_>>();
        assert_eq!(result, vec![0, 36]);
    }

    #[test]
    fn test_nested_for_if() {
        let result = comp![(x, y)
            for x in 0..=3 if x % 2 == 0
            for y in 0..=3 if y > x]
            .collect::<Vec<_>>();
        assert_eq!(result, vec![(0, 1), (0, 2), (0, 3), (2, 3)]);
    }
}
