pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut l: usize = 0;
    let mut r: usize = numbers.len() - 1;

    while l < r {
        let sum: i32 = numbers[l] + numbers[r];
        if sum > target {
            r -= 1;
        } else if sum < target {
            l += 1;
        } else {
            return vec![l as i32 + 1,r as i32 + 1];
        }
    }
    return Vec::new();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
        assert_eq!(two_sum(vec![2, 3, 4], 6), vec![1, 3]);
        assert_eq!(two_sum(vec![-1, 0], -1), vec![1, 2]);
    }
}
