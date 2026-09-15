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
}
