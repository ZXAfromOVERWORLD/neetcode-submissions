use std::collections::HashMap;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map = HashMap::new();
        for i in strs{
            let mut chr = vec![0;26];

            for c in i.chars(){
                let pos = (c as u8 - b'a') as usize;
                chr[pos] += 1;
            }
            map.entry(chr).or_insert(Vec::new()).push(i);
        }
        let mut res : Vec<Vec<String>>= Vec::new();
        
        for i in map.into_values(){
            res.push(i);
        }

        res
        

    }
}
