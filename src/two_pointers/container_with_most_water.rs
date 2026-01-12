use std::cmp;

pub fn max_area(height: Vec<i32>) -> i32 {

    let mut l:usize = 0;
    let mut r:usize = height.len() - 1;
    let mut res:i32 = 0;

    while l<r {
        let area:i32 = (r-l) as i32 * (cmp::min(height[l], height[r]));
        res = cmp::max(res,area);
        if height[l] > height[r] {
            r-=1;
        }else{
            l+=1;
        }

    }
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_area() {
        assert_eq!(max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
        assert_eq!(max_area(vec![1, 1]), 1);
    }
}
