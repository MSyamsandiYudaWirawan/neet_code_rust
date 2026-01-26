use std::collections::BinaryHeap;

pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut res: Vec<i32> = Vec::new();
    let mut binary: BinaryHeap<(i32, usize)> = BinaryHeap::new();
    let mut l:usize = 0;
    for r in 0..nums.len() {
        binary.push((nums[r],r));

        while (r-l+1) > k as usize {
            l+=1;
            while let Some((_,idx)) = binary.peek(){
                if idx < &l {
                    binary.pop();
                }else {
                    break;
                }
            }
        }
        if (r-l+1) == k as usize {
            res.push(binary.peek().unwrap().0);
        }
    }
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_sliding_window() {
        assert_eq!(
            max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3),
            vec![3, 3, 5, 5, 6, 7]
        );
        assert_eq!(max_sliding_window(vec![1], 1), vec![1]);
    }
}
