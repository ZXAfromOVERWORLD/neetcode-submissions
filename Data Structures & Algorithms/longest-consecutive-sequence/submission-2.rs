use std::collections::HashSet;
use std::cmp;
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut set = HashSet::new();
        let mut res = 1;
        for &i in &nums{
            set.insert(i);
        }
        for i in nums{
            let mut cur = 1;
            let mut num = i;
            if set.contains(&(i-1)){
                continue;
            }
            while set.contains(&(num+1)){
                cur += 1;
                num += 1;
            }
            res = cmp::max(res,cur);
        }
        res
    }
}