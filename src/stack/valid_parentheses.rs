pub fn is_valid(s: String) -> bool {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        assert_eq!(is_valid("()".to_string()), true);
        assert_eq!(is_valid("()[]{}".to_string()), true);
        assert_eq!(is_valid("(]".to_string()), false);
        assert_eq!(is_valid("([)]".to_string()), false);
        assert_eq!(is_valid("{[]}".to_string()), true);
    }
}
