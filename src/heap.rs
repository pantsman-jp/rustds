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

    fn left_idx(index: &usize) -> usize {
        2 * index + 1
    }

    fn right_idx(index: &usize) -> usize {
        2 * index + 2
    }

    fn parent_idx(index: &usize) -> usize {
        (index - 1) / 2
    }

    pub fn push(&mut self, value: T) {
        self.data.push(value);
        let mut index = self.data.len() - 1;
        while index > 0 {
            let parent_idx = Self::parent_idx(&index);
            if self.data[parent_idx] <= self.data[index] {
                break;
            }
            self.data.swap(parent_idx, index);
            index = parent_idx;
        }
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
}
