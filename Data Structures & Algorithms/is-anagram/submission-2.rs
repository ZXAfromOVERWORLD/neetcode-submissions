impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut v = vec![0;26];
        for i in s.bytes(){
            let pos = (i as u8 - b'a') as usize;
            v[pos] += 1;
        }

        for i in t.bytes(){
            let pos = (i as u8 - b'a') as usize;
            v[pos] -= 1;
        }

        for i in v{
            if i != 0{
                return false;
            }
        }
        true
        
    }
}
