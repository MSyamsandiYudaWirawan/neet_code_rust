pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_daily_temperatures() {
        assert_eq!(daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]), vec![1, 1, 4, 2, 1, 1, 0, 0]);
        assert_eq!(daily_temperatures(vec![30, 40, 50, 60]), vec![1, 1, 1, 0]);
        assert_eq!(daily_temperatures(vec![30, 60, 90]), vec![1, 1, 0]);
    }
}
