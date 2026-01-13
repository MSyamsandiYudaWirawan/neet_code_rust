use std::{cmp, collections::HashSet};

pub fn longest_consecutive_sequence(nums: Vec<i32>) -> i32 {
    let mut res:i32 = 0;
    let set:HashSet<i32> = HashSet::from_iter(nums.iter().cloned());

    for num in set.iter() {
        let mut count:i32 = 1;
        if !set.contains(&(num - 1)){
            while set.contains(&(num + count)){
                count +=1;
            }
            res = cmp::max(res,count);
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
