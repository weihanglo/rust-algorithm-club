use core::ops::{Index, IndexMut};
use core::{fmt, mem, ptr, slice};
use std::alloc::{alloc, dealloc, handle_alloc_error, realloc, Layout};

/// A double-ended queue (abbreviated to _deque_), for which elements can be
/// added or remove from both back and front ends.
///
/// Underneath the hood, this [`Deque`] uses a contiguous memory block as a ring
/// buffer to store values.
///
/// References:
///
/// - [Rust Standard Library: std::collections::VecDeque][1]
/// - [Wikipedia: Circular buffer][2]
///
/// [1]: `std::collections::VecDeque`
/// [2]: https://en.wikipedia.org/wiki/Circular_buffer
// ANCHOR: layout
pub struct Deque<T> {
    tail: usize,
    head: usize,
    ring_buf: RawVec<T>,
}
// ANCHOR_END: layout

/// For demo purpose, set default capacity to 1 in order to trigger
/// buffer expansions easily. This value must be power of 2.
const DEFAULT_CAPACITY: usize = 1;

impl<T> Deque<T> {
    /// Constructs a new, empty [`Deque<T>`].
    ///
    /// For convenience, the deque initially allocates a region of a single `T`.
    // ANCHOR: new
    pub fn new() -> Self {
        Self {
            tail: 0,
            head: 0,
            ring_buf: RawVec::with_capacity(DEFAULT_CAPACITY),
        }
    }
    // ANCHOR_END: new

    /// Prepends the given element value to the beginning of the container.
    ///
    /// # Parameters
    ///
    /// * `elem` - The element to prepend.
    ///
    /// # Complexity
    ///
    /// Constant.
    // ANCHOR: push_front
    pub fn push_front(&mut self, elem: T) {
        self.try_grow(); // 1
        self.tail = self.wrapping_sub(self.tail, 1); // 2

        // This is safe because the offset is wrapped inside valid memory region.
        unsafe { self.ptr().add(self.tail).write(elem) } // 3
    }
    // ANCHOR_END: push_front

    /// Appends the given element value to the end of the container.
    ///
    /// # Parameters
    ///
    /// * `elem` - The element to append.
    ///
    /// # Complexity
    ///
    /// Constant.
    // ANCHOR: push_back
    pub fn push_back(&mut self, elem: T) {
        self.try_grow(); // 1
        let head = self.head;
        self.head = self.wrapping_add(self.head, 1); // 2

        // This is safe because the offset is wrapped inside valid memory region.
        unsafe { self.ptr().add(head).write(elem) } // 3
    }
    // ANCHOR_END: push_back

    /// Removes and returns the first element of the container.
    /// If there are no elements in the container, return `None`.
    ///
    /// # Complexity
    ///
    /// Constant.
    // ANCHOR: pop_front
    pub fn pop_front(&mut self) -> Option<T> {
        if self.is_empty() {
            return None; // 1
        }

        let tail = self.tail;
        self.tail = self.wrapping_add(self.tail, 1); // 2

        // This is safe because the offset is wrapped inside valid memory region.
        unsafe { Some(self.ptr().add(tail).read()) } // 3
    }
    // ANCHOR_END: pop_front

    /// Removes and returns the last element of the container.
    /// If there are no elements in the container, return `None`.
    ///
    /// # Complexity
    ///
    /// Constant.
    // ANCHOR: pop_back
    pub fn pop_back(&mut self) -> Option<T> {
        if self.is_empty() {
            return None; // 1
        }

        self.head = self.wrapping_sub(self.head, 1); // 2

        // This is safe because the offset is wrapped inside valid memory region.
        unsafe { Some(self.ptr().add(self.head).read()) } // 3
    }
    // ANCHOR_END: pop_back

    /// Peeks the first element of the container.
    /// If there are no elements in the container, return `None`.
    ///
    /// # Complexity
    ///
    /// Constant.
    // ANCHOR: front
    pub fn front(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }
        // This is safe because the offset is wrapped inside valid memory region.
        unsafe { Some(&*self.ptr().add(self.tail)) }
    }
    // ANCHOR_END: front

    /// Peeks the last element of the container.
    /// If there are no elements in the container, return `None`.
    ///
    /// # Complexity
    ///
    /// Constant.
    // ANCHOR: back
    pub fn back(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }
        let head = self.wrapping_sub(self.head, 1);
        // This is safe because the offset is wrapped inside valid memory region.
        unsafe { Some(&*self.ptr().add(head)) }
    }
    // ANCHOR_END: back

    ///	Checks whether the container is empty.
    ///
    /// # Complexity
    ///
    /// Constant.
    // ANCHOR: is_empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    // ANCHOR_END: is_empty

    ///	Gets the number of elements in the container.
    ///
    /// # Complexity
    ///
    /// Constant.
    // ANCHOR: len
    pub fn len(&self) -> usize {
        self.head.wrapping_sub(self.tail) & (self.cap() - 1)
    }
    // ANCHOR_END: len

    /// Creates an iterator that yields immutable reference of each element.
    // ANCHOR: iter
    pub fn iter(&self) -> Iter<T> {
        Iter {
            head: self.head,
            tail: self.tail,
            // This is safe because only initialized contents would be accessed.
            ring_buf: unsafe { self.ring_buf.as_slice() },
        }
    }
    // ANCHOR_END: iter

    /// Creates an iterator that yields mutable reference of each element.
    // ANCHOR: iter_mut
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            head: self.head,
            tail: self.tail,
            // This is safe because only initialized contents would be accessed.
            ring_buf: unsafe { self.ring_buf.as_mut_slice() },
        }
    }
    // ANCHOR_END: iter_mut

    /// Checks if underlying ring buffer is full.
    // ANCHOR: is_full
    fn is_full(&self) -> bool {
        self.cap() - self.len() == 1
    }
    // ANCHOR_END: is_full

    /// Resizes the underlying ring buffer if necessary.
    ///
    /// This method simply makes the ring buffer contiguous for the beneath
    /// scenario (tail > head). For a thorough Implementation, please refer to
    /// [`VecDeque::handle_capacity_increase`][1].
    ///
    /// ```console,ignore
    /// Before:
    ///          h   t
    /// [o o o o x x o o]
    ///
    /// Resize:
    ///          h   t
    /// [o o o o x x o o | x x x x x x x x]
    ///
    /// Copy:
    ///              t           h
    /// [x x x x x x o o | o o o o x x x x]
    ///  _ _ _ _           _ _ _ _
    /// ```
    ///
    /// # Complexity
    ///
    /// Linear in the size of the container.
    ///
    /// [1]: https://github.com/rust-lang/rust/blob/07194ff/library/alloc/src/collections/vec_deque/mod.rs#L405-L447
    // ANCHOR: try_grow
    fn try_grow(&mut self) {
        if self.is_full() {
            let old_cap = self.cap(); // 1
            self.ring_buf.try_grow(); // 2

            // 3
            if self.tail > self.head {
                // The content of ring buffer won't overlapped, so it's safe to
                // call `copy_nonoverlapping`. It's also safe to advance the
                // pointer by `old_cap` since the buffer has been doubled.
                let src = self.ptr(); // 4-1
                let dst = unsafe { self.ptr().add(old_cap) }; // 4-2
                unsafe {
                    ptr::copy_nonoverlapping(src, dst, self.head);
                }
                self.head += old_cap; // 5
            }
        }
    }
    // ANCHOR_END: try_grow

    /// Returns the actual index of the underlying ring buffer for a given
    /// logical index + addend.
    // ANCHOR: wrapping_add
    fn wrapping_add(&self, index: usize, addend: usize) -> usize {
        wrap_index(index.wrapping_add(addend), self.cap())
    }
    // ANCHOR_END: wrapping_add

    /// Returns the actual index of the underlying ring buffer for a given
    /// logical index - subtrahend.
    // ANCHOR: wrapping_sub
    fn wrapping_sub(&self, index: usize, subtrahend: usize) -> usize {
        wrap_index(index.wrapping_sub(subtrahend), self.cap())
    }
    // ANCHOR_END: wrapping_sub

    /// An abstraction for accessing the pointer of the ring buffer.
    // ANCHOR: ptr
    fn ptr(&self) -> *mut T {
        self.ring_buf.ptr
    }
    // ANCHOR_END: ptr

    /// An abstraction for accessing the capacity of the ring buffer.
    // ANCHOR: cap
    fn cap(&self) -> usize {
        self.ring_buf.cap()
    }
    // ANCHOR_END: cap
}

// ANCHOR: Drop
impl<T> Drop for Deque<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop_back() {}
    }
}
// ANCHOR_END: Drop

/// Returns the actual index of the underlying ring buffer for a given logical index.
///
/// To ensure all bits of `size - 1` is set to 1, here the size must always be
/// power of two.
// ANCHOR: wrap_index
fn wrap_index(index: usize, size: usize) -> usize {
    debug_assert!(size.is_power_of_two());
    index & (size - 1)
}
// ANCHOR_END: wrap_index

/// An immutable iterator over the elements of a [`Deque`].
///
/// This struct is created by the `iter` method on [`Deque`].
// ANCHOR: Iter_layout
pub struct Iter<'a, T> {
    head: usize,
    tail: usize,
    ring_buf: &'a [T],
}
// ANCHOR_END: Iter_layout

// ANCHOR: Iter
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.tail == self.head {
            return None; // 1
        }
        let tail = self.tail; // 2
        self.tail = wrap_index(self.tail.wrapping_add(1), self.ring_buf.len()); // 3
        self.ring_buf.get(tail) // 4
    }
}
// ANCHOR_END: Iter

/// A mutable iterator over the elements of a [`Deque`].
///
/// This struct is created by the `iter_mut` method on [`Deque`].
// ANCHOR: IterMut_layout
pub struct IterMut<'a, T> {
    head: usize,
    tail: usize,
    ring_buf: &'a mut [T],
}
// ANCHOR_END: IterMut_layout

// ANCHOR: IterMut
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.tail == self.head {
            return None;
        }
        let tail = self.tail;
        self.tail = wrap_index(self.tail.wrapping_add(1), self.ring_buf.len());
        // This unsafe block is needed for solving the limitation of Iterator
        // trait: the `&mut self` is bound to an anonymous lifetime which rustc
        // cannot figure out whether it would outlive returning element. Hence
        // the explicit pointer casting is required.
        
            let ptr = self.ring_buf as *mut [T]; // 1
            let slice = unsafe { &mut *ptr }; // 2
            slice.get_mut(tail) // 3
        
    }
}
// ANCHOR_END: IterMut

/// An owning iterator over the elements of a [`Deque`].
///
/// This struct is created by the `into_iter` method on [`Deque`].
// ANCHOR: IntoIter_layout
pub struct IntoIter<T>(Deque<T>);
// ANCHOR_END: IntoIter_layout

// ANCHOR: IntoIter
impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}
// ANCHOR_END: IntoIter

// ANCHOR: IntoIterator
impl<T> IntoIterator for Deque<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}
// ANCHOR_END: IntoIterator

// ANCHOR: IntoIterator_ref
impl<'a, T> IntoIterator for &'a Deque<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Deque<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
// ANCHOR_END: IntoIterator_ref

// ANCHOR: Index
impl<T> Index<usize> for Deque<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.len(), "Out of bound");
        let index = self.wrapping_add(self.tail, index);
        // This is safe because the offset is wrapped inside valid memory region.
        unsafe { &*self.ptr().add(index) }
    }
}
// ANCHOR_END: Index

// ANCHOR: IndexMut
impl<T> IndexMut<usize> for Deque<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        assert!(index < self.len(), "Out of bound");
        let index = self.wrapping_add(self.tail, index);
        // This is safe because the offset is wrapped inside valid memory region.
        unsafe { &mut *self.ptr().add(index) }
    }
}
// ANCHOR_END: IndexMut

// ANCHOR: Debug
impl<T: fmt::Debug> fmt::Debug for Deque<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}
// ANCHOR_END: Debug

/// A growable, contiguous heap memory allocation that stores homogeneous elements.
///
/// This type can be seen as a simplified version of [`RawVec`] inside Rust
/// Standard Library. Use at your own risk.
///
/// [`RawVec`]: https://github.com/rust-lang/rust/blob/ff6ee2a7/library/alloc/src/raw_vec.rs
#[derive(Debug)]
// ANCHOR: RawVec
struct RawVec<T> {
    ptr: *mut T,
    cap: usize,
}
// ANCHOR_END: RawVec

impl<T> RawVec<T> {
    /// Allocates on the heap with a certain capacity.
    ///
    /// The `cap` argument would be ignored when allocating zero sized types
    /// and use zero instead. The caller would see a quite large capacity.
    /// See [`RawVec::cap`] for more.
    // ANCHOR: RawVec_with_capacity
    pub fn with_capacity(cap: usize) -> Self {
        let layout = Layout::array::<T>(cap).unwrap(); // 1

        // 2
        if layout.size() == 0 {
            // This is safe for zero sized types. However, be careful when facing
            // zero capacity layouts. It must be replaced with an actual pointer
            // before operations such as dereference or read/write.
            let ptr = ptr::NonNull::dangling().as_ptr(); // 3
            Self { ptr, cap: 0 }
        } else {
            // This is safe because it conforms to the [safety contracts][1].
            //
            // [1]: https://doc.rust-lang.org/1.49.0/alloc/alloc/trait.GlobalAlloc.html#safety-1
            let ptr = unsafe { alloc(layout) }; // 4
            if ptr.is_null() {
                handle_alloc_error(layout);
            }
            Self {
                ptr: ptr.cast(),
                cap,
            }
        }
    }
    // ANCHOR_END: RawVec_with_capacity

    /// Doubles the size of the memory region.
    ///
    /// This method maybe only reallocates non-zero sized types. Non-zero sized
    /// types would not grow since they are not actually allocated.
    // ANCHOR: RawVec_try_grow
    pub fn try_grow(&mut self) {
        if mem::size_of::<T>() == 0 {
            return; // 1
        }

        if self.cap == 0 {
            *self = Self::with_capacity(1); // 2
            return;
        }

        let old_layout = Layout::array::<T>(self.cap).unwrap(); // 3
        let new_cap = self.cap << 1;
        let new_size = old_layout.size() * new_cap;
        // This is safe because it conforms to the [safety contracts][1].
        //
        // [1]: https://doc.rust-lang.org/1.49.0/alloc/alloc/trait.GlobalAlloc.html#safety-4
        let ptr = unsafe { realloc(self.ptr.cast(), old_layout, new_size) };
        if ptr.is_null() {
            handle_alloc_error(old_layout);
        }
        // ...Old allocation is unusable and may be released from here at anytime.

        self.ptr = ptr.cast(); // 4
        self.cap = new_cap;
    }
    // ANCHOR_END: RawVec_try_grow

    /// Gets the capacity of the allocation.
    ///
    /// If `T` is zero sized, this will always be the largest possible power of
    /// two of `usize`. Currently `(usize::MAX + 1) / 2`.
    ///
    /// Ref: [https://github.com/rust-lang/rust/blob/f7534b/library/alloc/src/collections/vec_deque/mod.rs#L61]
    // ANCHOR: RawVec_cap
    pub fn cap(&self) -> usize {
        if mem::size_of::<T>() == 0 {
            1usize << (mem::size_of::<usize>() * 8 - 1)
        } else {
            self.cap
        }
    }
    // ANCHOR_END: RawVec_cap

    /// Returns an immutable slice of underlying allocation.
    ///
    /// This is unsafe because the slice may not have all its contents initialized.
    // ANCHOR: RawVec_as_slice
    pub unsafe fn as_slice(&self) -> &[T] {
        slice::from_raw_parts(self.ptr.cast(), self.cap())
    }
    // ANCHOR_END: RawVec_as_slice

    /// Returns a mutable slice of underlying allocation.
    ///
    /// This is unsafe because the slice may not have all its contents initialized.
    // ANCHOR: RawVec_as_mut_slice
    pub unsafe fn as_mut_slice(&self) -> &mut [T] {
        slice::from_raw_parts_mut(self.ptr.cast(), self.cap())
    }
    // ANCHOR_END: RawVec_as_mut_slice
}

// ANCHOR: RawVec_drop
impl<T> Drop for RawVec<T> {
    /// Deallocates the underlying memory region by calculating the type layout
    /// and number of elements.
    ///
    /// This only drop the memory block allocated by `RawVec` itself but not
    /// dropping the contents. Callers need to drop the contents by themselves.
    fn drop(&mut self) {
        let layout = Layout::array::<T>(self.cap).unwrap(); // 1
        if layout.size() > 0 {
            // This is safe because it conforms to the [safety contracts][1].
            //
            // [1]: https://doc.rust-lang.org/1.49.0/alloc/alloc/trait.GlobalAlloc.html#safety-2
            unsafe { dealloc(self.ptr.cast(), layout) }
        }
    }
}
// ANCHOR_END: RawVec_drop

#[cfg(test)]
mod deque {
    use super::Deque;

    #[test]
    fn push_pop() {
        let mut d = Deque::new();
        assert_eq!(d.len(), 0);
        assert_eq!(d.front(), None);
        assert_eq!(d.back(), None);

        d.push_back(1);
        d.push_back(2);
        // [1, 2]
        assert_eq!(d.len(), 2);
        assert_eq!(d.front(), Some(&1));
        assert_eq!(d.back(), Some(&2));

        d.push_front(3);
        d.push_front(4);
        // [4, 3, 1, 2]
        assert_eq!(d.len(), 4);
        assert_eq!(d.front(), Some(&4));
        assert_eq!(d.back(), Some(&2));

        assert_eq!(d.pop_front(), Some(4));
        assert_eq!(d.pop_front(), Some(3));
        assert_eq!(d.pop_front(), Some(1));
        assert_eq!(d.pop_front(), Some(2));
        assert_eq!(d.pop_front(), None);
        assert_eq!(d.len(), 0);
        assert_eq!(d.front(), None);
        assert_eq!(d.back(), None);

        d.push_front(5);
        d.push_front(6);
        // [6, 5]
        assert_eq!(d.len(), 2);
        assert_eq!(d.front(), Some(&6));
        assert_eq!(d.back(), Some(&5));

        assert_eq!(d.pop_back(), Some(5));
        assert_eq!(d.pop_back(), Some(6));
        assert_eq!(d.pop_back(), None);
        assert_eq!(d.len(), 0);
        assert_eq!(d.front(), None);
        assert_eq!(d.back(), None);
    }

    #[test]
    fn iter() {
        let mut d = Deque::new();
        d.push_back(1);
        d.push_back(2);
        d.push_front(3);
        d.push_front(4);
        d.push_front(5);
        d.push_front(6);
        // [6, 5, 4, 3, 1, 2]

        let mut iter = d.iter();
        assert_eq!(iter.next(), Some(&6));
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut d = Deque::new();
        d.push_back(1);
        d.push_back(2);
        d.push_front(3);
        d.push_front(4);
        // [4, 3, 1, 2]

        for elem in d.iter_mut() {
            *elem *= *elem;
        }

        let mut iter = d.iter_mut();
        assert_eq!(iter.next(), Some(&mut 16));
        assert_eq!(iter.next(), Some(&mut 9));
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), Some(&mut 4));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn into_iter() {
        let mut d = Deque::new();
        d.push_back(1);
        d.push_back(2);
        d.push_front(3);
        d.push_front(4);
        // [4, 3, 1, 2]

        let l = d.into_iter().collect::<Vec<_>>();
        assert_eq!(&[4, 3, 1, 2], &l[..]);

        let mut d = Deque::new();
        d.push_back(1);
        d.push_back(2);
        d.push_front(3);
        d.push_front(4);
        // [4, 3, 1, 2]
        let mut l = vec![];
        for elem in &d {
            l.push(elem);
        }
        assert_eq!(&[&4, &3, &1, &2], &l[..]);

        let mut d = Deque::new();
        d.push_back(1);
        d.push_back(2);
        d.push_front(3);
        d.push_front(4);
        // [4, 3, 1, 2]

        for elem in &mut d {
            *elem *= *elem;
        }

        let mut iter = d.iter_mut();
        assert_eq!(iter.next(), Some(&mut 16));
        assert_eq!(iter.next(), Some(&mut 9));
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), Some(&mut 4));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn index() {
        let mut d = Deque::new();
        d.push_back(1);
        d.push_back(2);
        d.push_front(3);
        d.push_front(4);
        // [4, 3, 1, 2]

        for i in 0..d.len() {
            d[i] *= d[i];
        }
        assert_eq!(d[0], 16);
        assert_eq!(d[1], 9);
        assert_eq!(d[2], 1);
        assert_eq!(d[3], 4);
    }

    #[test]
    fn zero_sized() {
        let mut d = Deque::new();
        d.push_back(());
        d.push_front(());
        d.push_front(());
        d.push_back(());
        assert_eq!(d.len(), 4);
        assert_eq!(d.pop_back(), Some(()));
        assert_eq!(d.pop_back(), Some(()));
        assert_eq!(d.len(), 2);
        assert_eq!((d[0], d[1]), ((), ()));
        assert_eq!(d.front(), Some(&()));
        assert_eq!(d.back(), Some(&()));
        assert_eq!(d.into_iter().collect::<Vec<_>>(), vec![(), ()],);
    }

    #[test]
    fn complex_data() {
        let mut d = Deque::new();
        assert_eq!(d.len(), 0);
        d.push_front(vec![]);
        d.push_back(vec![Box::new(())]);
        d.push_back(vec![Box::new(()), Box::new(())]);
        d.push_front(vec![Box::new(()), Box::new(()), Box::new(())]);
        assert_eq!(d[0].len(), 3);
        assert_eq!(d[1].len(), 0);
        assert_eq!(d[2].len(), 1);
        assert_eq!(d[3].len(), 2);
    }

    #[test]
    fn drop() {
        static mut DROPS: u32 = 0;
        struct S;
        impl Drop for S {
            fn drop(&mut self) {
                unsafe {
                    DROPS += 1;
                }
            }
        }
        let mut d = Deque::new();
        d.push_back(S);
        d.push_back(S);
        d.push_back(S);
        d.push_front(S);
        d.push_front(S);
        core::mem::drop(d);
        unsafe {
            assert_eq!(DROPS, 5);
        }
    }
}
