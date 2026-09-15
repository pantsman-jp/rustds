use std::collections::VecDeque;

// FIFO
pub struct Queue<T> {
    data: VecDeque<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            data: VecDeque::new(),
        }
    }

    pub fn push(&mut self, value: T) {
        self.data.push_back(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.data.pop_front()
    }

    pub fn peek(&self) -> Option<&T> {
        self.data.front()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_empty_queue() {
        let q = Queue::<i32>::new();
        assert_eq!(q.data, VecDeque::new());
    }

    #[test]
    fn push_element() {
        let mut q = Queue::<i32>::new();
        q.push(10);
        assert_eq!(q.data, [10]);
        q.push(20);
        assert_eq!(q.data, [10, 20]);
    }

    #[test]
    fn pop_element() {
        let mut q = Queue::<i32>::new();
        q.push(10);
        assert_eq!(q.data, [10]);
        assert_eq!(q.pop(), Some(10));
    }

    #[test]
    fn peek_element() {
        let mut q = Queue::<i32>::new();
        q.push(10);
        assert_eq!(q.peek(), Some(&10));
        q.push(20);
        assert_eq!(q.peek(), Some(&10));
        q.pop();
        assert_eq!(q.peek(), Some(&20));
        q.pop();
        assert_eq!(q.peek(), None);
    }
}
