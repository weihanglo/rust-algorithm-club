/// A queue-like data structure implement through [`std::vec::Vec`][].
///
/// This is a naive implementation whose insertion time complexity is `O(n)`,
/// which can be improved trivially by using a `Deque` or
/// [`SinglyLinkedList`](crate::collections::SinglyLinkedList).
///
/// References:
///
/// - [Queue (abstract data type)](<https://en.wikipedia.org/wiki/Queue_(abstract_data_type)>)
// ANCHOR: struct
pub struct Queue<T> {
    items: Vec<T>,
}
// ANCHOR_END: struct

impl<T> Queue<T> {
    /// Initialize a queue with empty vector
    // ANCHOR: new
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
    // ANCHOR_END: new

    /// Adds an element into queue.
    ///
    /// # Complexity
    ///
    /// Constant.
    // ANCHOR: enqueue
    pub fn enqueue(&mut self, item: T) {
        self.items.push(item);
    }
    // ANCHOR_END: enqueue

    /// Removes the oldest added element in queue.
    ///
    /// # Complexity
    ///
    /// Linear in the size of the container.
    // ANCHOR: dequeue
    pub fn dequeue(&mut self) -> Option<T> {
        match self.items.is_empty() {
            false => Some(self.items.remove(0)),
            true => None,
        }
    }
    // ANCHOR_END: dequeue

    /// Retrieves the least recently added element without dequeuing.
    ///
    /// # Complexity
    ///
    /// Constant.
    // ANCHOR: peek
    pub fn peek(&self) -> Option<&T> {
        self.items.first()
    }
    // ANCHOR_END: peek

    /// Retrieves the size of the queue.
    ///
    /// # Complexity
    ///
    /// Constant.
    // ANCHOR: size
    pub fn size(&self) -> usize {
        self.items.len()
    }
    // ANCHOR_END: size
}

#[cfg(test)]
mod impl_by_vec {
    use super::*;

    #[test]
    fn new() {
        let queue = Queue::<()>::new();
        assert!(queue.items.is_empty());
    }

    #[test]
    fn enqueue() {
        let mut queue = Queue::new();
        queue.enqueue(32);
        assert_eq!(Some(&32), queue.peek());
        assert_eq!(1, queue.size());
    }
    #[test]
    fn dequeue() {
        let mut queue = Queue::new();
        queue.enqueue(32);
        assert_eq!(Some(32), queue.dequeue());
        assert_eq!(None, queue.dequeue());
    }
    #[test]
    fn size() {
        let mut queue = Queue::new();
        queue.enqueue(-20);
        assert_eq!(1, queue.size());
        assert_eq!(Some(&-20), queue.peek());
    }
    #[test]
    fn integration() {
        let mut queue = Queue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);
        assert_eq!(3, queue.size());
        assert_eq!(Some(1), queue.dequeue());
        assert_eq!(Some(&2), queue.peek());
        assert_eq!(Some(2), queue.dequeue());
        assert_eq!(Some(3), queue.dequeue());
        assert_eq!(0, queue.size());
    }
}
