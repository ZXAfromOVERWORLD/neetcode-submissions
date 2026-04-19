use std::collections::VecDeque;

struct MyStack {
    v : VecDeque<i32>,
}

impl MyStack {
    pub fn new() -> Self {
        MyStack{
        v : VecDeque::new(),
        }
    }

    pub fn push(&mut self, x: i32) {
        self.v.push_back(x);
    }

    pub fn pop(&mut self) -> i32 {
        let len = self.v.len() - 1;
        for i in 0..len{
            let dig = self.v.pop_front().unwrap();
            self.v.push_back(dig);
        }
        self.v.pop_front().unwrap()
    }

    pub fn top(&self) -> i32 {
        self.v[self.v.len()-1]
    }

    pub fn empty(&self) -> bool {
        self.v.len() == 0
    }
}
