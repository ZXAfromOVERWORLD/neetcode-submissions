impl Solution {
    pub fn cal_points(operations: Vec<String>) -> i32 {
        let mut stack : Vec<i32> = Vec::new();
        for i in &operations{
            match i.as_str(){
                "C" => {stack.pop();},
                "D" => {
                    let digit = stack.last().unwrap();
                    stack.push(digit*2);
                }
                "+" => {
                    let len = stack.len();
                    let sum = stack[len-1] + stack[len-2];
                    stack.push(sum);
                }
                _ => stack.push(i.parse::<i32>().unwrap()),
            }
        }
        let mut ans = 0;
        for i in stack{
            ans += i;
        }
        ans
    }
}
