/// A queue-like data structure implement through Vector
pub struct Queue<T> {
    items: Vec<T>,
}

impl<T> Queue<T> {
    /// Initialize a queue with empty vector
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
        }
    }

    /// Adds an element into queue
    pub fn enqueue(&mut self, item: T) -> bool {
        self.items.push(item);
        true
    }

    /// Removes the oldest added element in queue
    pub fn dequeue(&mut self) -> Option<T> {
        match self.items.is_empty() {
            false => Some(self.items.remove(0)),
            true => None,
        }
    }

    /// Retrieves the element that is the oldest added without dequeue
    pub fn peek(&self) -> Option<&T> {
        self.items.first()
    }

    /// Retrieves the size of the queue
    pub fn size(&self) -> usize {
        self.items.len()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_new() {
        let queue = super::Queue::<i32>::new();
        assert!(queue.items.is_empty());
    }
    #[test]
    fn test_enqueue() {
        let mut queue = super::Queue::<i32>::new();
        queue.enqueue(32i32);
        assert_eq!(Some(&32i32), queue.peek());
        assert_eq!(1, queue.size());
    }
    #[test]
    fn test_dequeue() {
        let mut queue = super::Queue::<i32>::new();
        queue.enqueue(32i32);
        assert_eq!(Some(32i32), queue.dequeue());
        assert_eq!(None, queue.dequeue());
    }
    #[test]
    fn test_size() {
        let mut queue = super::Queue::<i32>::new();
        queue.enqueue(-20i32);
        assert_eq!(1, queue.size());
        assert_eq!(Some(&-20i32), queue.peek());
    }
    #[test]
    fn test_integration() {
        let mut queue = super::Queue::<i32>::new();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);
        assert_eq!(3, queue.size());
        assert_eq!(Some(1i32), queue.dequeue());
        assert_eq!(Some(&2i32), queue.peek());
        assert_eq!(Some(2i32), queue.dequeue());
        assert_eq!(Some(3i32), queue.dequeue());
        assert_eq!(0, queue.size());
    }
}
