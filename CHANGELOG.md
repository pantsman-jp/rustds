# Changelog

All notable changes to this project will be documented in this file.

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
