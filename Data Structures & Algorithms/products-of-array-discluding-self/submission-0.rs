impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut res = Vec::new();
        let mut prefix = vec![0; n];
        let mut suffix = vec![0; n]; 
        let mut run = 1;
        for i in 0..n{
            run *= nums[i];
            prefix[i] = run;
        }
        run = 1;
        for i in (0..n).rev(){
            run *= nums[i];
            suffix[i] = run;
        }

        for i in 0..n{
            if i == 0{
                res.push(suffix[1]);
                continue;
            }

            if i == n - 1{
                res.push(prefix[i-1]);
                continue;
            }

            res.push(suffix[i+1]*prefix[i-1]);
        }

        res

    }
}