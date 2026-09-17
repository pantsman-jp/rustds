pub struct Heap<T> {
    data: Vec<T>,
}

impl<T: Ord> Heap<T> {
    pub fn new() -> Self {
        Heap { data: Vec::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn left_idx(index: usize) -> usize {
        2 * index + 1
    }

    fn right_idx(index: usize) -> usize {
        2 * index + 2
    }

    fn parent_idx(index: usize) -> usize {
        (index - 1) / 2
    }

    pub fn push(&mut self, value: T) {
        self.data.push(value);
        let mut index = self.data.len() - 1;
        while index > 0 {
            let parent_idx = Self::parent_idx(index);
            if self.data[parent_idx] <= self.data[index] {
                break;
            }
            self.data.swap(parent_idx, index);
            index = parent_idx;
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.data.first()
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.data.is_empty() {
            return None;
        }
        let last_idx = self.data.len() - 1;
        self.data.swap(0, last_idx);
        let min = self.data.pop();
        let mut index = 0;
        while Self::left_idx(index) < self.data.len() {
            let left_idx = Self::left_idx(index);
            let right_idx = Self::right_idx(index);
            let child_idx =
                if right_idx < self.data.len() && self.data[right_idx] < self.data[left_idx] {
                    right_idx
                } else {
                    left_idx
                };
            if self.data[index] <= self.data[child_idx] {
                break;
            }
            self.data.swap(index, child_idx);
            index = child_idx;
        }
        min
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let heap = Heap::<i32>::new();
        assert!(heap.is_empty());
    }

    #[test]
    fn test_push() {
        let mut heap = Heap::<i32>::new();
        assert_eq!(heap.peek(), None);
        heap.push(10);
        assert_eq!(heap.peek(), Some(&10));
        heap.push(0);
        heap.push(5);
        assert_eq!(heap.peek(), Some(&0));
        heap.push(30);
        heap.push(20);
        assert_eq!(heap.peek(), Some(&0));
    }

    #[test]
    fn test_pop() {
        let mut heap = Heap::<i32>::new();
        assert_eq!(heap.pop(), None);
        heap.push(10);
        heap.push(0);
        assert_eq!(heap.pop(), Some(0));
        heap.push(5);
        assert_eq!(heap.pop(), Some(5));
        assert_eq!(heap.pop(), Some(10));
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn test_len() {
        let mut heap = Heap::<i32>::new();
        assert_eq!(heap.len(), 0);
        heap.push(10);
        heap.push(5);
        assert_eq!(heap.len(), 2);
        heap.pop();
        assert_eq!(heap.len(), 1);
    }
}
