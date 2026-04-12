impl Solution {
    pub fn is_palindrome(mut s: String) -> bool {
        let mut st = String::new();
        for i in s.chars(){
            if i.is_alphanumeric(){
                st.push(i.to_ascii_lowercase());
            }
        }
        println!("{st}");
        st == st.chars().rev().collect::<String>()
    }

    
}
