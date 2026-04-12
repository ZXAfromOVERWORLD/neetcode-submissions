use std::cmp;
impl Solution {
    pub fn max_area(heights: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut left = 0;
        let mut right = heights.len()-1;
        while left < right{
            let water = cmp::min(heights[left],heights[right])* (right - left) as i32;
            res = cmp::max(res,water);

            if heights[left] <= heights[right]{
                left += 1;
            } else {
                right -= 1;
            }
        }
        res
    }
}
