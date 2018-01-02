/// Reference:
/// - http://cglab.ca/~abeinges/blah/too-many-lists/book/README.html
/// - https://doc.rust-lang.org/src/alloc/linked_list.rs.html

type Link = Option<Box<Node>>;
type Element = i32;

pub struct SinglyLinkedList {
    head: Link,
    len: usize,
}

struct Node {
    element: Element,
    next: Link,
}

impl Node {
    fn new(element: Element) -> Self {
        Node {
            element,
            next: None,
        }
    }
}

impl SinglyLinkedList {
    pub fn new() -> Self {
        SinglyLinkedList { head: None, len: 0 }
    }

    pub fn push_front(&mut self, element: Element) {
        let mut node = Box::new(Node::new(element));
        node.next = self.head.take();
        self.head = Some(node);
        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<Element> {
        match self.head.take() {
            None => None,
            Some(boxed) => {
                let node = *boxed; // Dereference from Box.
                self.head = node.next;
                self.len -= 1;
                Some(node.element)
            }
        }
    }

    pub fn clear(&mut self) {
        *self = Self::new();
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl Drop for SinglyLinkedList {
    fn drop(&mut self) {
        let mut link = self.head.take();
        while let Some(mut boxed) = link {
            link = boxed.next.take();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SinglyLinkedList;

    #[test]
    fn basics() {
        let mut l = SinglyLinkedList::new();
        assert_eq!(l.len(), 0);
        assert_eq!(l.pop_front(), None);
        assert_eq!(l.len(), 0);
        assert!(l.is_empty());
    }

    #[test]
    fn push_pop() {

        let mut l = SinglyLinkedList::new();

        // Push 3 element.
        l.push_front(1);
        l.push_front(2);
        l.push_front(3);

        // Check length and emptyness.
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
}
