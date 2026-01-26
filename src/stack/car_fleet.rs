pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_car_fleet() {
        assert_eq!(car_fleet(12, vec![10, 8, 0, 5, 3], vec![2, 4, 1, 1, 3]), 3);
        assert_eq!(car_fleet(10, vec![3], vec![3]), 1);
        assert_eq!(car_fleet(100, vec![0, 2, 4], vec![4, 2, 1]), 1);
    }
}
