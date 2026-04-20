impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len()-1;
        while left < right{
            if nums[left] < nums[right]{
                return nums[left];
            } else if nums[left] == nums[right]{
                left += 1;
            }
            let mid = left + (right - left)/2;
            if nums[mid] >= nums[left]{
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        nums[left]
    }
}
