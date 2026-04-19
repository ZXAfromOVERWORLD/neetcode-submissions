impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        for i in s.chars(){
            match i{
                ')' => {
                    if stack.pop() != Some('('){
                        return false;
                    }
                },
                '}' => {
                    if stack.pop() != Some('{'){
                        return false;
                    }
                },
                ']' => {
                    if stack.pop() != Some('['){
                        return false;
                    }
                },
                _ => stack.push(i),
            }
        }
        stack.is_empty()
    }
}
