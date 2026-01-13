pub fn product_of_array_except_self(nums: Vec<i32>) -> Vec<i32>{
   let mut res:Vec<i32> = vec![1; nums.len()];

    let mut lp:i32 = 1;
    for i in 0..nums.len() {
        res[i] *= lp;
        lp *= nums[i];
    }

    let mut rp:i32 = 1;
    for i in (0..nums.len()).rev() {
        res[i] *= rp;
        rp *= nums[i];
    }
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(product_of_array_except_self(vec![1,2,3,4]), vec![24,12,8,6]);
    }

    #[test]
    fn test_with_zero() {
        assert_eq!(product_of_array_except_self(vec![-1,1,0,-3,3]), vec![0,0,9,0,0]);
    }
}
