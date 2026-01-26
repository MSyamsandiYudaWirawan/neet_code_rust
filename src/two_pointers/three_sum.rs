pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res:Vec<Vec<i32>> = Vec::new();
    nums.sort();

   for i in 0..nums.len() - 2 {
       if i>0 && nums[i] == nums[i-1]{
           continue;
       }
       let mut l:usize = i+1;
       let mut r:usize = nums.len() - 1;
       while l<r {
           let sum = nums[i] + nums[l] + nums[r];
           if sum > 0 {
               r-=1;
           } else if sum < 0 {
               l+=1;
           }
           else {
               res.push(vec![nums[i],nums[l],nums[r]]);
               l+=1;
               r-=1;
               while l<r && nums[l] == nums[l-1] {
                   l+=1;
               }
               while l<r && nums[r] == nums[r+1] {
                   r-=1;
               }
           }
       }
   }
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_sum() {
        assert_eq!(
            three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vec![vec![-1, -1, 2], vec![-1, 0, 1]]
        );
        assert_eq!(three_sum(vec![0, 1, 1]), Vec::<Vec<i32>>::new());
        assert_eq!(three_sum(vec![0, 0, 0]), vec![vec![0, 0, 0]]);
    }
}
