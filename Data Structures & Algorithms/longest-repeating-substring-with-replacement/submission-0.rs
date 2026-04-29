use std::collections::HashMap;
use std::cmp;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let mut map = HashMap::new();

        let s = s.as_bytes();
        let mut l = 0;
 
        let mut max = 0;
        let mut res = 0;
        for i in 0..s.len(){

            let entry = map.entry(s[i]).or_insert(0);
            *entry += 1;

            max = cmp::max(max, *entry);

            while i-l+1 - max > k as usize{
                *map.entry(s[l]).or_insert(0) -= 1;
                l += 1;
            }

            res = cmp::max(res, i-l+1);


        }
        res as i32
    }
}
