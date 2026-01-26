use std::collections::{BinaryHeap, HashMap};

pub fn top_k_frequent_elements(nums:Vec<i32>, k:i32) -> Vec<i32>{
    let mut map:HashMap<i32,i32> = HashMap::new();
    let mut binary:BinaryHeap<(i32,i32)> = BinaryHeap::new();
    let mut res:Vec<i32> = Vec::with_capacity(k as usize);
    for num in nums {
        *map.entry(num).or_insert(0) +=1;
    }
    for (key, value) in map {
        binary.push((value,key));
    }
    for _ in 0..k {
        res.push(binary.pop().unwrap().1);
    }
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_top_k_frequent_elements() {
        let mut result = top_k_frequent_elements(vec![1,1,1,2,2,3], 2);
        result.sort();
        assert_eq!(result, vec![1,2]);
        
        assert_eq!(top_k_frequent_elements(vec![1], 1), vec![1]);
    }
}
