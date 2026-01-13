use std::collections::{BinaryHeap, HashSet};

pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {

    let mut heap:BinaryHeap<(i32,usize)> = BinaryHeap::new();
    let mut removed:HashSet<usize> = HashSet::new();
    let mut res:Vec<i32> = Vec::new();

    for (i,n) in nums.iter().enumerate() {
        heap.push((*n,i));

        if i >= k as usize {
            removed.insert(i-k as usize);
        }
        if i+1 >= k as usize {
            while let Some((_,idx)) = heap.peek(){
                if removed.contains(idx){
                    heap.pop();
                }
                else {
                    break;
                }
            }
            res.push(heap.peek().unwrap().0);
        }
    }

    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_sliding_window() {
        assert_eq!(max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3), vec![3, 3, 5, 5, 6, 7]);
        assert_eq!(max_sliding_window(vec![1], 1), vec![1]);
    }
}
