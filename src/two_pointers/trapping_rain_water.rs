use std::cmp;

pub fn trap(height: Vec<i32>) -> i32 {
    let mut res: i32 = 0;
    let mut l:usize = 0;
    let mut r:usize = height.len()-1;
    let mut max_left:i32 = height[l];
    let mut max_right:i32 = height[r];

    while l<r {
        if max_left < max_right {
            l+=1;
            max_left = cmp::max(max_left,height[l]);
            res += max_left - height[l];
        }else {
            r-=1;
            max_right = cmp::max(max_right,height[r]);
            res += max_right - height[r];
        }
    }

    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trap() {
        assert_eq!(trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
        assert_eq!(trap(vec![4, 2, 0, 3, 2, 5]), 9);
    }
}
