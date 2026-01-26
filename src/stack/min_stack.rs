pub struct MinStack {
}

impl MinStack {
    pub fn new() -> Self {
        todo!()
    }
    
    pub fn push(&mut self, val: i32) {
        todo!()
    }
    
    pub fn pop(&mut self) {
        todo!()
    }
    
    pub fn top(&self) -> i32 {
        todo!()
    }
    
    pub fn get_min(&self) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_stack() {
        let mut min_stack = MinStack::new();
        min_stack.push(-2);
        min_stack.push(0);
        min_stack.push(-3);
        assert_eq!(min_stack.get_min(), -3);
        min_stack.pop();
        assert_eq!(min_stack.top(), 0);
        assert_eq!(min_stack.get_min(), -2);
    }
}
