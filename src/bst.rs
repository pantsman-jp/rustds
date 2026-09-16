struct Node<T> {
    left: Option<Box<Node<T>>>,
    value: T,
    right: Option<Box<Node<T>>>,
}

struct BST<T> {
    tree: Option<Box<Node<T>>>,
}

impl<T: Ord> BST<T> {
    pub fn new() -> Self {
        BST { tree: None }
    }

    pub fn is_empty(&self) -> bool {
        self.tree.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let bst = BST::<i32>::new();
        assert!(bst.is_empty());
    }
}
