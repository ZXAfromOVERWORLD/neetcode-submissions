impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut i = 0_i32;
        let mut j = numbers.len() as i32 - 1;
        while i < j{
            if numbers[i as usize] + numbers[j as usize] == target{
                return vec![i as i32 + 1, j as i32 +1];
            } else if numbers[i as usize] + numbers[j as usize] < target{
                i += 1;
            } else {
                j -= 1;
            }
        }
        vec![]
    }
}
