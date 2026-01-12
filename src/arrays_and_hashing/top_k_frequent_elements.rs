use std::collections::{BinaryHeap, HashMap};

pub fn top_k_frequent_elements(nums:Vec<i32>, k:i32) -> Vec<i32>{

    let mut map:HashMap<i32,i32> = HashMap::new();
    for num in nums{
        *map.entry(num).or_insert(0) += 1;
    }

    let mut pq:BinaryHeap<(i32,i32)> = BinaryHeap::new();
    for (key,val) in map.into_iter(){
        pq.push((val,key));
    }

    let mut res:Vec<i32> = Vec::new();
    for _ in 0..k as usize {
        res.push(pq.pop().unwrap().1);
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
