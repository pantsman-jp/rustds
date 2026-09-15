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

    pub fn pop_front(&mut self) -> Option<T> {
        let head = replace(&mut self.head, None);
        match head {
            None => None,
            Some(node) => {
                let node = *node;
                self.head = node.next;
                Some(node.value)
            }
        }
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

    #[test]
    fn test_pop_front() {
        let mut list = LinkedList::<i32>::new();
        list.push_front(10);
        list.push_front(20);
        assert_eq!(list.pop_front(), Some(20));
        assert_eq!(list.pop_front(), Some(10));
        assert_eq!(list.pop_front(), None);
    }
}
