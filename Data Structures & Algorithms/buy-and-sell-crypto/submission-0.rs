use std::cmp;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min = i32::MAX;
        let mut ans = 0;

        for i in prices{
            min = cmp::min(min,i);
            ans = cmp::max(ans, i - min);
        }
        ans
    }
}
