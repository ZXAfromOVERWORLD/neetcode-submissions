use std::collections::HashMap;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut ans = Vec::new();
        let mut map = HashMap::new();
        for i in strs{
            let mut v = vec![0;26];

            for x in i.bytes(){
                let loc = (x as u8 - b'a') as usize;
                v[loc] += 1;
            }
            map.entry(v).or_insert(Vec::new()).push(i);
        }

        for i in map.values(){
            ans.push(i.clone());
        }
        ans
    }
}
