use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
   let mut map:HashMap<i32,usize> = HashMap::new();

    for (i,num) in nums.iter().enumerate() {
        let diff = target - num;
        if let Some(&usize) = map.get(&diff){
            return vec![usize as i32, i as i32];
        }
        map.insert(*num,i);
    }
    return Vec::new();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}
