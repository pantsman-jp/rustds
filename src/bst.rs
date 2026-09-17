struct Node<T> {
    left: Option<Box<Node<T>>>,
    value: T,
    right: Option<Box<Node<T>>>,
}

pub struct BST<T> {
    tree: Option<Box<Node<T>>>,
}

impl<T: Ord> BST<T> {
    pub fn new() -> Self {
        BST { tree: None }
    }

    pub fn is_empty(&self) -> bool {
        self.tree.is_none()
    }

    fn insert_node(node: &mut Option<Box<Node<T>>>, value: T) {
        match node {
            Some(tree) => {
                if value < tree.value {
                    Self::insert_node(&mut tree.left, value);
                } else if value > tree.value {
                    Self::insert_node(&mut tree.right, value);
                }
            }
            None => {
                *node = Some(Box::new(Node {
                    left: None,
                    value: value,
                    right: None,
                }));
            }
        }
    }

    pub fn insert(&mut self, value: T) {
        Self::insert_node(&mut self.tree, value);
    }

    fn contains_node(node: &Option<Box<Node<T>>>, value: &T) -> bool {
        match node {
            Some(tree) => {
                if &tree.value < value {
                    Self::contains_node(&tree.right, value)
                } else if &tree.value == value {
                    true
                } else {
                    Self::contains_node(&tree.left, value)
                }
            }
            None => false,
        }
    }

    pub fn contains(&self, value: &T) -> bool {
        Self::contains_node(&self.tree, value)
    }

    fn inorder_node<'a>(node: &'a Option<Box<Node<T>>>, result: &mut Vec<&'a T>) {
        match node {
            Some(tree) => {
                Self::inorder_node(&tree.left, result);
                result.push(&tree.value);
                Self::inorder_node(&tree.right, result);
            }
            None => {}
        }
    }

    pub fn in_order(&self) -> Vec<&T> {
        let mut result = Vec::new();
        Self::inorder_node(&self.tree, &mut result);
        result
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

    #[test]
    fn test_insert() {
        let mut bst = BST::<i32>::new();
        bst.insert(8);
        assert!(bst.contains(&8));
        bst.insert(10);
        bst.insert(12);
        bst.insert(6);
        bst.insert(4);
        assert!(bst.contains(&10));
        assert!(!bst.contains(&5));
        assert!(!bst.contains(&0));
    }

    #[test]
    fn test_in_order() {
        let mut bst = BST::<i32>::new();
        bst.insert(8);
        bst.insert(10);
        bst.insert(12);
        bst.insert(6);
        bst.insert(4);
        assert_eq!(bst.in_order(), vec![&4, &6, &8, &10, &12]);
    }
}
