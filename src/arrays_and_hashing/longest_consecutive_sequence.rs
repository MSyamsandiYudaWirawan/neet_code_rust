use std::{cmp, collections::HashSet};

pub fn longest_consecutive_sequence(nums: Vec<i32>) -> i32 {
    let set:HashSet<i32> = nums.iter().cloned().collect();
    let mut res:i32 = 0;
    for &num in &set{
        if !set.contains(&(num+1)){
            let mut freq:i32 = 1;
            while set.contains(&(num-freq)){
                freq += 1;
            }
            res = cmp::max(freq, res);
        }
        
    }
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(longest_consecutive_sequence(vec![100, 4, 200, 1, 3, 2]), 4);
    }

    #[test]
    fn test_single() {
        assert_eq!(
            longest_consecutive_sequence(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]),
            9
        );
    }
}
