/// A stack-like data structure implemented through a `Vec`.
///
/// The name "stack" for this type of structure comes from the analogy to a set
/// of physical items stacked on top of each other, which makes it easy to take
/// an item off the top of the stack, while getting to an item deeper in the
/// stack may require taking off multiple other items first.
///
/// Considered as a linear data structure, or more abstractly a sequential
/// collection, the push and pop operations occur only at one end of the
/// structure, referred to as the top of the stack.
///
/// References:
///
/// * [Stack (abstract data type)](https://en.wikipedia.org/wiki/Stack_\(abstract_data_type\))
/// * [Big-O Algorithm Complexity Cheat Sheet](http://bigocheatsheet.com/)
pub struct Stack<T> {
    maxsize: usize,
    items: Vec<T>,
}

impl<T> Stack<T> {
    /// Initialize a stack of certain capacity.
    ///
    /// # Parameters
    ///
    /// * `maxsize`: Capacity of the collection. It limits how many items can
    /// be stored.
    pub fn with_capacity(maxsize: usize) -> Self {
        Self {
            maxsize,
            items: Vec::with_capacity(maxsize),
        }
    }

    /// Removes the most recently added element that was not yet removed.
    ///
    /// # Returns
    ///
    /// Returns the most recently added item. If nothing was added, `None` will be returned.
    ///
    /// # Complexity
    ///
    /// Constant.
    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    /// Adds an element to the collection.
    ///
    /// # Returns
    ///
    /// Returns `true` if the collection has space left and item is
    /// successfully added, otherwise returns `false`.
    ///
    /// # Complexity
    ///
    /// Constant.
    pub fn push(&mut self, item: T) -> bool {
        if self.items.len() == self.maxsize {
            return false;
        }
        self.items.push(item);
        return true;
    }

    /// # Returns
    ///
    /// Returns the size of collection, indicates how many items are added in
    /// the collection.
    ///
    /// # Note
    ///
    /// Size and capacity are different concepts.
    /// Capacity limits how many items can be stored, while size indicates how
    /// many items is currently stored.
    pub fn size(&self) -> usize {
        self.items.len()
    }

    /// Peeks the last element added without tampering the collection.
    ///
    /// # Returns
    ///
    /// Returns the most recently added item. If nothing was added, `None` will
    /// be returned.
    pub fn peek(&self) -> Option<&T> {
        self.items.last()
    }
}

#[cfg(test)]
mod impl_by_vec {
    use super::*;

    #[test]
    fn new_with_capacity() {
        let stack: Stack<u32> = Stack::with_capacity(10);
        assert_eq!(10, stack.items.capacity());
    }

    #[test]
    fn pop() {
        let mut stack = Stack::with_capacity(1);
        stack.push(1);
        assert_eq!(Some(1), stack.pop());
        assert_eq!(None, stack.pop());
    }

    #[test]
    fn push() {
        let mut stack = Stack::with_capacity(1);
        stack.push(32);
        assert_eq!(Some(&32), stack.peek());
        assert_eq!(1, stack.size());
    }

    #[test]
    fn push_maxsize() {
        let mut stack = Stack::with_capacity(1);
        assert_eq!(true, stack.push(1));
        assert_eq!(Some(&1), stack.peek());
        assert_eq!(false, stack.push(2));
    }

    #[test]
    fn size() {
        let mut stack = Stack::with_capacity(1);
        assert_eq!(0, stack.size());
        stack.push(1);
        assert_eq!(1, stack.size());
    }
}
