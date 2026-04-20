impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        let mut res = String::new();

        for i in strs{
            res.push_str(&format!("{}@{}",i.len(),i));
        }
        res
    }

    pub fn decode(s: String) -> Vec<String> {
        let mut i = 0;
        let mut res = Vec::new();
        while i < s.len(){
            let mut j = i;
            while s.as_bytes()[j] != b'@'{
                j += 1;
            }
            let mut start = j + 1;
            let len : usize = s[i..j].parse().unwrap();
            let end  = start + len;

            res.push(s[start..end].to_string());
            i = end
        }
        res
    }
}
