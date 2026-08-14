impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut f_v = vec![0;26];

        for i in s.bytes(){
            let loc = (i as u8 - b'a') as usize;
            f_v[loc] += 1;
        }

        for i in t.bytes(){
            let loc = (i as u8 - b'a') as usize;
            f_v[loc] -= 1; 
        }

        for i in f_v{
            if i != 0{
                return false;
            }
        }
        return true;
    }
}
