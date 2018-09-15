/// A singly-linked list with owned nodes.
///
/// This implementation is a simplified version of `forward_list` in C++.
///
/// Reference:
///
/// - https://doc.rust-lang.org/src/alloc/linked_list.rs.html
/// - http://cglab.ca/~abeinges/blah/too-many-lists/book/README.html
/// - https://stackoverflow.com/questions/51134192/
/// - https://codereview.stackexchange.com/questions/150906
pub struct SinglyLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

/// An owning iterator over the elements of a `SinglyLinkedList`.
///
/// This struct is created by the `into_iter` method on `SinglyLinkedList`.
pub struct IntoIter<T>(SinglyLinkedList<T>);

/// A mutable iterator over the elements of a `SinglyLinkedList`.
///
/// This struct is created by the `iter` method on `SinglyLinkedList`.
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

/// A mutable iterator over the elements of a `SinglyLinkedList`.
///
/// This struct is created by the `iter_mut` method on `SinglyLinkedList`.
pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

/// Internal node representation.
#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Option<Box<Node<T>>>,
}

impl<T> SinglyLinkedList<T> {
    /// Constructs a new, empty `SinglyLinkedList<T>`.
    ///
    /// The list will not allocate until elements are pushed onto it.
    pub fn new() -> Self {
        Self { head: None }
    }

    /// Prepends the given element value to the beginning of the container.
    ///
    /// # Parameters
    ///
    /// * `elem` - The element to prepend.
    ///
    /// # Complexity
    ///
    /// Constant.
    pub fn push_front(&mut self, elem: T) {
        let next = self.head.take();
        self.head = Some(Box::new(Node { elem, next }));
    }

    /// Removes and returns the first element of the container.
    /// If there are no elements in the container, return `None`.
    ///
    /// # Complexity
    ///
    /// Constant.
    pub fn pop_front(&mut self) -> Option<T> {
        let head = self.head.take(); // Take ownership of head;
        match head {
            Some(node) => {
                self.head = node.next;
                Some(node.elem)
            }
            None => None,
        }
    }

    /// Inserts an element after the specified position in the container.
    ///
    /// If the position is out of bound, returns an `Result:Err` with the
    /// size of the list.
    ///
    /// # Parameters
    ///
    /// * `pos` - The index after which the element will be inserted.
    /// * `elem` - The element to insert.
    ///
    /// # Complexity
    ///
    /// Search time O(n) + O(1).
    pub fn insert_after(&mut self, pos: usize, elem: T) -> Result<(), usize> {
        let mut curr = &mut self.head;
        let mut pos_ = pos;

        // Find the node at `pos`.
        while pos_ > 0 {
            curr = match curr.as_mut() {
                Some(node) => &mut node.next,
                None => return Err(pos - pos_),
            };
            pos_ -= 1;
        }

        // Take the ownership of current node.
        match curr.take() {
            Some(mut node) => {
                // Create new node.
                let new_node = Box::new(Node {
                    elem,
                    next: node.next,
                });
                // Re-link new node and current node.
                node.next = Some(new_node);

                // Assign current node back to the list.
                *curr = Some(node);
            }
            None => return Err(pos - pos_)
        }
        Ok(())
    }

    /// Removes and returns an element at specified position from the container.
    ///
    /// # Parameters
    ///
    /// * `pos` - The index at which the element will be moved.
    ///
    /// # Complexity
    ///
    /// Search time O(n) + constant.
    pub fn erase(&mut self, pos: usize) -> Option<T> {
        let mut curr = &mut self.head;
        let mut pos = pos;

        // Find the node at `pos`.
        while pos > 0 {
            curr = match curr.as_mut() {
                Some(node) => &mut node.next,
                None => return None,
            };
            pos -= 1;
        }

        match curr.take() {
            Some(node) => {
                // Assign next node to previous node.next pointer.
                *curr = node.next;
                Some(node.elem)
            }
            None => None
        }
    }

    /// Removes all elements from the container.
    ///
    /// # Complexity
    ///
    /// Linear in the size of the container.
    pub fn clear(&mut self) {
        *self = Self::new();
    }

    ///	Checks whether the container is empty.
    ///
    /// # Complexity
    ///
    /// Constant.
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    /// Gets the number of elements in the container.
    ///
    /// # Complexity
    ///
    /// Linear in the size of the container.
    pub fn len(&self) -> usize {
        self.iter().count()
    }

    /// Reverses the order of the elements in the container.
    ///
    /// # Complexity
    ///
    /// Linear in the size of the container.
    pub fn reverse(&mut self) {
        let mut prev = None;
        let mut curr = self.head.take();
        while let Some(mut node) = curr {
            let next = node.next;
            node.next = prev.take(); // Take ownership from previous node.
            prev = Some(node); // Transfer ownership from current node to previous.
            curr = next; // curr references to next node for next iteration.
        }
        self.head = prev.take();
    }

    /// Creates a iterator that yields immutable refernce of each element.
    pub fn iter(&self) -> Iter<T> {
        Iter { next: self.head.as_ref().map(|node| &**node) }
    }

    /// Creates a iterator that yields mutable refernce of each element.
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut { next: self.head.as_mut().map(|node| &mut **node) }
    }
}

impl<T> Drop for SinglyLinkedList<T> {
    fn drop(&mut self) {
        let mut link = self.head.take();
        while let Some(mut node) = link {
            link = node.next.take(); // Take ownership of next `link` here.
        } // Previous `node` goes out of scope and gets dropped here.
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.next {
            Some(node) => {
                self.next = node.next.as_ref().map(|node| &**node);
                Some(&node.elem)
            }
            None => None,
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.next.take() {
            Some(node) => {
                self.next = node.next.as_mut().map(|node| &mut **node);
                Some(&mut node.elem)
            }
            None => None,
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

impl<T> IntoIterator for SinglyLinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    /// Creates a consuming iterator, that is, one that moves each value out of
    /// the list (from start to end). The list cannot be used after calling this.
    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

impl<T: PartialEq> PartialEq for SinglyLinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }
        self.iter()
            .zip(other.iter())
            .all(|pair| pair.0 == pair.1)
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for SinglyLinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for node in self.iter() {
            write!(f, "{:?} -> ", node)?
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::SinglyLinkedList;

    #[test]
    fn basics() {
        let mut l = SinglyLinkedList::<i32>::new();
        assert_eq!(l.len(), 0);
        assert_eq!(l.pop_front(), None);
        assert_eq!(l.len(), 0);
        assert!(l.is_empty());
    }

    #[test]
    fn push_pop() {
        let mut l = SinglyLinkedList::<i32>::new();
        l.push_front(1);
        l.push_front(2);
        l.push_front(3);

        // Check length and emptiness.
        assert_eq!(l.len(), 3);
        assert!(!l.is_empty());

        // Check removal.
        assert_eq!(l.pop_front(), Some(3));

        // Push more element to check popping correctly.
        l.push_front(4);
        l.push_front(5);
        assert_eq!(l.len(), 4);
        assert_eq!(l.pop_front(), Some(5));
        assert_eq!(l.pop_front(), Some(4));

        // Pop all elements to check exhaustion.
        assert_eq!(l.pop_front(), Some(2));
        assert_eq!(l.pop_front(), Some(1));
        assert_eq!(l.pop_front(), None);
        assert_eq!(l.len(), 0);

        // Check clear works.
        l.push_front(6);
        l.push_front(7);
        l.clear();
        assert!(l.is_empty());
    }

    #[test]
    fn reverse() {
        let mut l = SinglyLinkedList::<i32>::new();
        // 0 elements without crash;
        l.reverse();
        let res = SinglyLinkedList::<i32>::new();
        assert!(l == res);

        let mut l = SinglyLinkedList::<i32>::new();
        l.push_front(1);
        l.reverse();
        let mut res = SinglyLinkedList::<i32>::new();
        res.push_front(1);
        assert!(l == res);

        let mut l = SinglyLinkedList::<i32>::new();
        l.push_front(1);
        l.push_front(2);
        l.reverse();
        let mut res = SinglyLinkedList::<i32>::new();
        res.push_front(2);
        res.push_front(1);
        assert!(l == res);

        let mut l = SinglyLinkedList::<i32>::new();
        l.push_front(1);
        l.push_front(2);
        l.push_front(3);
        l.reverse();
        let mut res = SinglyLinkedList::<i32>::new();
        res.push_front(3);
        res.push_front(2);
        res.push_front(1);
        assert!(l == res);
    }


    #[test]
    fn iterators() {
        // 1. `iter`
        let mut l = SinglyLinkedList::<i32>::new();
        l.push_front(1);
        l.push_front(2);
        l.push_front(3);

        let mut it = l.iter();
        assert_eq!(it.next(), Some(&3));
        assert_eq!(it.next(), Some(&2));
        assert_eq!(it.next(), Some(&1));

        // 2. `iter_mut`
        for elem in l.iter_mut() {
            *elem *= *elem;
        }

        let mut res = SinglyLinkedList::<i32>::new();
        res.push_front(1);
        res.push_front(4);
        res.push_front(9);
        assert_eq!(l, res);

        // 3. `into_iter`
        let collected = l.into_iter().collect::<Vec<i32>>();
        assert_eq!(vec![9, 4, 1], collected);
        // Cannot access `l`. Value moved into `collected`.
    }

    #[test]
    fn insert() {
        let mut l = SinglyLinkedList::<i32>::new();
        l.push_front(1);
        l.push_front(2);
        l.push_front(3);

        // Out of bound! Return the size of the list.
        assert_eq!(l.insert_after(3, 10).unwrap_err(), 3);

        // Insertion succeeded.
        assert!(l.insert_after(0, 10).is_ok());

        let mut res = SinglyLinkedList::<i32>::new();
        res.push_front(1);
        res.push_front(2);
        res.push_front(10);
        res.push_front(3);
        assert_eq!(l, res);

        // Insertion succeeded again.
        assert!(l.insert_after(3, 11).is_ok());

        let mut res = SinglyLinkedList::<i32>::new();
        res.push_front(11);
        res.push_front(1);
        res.push_front(2);
        res.push_front(10);
        res.push_front(3);
        assert_eq!(l, res);
    }

    #[test]
    fn erase() {
        let mut l = SinglyLinkedList::<i32>::new();
        l.push_front(1);
        l.push_front(2);
        l.push_front(3);
        assert!(l.erase(5).is_none());
        assert_eq!(l.erase(1), Some(2));

        // Check remain list is in correct form.
        let mut res = SinglyLinkedList::<i32>::new();
        res.push_front(1);
        res.push_front(3);
        assert_eq!(l, res);

        // Erase all elements
        assert_eq!(l.erase(1), Some(1));
        assert_eq!(l.erase(0), Some(3));
    }
}
