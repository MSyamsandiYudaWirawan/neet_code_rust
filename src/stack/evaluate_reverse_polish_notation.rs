pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_rpn() {
        assert_eq!(eval_rpn(vec!["2".to_string(), "1".to_string(), "+".to_string(), "3".to_string(), "*".to_string()]), 9);
        assert_eq!(eval_rpn(vec!["4".to_string(), "13".to_string(), "5".to_string(), "/".to_string(), "+".to_string()]), 6);
        assert_eq!(eval_rpn(vec!["10".to_string(), "6".to_string(), "9".to_string(), "3".to_string(), "+".to_string(), "-11".to_string(), "*".to_string(), "/".to_string(), "*".to_string(), "17".to_string(), "+".to_string(), "5".to_string(), "+".to_string()]), 22);
    }
}
