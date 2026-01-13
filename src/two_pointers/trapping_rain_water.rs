use std::cmp;

pub fn trap(height: Vec<i32>) -> i32 {
    let mut l:usize = 0;
    let mut r:usize = height.len() - 1;
    let mut l_max:i32 = height[l];
    let mut r_max:i32 = height[r];
    let mut res:i32 = 0;

    while l<r {
        if l_max > r_max {
            r-=1;
            r_max = cmp::max(height[r],r_max);
            res += r_max - height[r];
        }else {
            l+=1;
            l_max = cmp::max(height[l],l_max);
            res += l_max - height[l];
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
