use std::collections::HashSet;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut set = HashSet::new();

        let s = s.as_bytes();
        let mut l = 0;
        let mut ans = 0;

        for i in 0..s.len(){
            while set.contains(&s[i]){
                set.remove(&s[l]);
                l += 1;
            }

            set.insert(s[i]);
            ans = ans.max(set.len());
        }
        ans as i32
    }
}
