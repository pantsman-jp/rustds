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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_empty_queue() {
        let q = Queue::<i32>::new();
        assert_eq!(q.data, VecDeque::new());
    }
}
