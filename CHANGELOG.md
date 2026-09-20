# Changelog

All notable changes to this project will be documented in this file.

## 0.6.0 (2026-09-20)

### Added

- a generic `HashTable<K, V>` data structure.
- `HashTable::new()` for creating an empty hash table.
- `HashTable::insert()` for inserting and updating key-value pairs.
- `HashTable::get()` for borrowing a value by key.
- `HashTable::contains_key()` for checking whether a key exists.
- `HashTable::remove()` for removing and returning a value by key.
- collision handling using separate chaining.
- unit tests for the Hash Table implementation.

## 0.5.0 (2026-09-17)

### Added

- a generic `Heap<T>` data structure.
- `Heap::new()` for creating an empty min-heap.
- `Heap::is_empty()` for checking whether the heap is empty.
- `Heap::push()` for inserting elements into the heap.
- `Heap::peek()` for borrowing the minimum element.
- `Heap::pop()` for removing and returning the minimum element.
- `Heap::len()` for getting the number of elements in the heap.
- unit tests for the Heap implementation.

## 0.4.0 (2026-09-16)

### Added

- a generic `BST<T>` data structure.
- `BST::new()` for creating an empty binary search tree.
- `BST::is_empty()` for checking whether the binary search tree is empty.
- `BST::insert()` for inserting elements into the binary search tree.
- `BST::contains()` for checking whether an element exists in the binary search tree.
- `BST::in_order()` for traversing the binary search tree in-order.
- unit tests for the BST implementation.

## 0.3.0 (2026-09-15)

### Added

- a generic `LinkedList<T>` data structure.
- `LinkedList::new()` for creating an empty linked list.
- `LinkedList::is_empty()` for checking whether the linked list is empty.
- `LinkedList::front()` for borrowing the first element.
- `LinkedList::push_front()` for adding an element to the front of the linked list.
- `LinkedList::pop_front()` for removing and returning the first element.
- unit tests for the Linked List implementation.

## 0.2.0 (2026-09-15)

### Added

- a generic `Queue<T>` data structure.
- `Queue::new()` for creating an empty queue.
- `Queue::push()` for adding elements to the back of the queue.
- `Queue::pop()` for removing and returning the front element.
- `Queue::peek()` for borrowing the front element.
- `Queue::is_empty()` for checking whether the queue is empty.
- unit tests for the Queue implementation.

## 0.1.0 (2026-09-14)

### Added

- a generic `Stack<T>` data structure.
- `Stack::new()` for creating an empty stack.
- `Stack::push()` for adding elements.
- `Stack::pop()` for removing and returning the top element.
- `Stack::peek()` for borrowing the top element.
- `Stack::is_empty()` for checking whether the stack is empty.
- unit tests for the Stack implementation.
