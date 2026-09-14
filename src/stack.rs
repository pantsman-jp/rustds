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

    pub fn peek(&self) -> Option<&T> {
        self.data.last()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_empty_stack() {
        let stack = Stack::<i32>::new();
        assert!(stack.is_empty());
    }

    #[test]
    fn push_adds_element() {
        let mut stack = Stack::<i32>::new();
        stack.push(0);
        assert!(!stack.is_empty());
        assert_eq!(stack.peek(), Some(&0));
        stack.push(1);
        assert_eq!(stack.peek(), Some(&1));
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

    #[test]
    fn peek_element() {
        let mut stack = Stack::<i32>::new();
        stack.push(10);
        stack.push(20);
        assert_eq!(stack.peek(), Some(&20));
        stack.push(30);
        assert_eq!(stack.peek(), Some(&30));
        stack.pop();
        stack.pop();
        assert_eq!(stack.peek(), Some(&10));
        stack.pop();
        assert_eq!(stack.peek(), None);
    }

    #[test]
    fn is_empty_works() {
        let mut stack = Stack::<i32>::new();
        assert!(stack.is_empty());
        stack.push(10);
        assert!(!stack.is_empty());
        stack.push(20);
        assert!(!stack.is_empty());
        stack.pop();
        stack.pop();
        assert!(stack.is_empty());
    }
}
