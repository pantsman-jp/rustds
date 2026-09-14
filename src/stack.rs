pub struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { data: Vec::new() }
    }

    pub fn push(&mut self, value: T) {
        self.data.push(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_empty_stack() {
        let stack = Stack::<i32>::new();
        assert!(stack.data.is_empty());
    }

    #[test]
    fn push_adds_element() {
        let mut stack = Stack::<i32>::new();
        stack.push(0);
        assert_eq!(stack.data, vec![0]);
        stack.push(1);
        assert_eq!(stack.data, vec![0, 1]);
    }

    #[test]
    fn pop_element() {
        let mut stack = Stack::<i32>::new();
        stack.push(10);
        stack.push(20);
        assert_eq!(stack.pop(), Some(20));
        assert_eq!(stack.pop(), Some(10));
        assert_eq!(stack.pop(), None);
    }
}
