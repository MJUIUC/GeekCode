struct MinStack {
    stack: Vec<i32>,
    min_pointer_stack: Vec<usize>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    fn new() -> Self {
        Self {
            stack: vec![],
            min_pointer_stack: vec![],
        }
    }

    fn push(&mut self, val: i32) {
        let cur_min_pointer: usize = if self.min_pointer_stack.is_empty() {
            self.min_pointer_stack.push(0);
            0
        } else {
            *self.min_pointer_stack.last().unwrap()
        };

        if !self.stack.is_empty() && self.stack[cur_min_pointer] > val {
            self.min_pointer_stack.push(self.stack.len());
        }

        self.stack.push(val);
    }

    fn pop(&mut self) {
        let cur_pointer = self.stack.len() - 1;
        let cur_min_pointer = *self.min_pointer_stack.last().unwrap();

        if cur_pointer == cur_min_pointer {
            self.min_pointer_stack.pop();
        }

        let _ = self.stack.pop();
    }

    fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        let cur_min_index = *self.min_pointer_stack.last().unwrap();
        self.stack[cur_min_index]
    }
}

// Your MinStack object will be instantiated and called as such:
// let obj = MinStack::new();
// obj.push(val);
// obj.pop();
// let ret_3: i32 = obj.top();
// let ret_4: i32 = obj.get_min();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_operations() {
        let mut stack = MinStack::new();
        stack.push(-2);
        stack.push(0);
        stack.push(-3);
        assert_eq!(stack.get_min(), -3);
        stack.pop();
        assert_eq!(stack.top(), 0);
        assert_eq!(stack.get_min(), -2);
    }

    #[test]
    fn single_element() {
        let mut stack = MinStack::new();
        stack.push(42);
        assert_eq!(stack.top(), 42);
        assert_eq!(stack.get_min(), 42);
        stack.pop();
    }

    #[test]
    fn ascending_order() {
        let mut stack = MinStack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(stack.get_min(), 1);
        stack.pop();
        assert_eq!(stack.get_min(), 1);
        stack.pop();
        assert_eq!(stack.get_min(), 1);
    }

    #[test]
    fn descending_order() {
        let mut stack = MinStack::new();
        stack.push(3);
        stack.push(2);
        stack.push(1);
        assert_eq!(stack.get_min(), 1);
        stack.pop();
        assert_eq!(stack.get_min(), 2);
        stack.pop();
        assert_eq!(stack.get_min(), 3);
    }

    #[test]
    fn duplicate_mins() {
        let mut stack = MinStack::new();
        stack.push(1);
        stack.push(0);
        stack.push(0);
        assert_eq!(stack.get_min(), 0);
        stack.pop();
        assert_eq!(stack.get_min(), 0);
        stack.pop();
        assert_eq!(stack.get_min(), 1);
    }

    #[test]
    fn min_pointer_stack_is_smaller() {
        let mut stack = MinStack::new();
        stack.push(5);
        stack.push(3);
        stack.push(7);
        stack.push(8);
        stack.push(2);
        assert!(stack.min_pointer_stack.len() < stack.stack.len());
        assert_eq!(stack.get_min(), 2);
    }
}
