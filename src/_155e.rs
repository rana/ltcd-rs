// Min Stack
// Design a stack supporting operations
// push, pop, top, get_min.
// Condition:
//  * All operations have time complexity of O(1).
// Use two stacks.
// One stack for operations push, pop, top.
// One stack for get_min.

struct MinStack {
    stk: Vec<i32>,
    min_stk: Vec<i32>,
}
impl MinStack {
    fn new() -> Self {
        MinStack {
            stk: Vec::new(),
            min_stk: Vec::new(),
        }
    }

    fn push(&mut self, val: i32) {
        self.stk.push(val);
        let min_val: i32 = match self.min_stk.last() {
            Some(&min) => min.min(val),
            None => val,
        };
        self.min_stk.push(min_val);
    }

    fn pop(&mut self) {
        self.stk.pop();
        self.min_stk.pop();
    }

    fn top(&self) -> i32 {
        *self.stk.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        *self.min_stk.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_stack_operations() {
        let mut stk = MinStack::new();
        stk.push(-2);
        stk.push(0);
        stk.push(-3);
        assert_eq!(stk.get_min(), -3);
        stk.pop();
        assert_eq!(stk.top(), 0);
        assert_eq!(stk.get_min(), -2);
    }

    #[test]
    fn test_single_element() {
        let mut stk = MinStack::new();
        stk.push(42);
        assert_eq!(stk.top(), 42);
        assert_eq!(stk.get_min(), 42);
        stk.pop();
    }

    #[test]
    fn test_increasing_elements() {
        let mut stk = MinStack::new();
        stk.push(1);
        stk.push(2);
        stk.push(3);
        assert_eq!(stk.get_min(), 1);
    }

    #[test]
    fn test_decreasing_elements() {
        let mut stk = MinStack::new();
        stk.push(3);
        stk.push(2);
        stk.push(1);
        assert_eq!(stk.get_min(), 1);
        stk.pop();
        assert_eq!(stk.get_min(), 2);
    }
}
