use std::collections::HashSet;

pub fn longest_consecutive_sequence(nums:Vec<i32>) -> i32{
    let mut set:HashSet<i32> = HashSet::new();
    let mut res:i32 = 0;
    
    for num in &nums {
        set.insert(*num);
    }
    
    for num in &nums {
        let mut freq:i32 = 1;
        while set.contains(&(num + freq)){
            freq+=1;
        }
        res = res.max(freq);
    }
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(longest_consecutive_sequence(vec![100,4,200,1,3,2]), 4);
    }

    #[test]
    fn test_single() {
        assert_eq!(longest_consecutive_sequence(vec![0,3,7,2,5,8,4,6,0,1]), 9);
    }
}
