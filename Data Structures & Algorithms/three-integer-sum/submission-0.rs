impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();

        let mut res : Vec<Vec<i32>> = Vec::new();
        for i in 0..nums.len(){
            if i > 0 && nums[i] == nums[i-1]{
              continue;  
            }
            let mut left = i as i32 + 1;
            let mut right = nums.len() as i32 - 1;

            while left < right{
                let sum = nums[left as usize] + nums[right as usize] + nums[i];
                if sum == 0{
                    res.push(vec![nums[i] , nums[left as usize] ,nums[right as usize]]);
                    while left < right && nums[left as usize] == nums[left as usize + 1]{
                        left += 1;
                    }

                    while left < right && nums[right as usize] == nums[right as usize - 1]{
                        right -= 1;
                    }
                    left += 1;
                    right -= 1;
                } else if sum > 0{
                    right -= 1;
                } else {
                    left += 1;
                }
            }
        }
        return res;
    }
}
