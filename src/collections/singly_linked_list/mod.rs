/// A singly-linked list with owned nodes.
///
/// This implementation is a simplified version of `std::forward_list` in C++.
///
/// References:
///
/// - [Rust Standard Library: std::collections::LinkedList][1]
/// - [Learning Rust With Entirely Too Many Linked Lists][2]
/// - [Stack Overflow: Deleting a node from singly linked list ...][3]
/// - [Stack Exchange: Reversal of a singly-linked list in Rust][4]
///
/// [1]: https://doc.rust-lang.org/stable/std/collections/struct.LinkedList.html
/// [2]: http://cglab.ca/~abeinges/blah/too-many-lists/book/README.html
/// [3]: https://stackoverflow.com/questions/51134192/
/// [4]: https://codereview.stackexchange.com/questions/150906
// ANCHOR: list_layout
pub struct SinglyLinkedList<T> {
    head: Option<Box<Node<T>>>,
}
// ANCHOR_END: list_layout

/// An owning iterator over the elements of a `SinglyLinkedList`.
///
/// This struct is created by the `into_iter` method on `SinglyLinkedList`.
// ANCHOR: IntoIter_layout
pub struct IntoIter<T>(SinglyLinkedList<T>);
// ANCHOR_END: IntoIter_layout

/// A mutable iterator over the elements of a `SinglyLinkedList`.
///
/// This struct is created by the `iter` method on `SinglyLinkedList`.
// ANCHOR: Iter_layout
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,              // 1
}
// ANCHOR_END: Iter_layout

/// A mutable iterator over the elements of a `SinglyLinkedList`.
///
/// This struct is created by the `iter_mut` method on `SinglyLinkedList`.
// ANCHOR: IterMut_layout
pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}
// ANCHOR_END: IterMut_layout

/// Internal node representation.
#[derive(Debug)]
// ANCHOR: node_layout
struct Node<T> {
    elem: T,
    next: Option<Box<Node<T>>>,
}
// ANCHOR_END: node_layout

impl<T> SinglyLinkedList<T> {
    /// Constructs a new, empty `SinglyLinkedList<T>`.
    ///
    /// The list will not allocate until elements are pushed onto it.
    // ANCHOR: list_new
    pub fn new() -> Self {
        Self { head: None }
    }
    // ANCHOR_END: list_new

    /// Prepends the given element value to the beginning of the container.
    ///
    /// # Parameters
    ///
    /// * `elem` - The element to prepend.
    ///
    /// # Complexity
    ///
    /// Constant.
    // ANCHOR: list_push_front
    pub fn push_front(&mut self, elem: T) {
        let next = self.head.take();                     // 1
        self.head = Some(Box::new(Node { elem, next })); // 2
    }
    // ANCHOR_END: list_push_front

    /// Removes and returns the first element of the container.
    /// If there are no elements in the container, return `None`.
    ///
    /// # Complexity
    ///
    /// Constant.
    // ANCHOR: list_pop_front
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
    // ANCHOR_END: list_pop_front

    /// Inserts an element after the specified position in the container.
    ///
    /// If the position is out of bound, returns an `Result:Err` with the
    /// size of the list.
    ///
    /// # Parameters
    ///
    /// * `pos` - The index after which the element will be inserted.
    /// * `elem` - The element to be inserted.
    ///
    /// # Complexity
    ///
    /// Search time O(n) + O(1).
    // ANCHOR: list_insert_after
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
            None => return Err(pos - pos_),
        }
        Ok(())
    }
    // ANCHOR_END: list_insert_after

    /// Removes and returns an element at specified position from the container.
    ///
    /// # Parameters
    ///
    /// * `pos` - The index at which the element will be moved.
    ///
    /// # Complexity
    ///
    /// Search time O(n) + constant.
    // ANCHOR: list_remove
    pub fn remove(&mut self, pos: usize) -> Option<T> {
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
            None => None,
        }
    }
    // ANCHOR_END: list_remove

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
    // ANCHOR: list_reverse
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
    // ANCHOR_END: list_reverse

    /// Creates an iterator that yields immutable reference of each element.
    pub fn iter(&self) -> Iter<T> {
    // ANCHOR: list_iter
        Iter {
            next: self.head.as_deref(),
        }
    }
    // ANCHOR_END: list_iter

    /// Creates an iterator that yields mutable reference of each element.
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

// ANCHOR: list_drop
impl<T> Drop for SinglyLinkedList<T> {
    fn drop(&mut self) {
        let mut link = self.head.take();
        while let Some(mut node) = link {
            link = node.next.take(); // Take ownership of next `link` here.
        } // Previous `node` goes out of scope and gets dropped here.
    }
}
// ANCHOR_END: list_drop

// ANCHOR: Iter
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next {
            Some(node) => {
                self.next = node.next.as_deref();
                Some(&node.elem)
            }
            None => None,
        }
    }
}
// ANCHOR_END: Iter

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.next.take() {
            Some(node) => {
                self.next = node.next.as_deref_mut();
                Some(&mut node.elem)
            }
            None => None,
        }
    }
}

// ANCHOR: IntoIter
impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}
// ANCHOR_END: IntoIter

// ANCHOR: IntoIterator
impl<T> IntoIterator for SinglyLinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    /// Creates a consuming iterator, that is, one that moves each value out of
    /// the list (from start to end). The list cannot be used after calling this.
    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}
// ANCHOR_END: IntoIterator

// ANCHOR: PartialEq
impl<T: PartialEq> PartialEq for SinglyLinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }
        self.iter().zip(other.iter()).all(|pair| pair.0 == pair.1)
    }
}
// ANCHOR_END: PartialEq

// ANCHOR: Debug
impl<T: std::fmt::Debug> std::fmt::Debug for SinglyLinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for node in self.iter() {
            write!(f, "{:?} -> ", node)?
        }
        Ok(())
    }
}
// ANCHOR_END: Debug

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
    fn iter() {
        let mut l = SinglyLinkedList::<i32>::new();
        l.push_front(1);
        l.push_front(2);
        l.push_front(3);

        let mut it = l.iter();
        assert_eq!(it.next(), Some(&3));
        assert_eq!(it.next(), Some(&2));
        assert_eq!(it.next(), Some(&1));
    }

    #[test]
    fn iter_mut() {
        let mut l = SinglyLinkedList::<i32>::new();
        l.push_front(1);
        l.push_front(2);
        l.push_front(3);

        for elem in l.iter_mut() {
            *elem *= *elem;
        }

        let mut res = SinglyLinkedList::<i32>::new();
        res.push_front(1);
        res.push_front(4);
        res.push_front(9);
        assert_eq!(l, res);
    }

    #[test]
    fn into_iter() {
        let mut l = SinglyLinkedList::<i32>::new();
        l.push_front(1);
        l.push_front(2);
        l.push_front(3);

        // 3. `into_iter`
        let collected = l.into_iter().collect::<Vec<i32>>();
        assert_eq!(vec![3, 2, 1], collected);
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
    fn remove() {
        let mut l = SinglyLinkedList::<i32>::new();
        l.push_front(1);
        l.push_front(2);
        l.push_front(3);
        assert!(l.remove(5).is_none());
        assert_eq!(l.remove(1), Some(2));

        // Check remain list is in correct form.
        let mut res = SinglyLinkedList::<i32>::new();
        res.push_front(1);
        res.push_front(3);
        assert_eq!(l, res);

        // Remove all elements
        assert_eq!(l.remove(1), Some(1));
        assert_eq!(l.remove(0), Some(3));
    }
}
