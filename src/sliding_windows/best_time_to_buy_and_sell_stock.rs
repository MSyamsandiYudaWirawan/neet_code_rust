use std::cmp;

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut res: i32 = 0;
    let mut l:usize = 0;
    for r in 0..prices.len() {
        let mut profit = 0;
        if prices[l] < prices[r]{
            profit = prices[r] - prices[l];
        }else {
            l=r;
        }
        res = cmp::max(res,profit);
    }
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_profit() {
        assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0);
    }
}
