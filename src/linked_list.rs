use std::mem::replace;

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn front(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    pub fn push_front(&mut self, value: T) {
        let mut new_node = Node {
            value: value,
            next: None,
        };
        new_node.next = replace(&mut self.head, None);
        self.head = Some(Box::new(new_node));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let list = LinkedList::<i32>::new();
        assert!(list.is_empty());
    }

    #[test]
    fn test_push_front() {
        let mut list = LinkedList::<i32>::new();
        list.push_front(10);
        assert_eq!(list.front(), Some(&10));
        list.push_front(20);
        assert_eq!(list.front(), Some(&20));
    }
}
