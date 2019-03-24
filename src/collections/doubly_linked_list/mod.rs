use std::rc::Rc;
use std::cell::RefCell;
use std::cell::{Ref, RefMut};

/// A doubly-linked list with owned nodes.
///
/// This implementation is a simplified version of `std::list` in C++.
///
/// References:
///
/// - [Rust Standard Library: std::collections::LinkedList][1]
/// - [Learning Rust With Entirely Too Many Linked Lists][2]
///
/// [1]: https://doc.rust-lang.org/stable/std/collections/struct.VecDeque.html
/// [2]: http://cglab.ca/~abeinges/blah/too-many-lists/book/README.html
#[derive(Debug)]
pub struct DoublyLinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

/// An owning iterator over the elements of a `DoublyLinkedList`.
///
/// This struct is created by the `into_iter` method on `DoublyLinkedList`.
pub struct IntoIter<T>(DoublyLinkedList<T>);

/// A mutable iterator over the elements of a `DoublyLinkedList`.
///
/// This struct is created by the `iter` method on `DoublyLinkedList`.
pub struct Iter<'a, T> {
   next: Link<T>,
   phantomData: std::marker::PhantomData<&'a T>,
}

/// Internal node representation.
#[derive(Debug)]
struct Node<T> {
    elem: T,
    prev: Link<T>,
    next: Link<T>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            elem,
            prev: None,
            next: None,
        }))
    }
}

impl<T> DoublyLinkedList<T> {
    /// Constructs a new, empty `DoublyLinkedList<T>`.
    ///
    /// The list will not allocate until elements are pushed onto it.
    pub fn new() -> Self {
        Self { head: None, tail: None }
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
        // Ensure:
        // * +2 links to new head
        // * +0 to the others
        let new = Node::new(elem);
        match self.head.take() {                            // -1 old
            Some(old) => {
                old.borrow_mut().prev = Some(new.clone());  // +1 new
                new.borrow_mut().next = Some(old);          // +1 old
                self.head = Some(new);                      // +1 new
                // Counts: +2 new, +0 old
            },
            None => {
                self.head = Some(new.clone());              // +1 new
                self.tail = Some(new);                      // +1 new
                // Counts: +2 new, no old
            }
        }
    }

    /// Appends the given element value to the end of the container.
    ///
    /// # Parameters
    ///
    /// * `elem` - The element to append.
    ///
    /// # Complexity
    ///
    /// Constant.
    pub fn push_back(&mut self, elem: T) {
        // Ensure:
        // * +2 links to new tail
        // * +0 to the others
        let new = Node::new(elem);
        match self.tail.take() {                            // -1 old
            Some(old) => {
                old.borrow_mut().next = Some(new.clone());  // +1 new
                new.borrow_mut().prev = Some(old);          // +1 old
                self.tail = Some(new);                      // +1 new
                // Counts: +2 new, +0 old
            },
            None => {
                self.head = Some(new.clone());              // +1 new
                self.tail = Some(new);                      // +1 new
                // Counts: +2 new, no old
            }
        }
    }

    /// Removes and returns the first element of the container.
    /// If there are no elements in the container, return `None`.
    ///
    /// # Complexity
    ///
    /// Constant.
    pub fn pop_front(&mut self) -> Option<T> {
        // Ensure:
        // * -2 links to old head
        // * +0 to the others
        self.head.take().and_then(|old| {           // -1 old
            match old.borrow_mut().next.take() {    // -1 new
                Some(new) => {
                    new.borrow_mut().prev.take();   // -1 old
                    self.head = Some(new);          // +1 new
                    // Counts: +0 new, -2 old
                },
                None => {
                    self.tail.take();               // -1 old
                    // Counts: no new, -2 old
                },
            };
            // Assertion to ensure there is no reference cycle.
            assert_eq!(Rc::strong_count(&old), 1);
            Rc::try_unwrap(old).ok().map(|old| old.into_inner().elem)
        })
    }

    /// Removes and returns the last element of the container.
    /// If there are no elements in the container, return `None`.
    ///
    /// # Complexity
    ///
    /// Constant.
    pub fn pop_back(&mut self) -> Option<T> {
        // Ensure:
        // * -2 links to old tail
        // * +0 to the others
        self.tail.take().and_then(|old| {           // -1 old
            match old.borrow_mut().prev.take() {    // -1 new
                Some(new) => {
                    new.borrow_mut().next.take();   // -1 old
                    self.tail = Some(new);          // +1 new
                    // Counts: +0 new, -2 old
                },
                None => {
                    self.tail.take();               // -1 old
                    // Counts: no new, -2 old
                },
            };
            Rc::try_unwrap(old).ok().map(|old| old.into_inner().elem)
        })
    }

    /// Inserts an element at specified position in the container. Shift
    /// all elements with greater of equal indices towards the back.
    ///
    /// If the position is out of bound, returns an `Result:Err` with the
    /// size of the list.
    ///
    /// # Parameters
    ///
    /// * `pos` - The index at which the element will be inserted.
    /// * `elem` - The element to be inserted.
    ///
    /// # Complexity
    ///
    /// Search time O(n) + O(1).
    pub fn insert(&mut self, pos: usize, elem: T) -> Result<(), usize> {
        // Ensures +0 current node
        let mut curr = self.head.clone(); // +1 curr
        let mut pos_ = pos;

        // Finds the node at `pos`.
        while pos_ > 0 {
            curr = match curr {
                Some(node) => node.borrow_mut().next.clone(), // +1 curr.next
                None => return Err(pos - pos_),
            }; // -1 curr (reassign)
            pos_ -= 1;
        }

        if curr.is_none() {
            return Err(pos - pos_);
        }

        curr.map(|next| { // -1 curr (move into closure)
            let new = Node::new(elem);
            // Links each nodes.
            // Ensures +0 prev, +0 next, and +2 new.
            new.borrow_mut().prev = next.borrow_mut().prev.take(); // -1+1 prev
            new.borrow_mut().next = Some(next.clone()); // +1 next
            new.borrow_mut().prev.as_mut().map_or_else(
                || self.head = Some(new.clone()), // -1 next, +1 new (insertion of first element)
                |prev| prev.borrow_mut().next = Some(new.clone()), // -1 next, +1 new
            );
            next.borrow_mut().prev = Some(new); // +1 new
            // Counts: +0 prev, +0 next, +2 new
        }).ok_or(pos - pos_) // Returns the size of the list.
        // Counts: +0 curr
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
    pub fn remove(&mut self, pos: usize) -> Option<T> {
        // Ensures -2 current node
        let mut curr = self.head.clone(); // +1 curr
        let mut pos_ = pos;

        // Finds the node at `pos`.
        while pos_ > 0 {
            curr = match curr {
                Some(node) => node.borrow_mut().next.clone(), // +1 curr.next
                None => return None,
            }; // -1 curr (reassign)
            pos_ -= 1;
        }


        // Un-links current node from other links.
        curr.and_then(|node| {
            let mut prev = node.borrow_mut().prev.take();
            let mut next = node.borrow_mut().next.take();
            prev.as_mut().map_or_else(
                || self.head = next.clone(),
                |prev| prev.borrow_mut().next = next.clone(),
            );
            next.as_mut().map_or_else(
                || self.tail = prev.clone(),
                |next| next.borrow_mut().prev = prev.clone(),
            );

            Rc::try_unwrap(node).ok().map(|node| node.into_inner().elem) // -1 curr
            // Counts: -2 curr
        })
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
        unimplemented!();
    }

    /// Gets the number of elements in the container.
    ///
    /// # Complexity
    ///
    /// Linear in the size of the container.
    pub fn len(&self) -> usize {
        unimplemented!();
    }

   /// Creates an iterator that yields immutable reference of each element.
   pub fn iter(&self) -> Iter<T> {
       Iter { next: self.head, phantomData: std::marker::PhantomData }
   }
}

impl<T> Drop for DoublyLinkedList<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = Ref<'a, T>;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.as_ref().map(|cell| {
            Ref::map(cell.borrow(), |node| &node.elem)
        })
    }
}

//impl<'a, T> Iterator for IterMut<'a, T> {
//    type Item = &'a mut T;
//    fn next(&mut self) -> Option<Self::Item> {
//        match self.next.take() {
//            Some(node) => {
//                self.next = node.next.as_mut().map(|node| &mut **node);
//                Some(&mut node.elem)
//            }
//            None => None,
//        }
//    }
//}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<T> {
        self.0.pop_back()
    }
}

impl<T> IntoIterator for DoublyLinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    /// Creates a consuming iterator, that is, one that moves each value out of
    /// the list (from start to end). The list cannot be used after calling this.
    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

#[cfg(test)]
mod tests {
    use super::DoublyLinkedList;

    #[ignore]
    #[test]
    fn basics() {
        let mut l = DoublyLinkedList::<i32>::new();
//         assert_eq!(l.len(), 0);
//         assert_eq!(l.pop_front(), None);
//         assert_eq!(l.len(), 0);
//         assert!(l.is_empty());
    }

    #[ignore]
    #[test]
    fn push_pop() {
    }

    #[ignore]
    #[test]
    fn reverse() {
    }

    #[test]
    fn insert() {
        // A list containing only 1 element.
        let mut l = DoublyLinkedList::<i32>::new();
        l.push_front(1);
        assert_eq!(l.insert(1, 2), Err(1), "insert at [1] on an 1-element list");
        assert!(l.insert(0, 2).is_ok(), "insert at [0] on an 1-element list");
        // Check remain list is in correct form.
//        let mut res = DoublyLinkedList::<i32>::new();
//        res.push_front(1);
//        res.push_front(3);
//        assert_eq!(l, res);

        // A list containing 4 elements.
        let mut l = DoublyLinkedList::<i32>::new();
        l.push_front(1);
        l.push_front(2);
        l.push_front(3);
        l.push_front(4);
        assert_eq!(l.insert(4, 10), Err(4), "insertion position out of bound");
        assert!(l.insert(3, 10).is_ok(), "insert at the last position");
        assert!(l.insert(0, 20).is_ok(), "insert at the first position");
        // Check remain list is in correct form.
//        let mut res = DoublyLinkedList::<i32>::new();
//        res.push_front(1);
//        res.push_front(3);
//        assert_eq!(l, res);
    }

    #[test]
    fn remove() {
        // A list containing only 1 element.
        let mut l = DoublyLinkedList::<i32>::new();
        l.push_front(1);
        assert_eq!(l.remove(0), Some(1));
        assert!(l.remove(0).is_none());

        // A list containing 4 elements.
        let mut l = DoublyLinkedList::<i32>::new();
        l.push_front(1);
        l.push_front(2);
        l.push_front(3);
        l.push_front(4);
        assert!(l.remove(4).is_none());
        // Remove from tail.
        assert_eq!(l.remove(3), Some(1));
        // Check remain list is in correct form.
//        let mut res = DoublyLinkedList::<i32>::new();
//        res.push_front(1);
//        res.push_front(3);
//        assert_eq!(l, res);

        // Remove from head.
        assert_eq!(l.remove(0), Some(4));
//        let mut res = DoublyLinkedList::<i32>::new();
//        res.push_front(1);
//        res.push_front(3);
//        assert_eq!(l, res);

        // Remove remain elements
        assert_eq!(l.remove(0), Some(3));
        assert_eq!(l.remove(0), Some(2));
    }

    #[test]
    fn into_iter() {
        let mut l = DoublyLinkedList::<i32>::new();
        l.push_back(1);
        l.push_back(2);
        l.push_back(3);

        // `into_iter`
        let collected = l.into_iter().collect::<Vec<i32>>();
        assert_eq!(vec![1, 2, 3], collected);
        // Cannot access `l`. Value moved into `collected`.

        let mut l = DoublyLinkedList::<i32>::new();
        l.push_back(1);
        l.push_back(2);
        l.push_back(3);

        let mut iter = l.into_iter();
        assert_eq!(iter.next_back(), Some(3));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next(), None);
    }
}
