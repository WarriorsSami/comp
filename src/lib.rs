#[cfg(test)]
mod tests {
    use comp_derive::comp;

    #[test]
    fn test_list_comprehension() {
        let result = comp![x * x for x in 0..=10 if x % 2 == 0 if x % 3 == 0].collect::<Vec<_>>();
        assert_eq!(result, vec![0, 36]);
    }
}
