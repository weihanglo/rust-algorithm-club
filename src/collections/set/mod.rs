use crate::collections::HashMap;
use std::borrow::Borrow;
use std::hash::Hash;
use std::iter::FromIterator;
use std::ops::{BitAnd, BitOr, BitXor, Sub};

/// A hash set implementation based on HashMap
pub struct HashSet<T>
where
    T: Hash + Eq,
{
    hash_map: HashMap<T, ()>,
}

impl<T> HashSet<T>
where
    T: Hash + Eq,
{
    /// Create an empty set.
    pub fn new() -> Self {
        Default::default()
    }

    /// Get the number of non-repetitive items.
    pub fn len(&self) -> usize {
        self.hash_map.len()
    }

    /// Return whether there is no any item in the set.
    pub fn is_empty(&self) -> bool {
        self.hash_map.is_empty()
    }

    /// Try to insert an item. Returns `true` if there were no such item in the set, returns `false`
    /// if an identical item is already in the set.
    pub fn insert(&mut self, value: T) -> bool {
        self.hash_map.insert(value, ()).is_none()
    }

    /// Returns whether an item is present in the set.
    pub fn contains<Q>(&self, value: &Q) -> bool
    where
        T: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.hash_map.get(value).is_some()
    }

    /// Try to remove an item from the set. Returns `true` if such item was present and removed,
    /// returns `false` if no such item was found in the set.
    pub fn remove<Q>(&mut self, value: &Q) -> bool
    where
        T: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.hash_map.remove(value).is_some()
    }

    /// Creates an iterator that yields immutable reference of each item
    /// in arbitrary order.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.hash_map.iter().map(|(k, _)| k)
    }

    /// Returns an iterator visiting every item(s) that exists in `self`, in `other`, or in both
    /// `self` and `other`
    pub fn union<'a>(&'a self, other: &'a HashSet<T>) -> impl Iterator<Item = &T> + 'a {
        self.iter().chain(other.difference(self))
    }

    /// Returns an iterator visiting every item(s) that exists in `self` but not in `other`.
    pub fn difference<'a>(&'a self, other: &'a HashSet<T>) -> impl Iterator<Item = &T> {
        Difference {
            iter: self.iter(),
            other,
        }
    }

    /// Returns an iterator visiting every item(s) that only exists in either `self` or `other`.
    pub fn symmetric_difference<'a>(&'a self, other: &'a HashSet<T>) -> impl Iterator<Item = &T> {
        self.difference(other).chain(other.difference(self))
    }

    /// Returns an iterator visiting every item(s) that exists in both `self` and `other`.
    pub fn intersection<'a>(&'a self, other: &'a HashSet<T>) -> impl Iterator<Item = &T> {
        Intersection {
            iter: self.iter(),
            other,
        }
    }
}

impl<T> Default for HashSet<T>
where
    T: Hash + Eq,
{
    fn default() -> Self {
        Self {
            hash_map: HashMap::new(),
        }
    }
}

// Check equality of sets. Two sets are considered same if they have the same items (regardless of
// stored order)
impl<T> PartialEq for HashSet<T>
where
    T: Hash + Eq,
{
    fn eq(&self, other: &HashSet<T>) -> bool {
        if self.len() != other.len() {
            return false;
        }
        self.iter().all(|item| other.contains(&item))
    }
}

impl<T> FromIterator<T> for HashSet<T>
where
    T: Hash + Eq,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let mut s = Self::new();
        iter.into_iter().for_each(|i| {
            s.insert(i);
        });
        s
    }
}

// An iterable struct that represent the `difference` of two sets.
// It holds an iterator over `self` and a reference of `other`. While iterated, it returns each item(s)
// that exists in `self` but not in `other`.
struct Difference<'a, T, I>
where
    T: Hash + Eq,
    I: Iterator<Item = &'a T>,
{
    iter: I,
    other: &'a HashSet<T>,
}

impl<'a, T, I> Iterator for Difference<'a, T, I>
where
    T: Hash + Eq,
    I: Iterator<Item = &'a T>,
{
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let item = self.iter.next()?;
            if !self.other.contains(item) {
                return Some(item);
            }
        }
    }
}

// An iterable struct that represent the `intersection` of two sets.
// It holds an iterator over `self` and a reference of `other`. While iterated, it returns each
// item(s) that exists in both `self` and `other`
struct Intersection<'a, T, I>
where
    T: Hash + Eq,
    I: Iterator<Item = &'a T>,
{
    iter: I,
    other: &'a HashSet<T>,
}

impl<'a, T, I> Iterator for Intersection<'a, T, I>
where
    T: Hash + Eq,
    I: Iterator<Item = &'a T>,
{
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let item = self.iter.next()?;
            if self.other.contains(item) {
                return Some(item);
            }
        }
    }
}

// The bit_and operator `|`, as an alias of union().
impl<'a, 'b, T> BitOr<&'b HashSet<T>> for &'a HashSet<T>
where
    T: Hash + Eq + Clone,
{
    type Output = HashSet<T>;

    fn bitor(self, rhs: &'b HashSet<T>) -> HashSet<T> {
        self.union(&rhs).cloned().collect()
    }
}

// The sub operator `-`, as an alias of difference().
impl<'a, 'b, T> Sub<&'b HashSet<T>> for &'a HashSet<T>
where
    T: Hash + Eq + Clone,
{
    type Output = HashSet<T>;

    fn sub(self, rhs: &'b HashSet<T>) -> HashSet<T> {
        self.difference(&rhs).cloned().collect()
    }
}

// The bit_xor operator `^`, as an alias of symmetric_difference().
impl<'a, 'b, T> BitXor<&'b HashSet<T>> for &'a HashSet<T>
where
    T: Hash + Eq + Clone,
{
    type Output = HashSet<T>;

    fn bitxor(self, rhs: &'b HashSet<T>) -> HashSet<T> {
        self.symmetric_difference(&rhs).cloned().collect()
    }
}

// The bit_and operator `&`, as an alias of intersection().
impl<'a, 'b, T> BitAnd<&'b HashSet<T>> for &'a HashSet<T>
where
    T: Hash + Eq + Clone,
{
    type Output = HashSet<T>;

    fn bitand(self, rhs: &'b HashSet<T>) -> HashSet<T> {
        self.intersection(&rhs).cloned().collect()
    }
}

#[cfg(test)]
mod hash_set {
    use super::HashSet;
    #[test]
    fn basic() {
        let s: HashSet<String> = HashSet::new();
        assert_eq!(s.len(), 0);
        assert!(s.is_empty());
    }

    #[test]
    fn insert() {
        let mut s = HashSet::new();
        let ok = s.insert("cat");
        assert!(ok);
        assert_eq!(s.len(), 1);

        let ok = s.insert("dog");
        assert!(ok);
        assert_eq!(s.len(), 2);

        // dog already exist!
        let ok = s.insert("dog");
        assert_eq!(
            ok, false,
            "Attempting to insert present value returns false"
        );
        assert_eq!(s.len(), 2, "Certain value can only be inserted to set once");
    }

    #[test]
    fn contains() {
        let mut s1: HashSet<&str> = HashSet::new();
        s1.insert("cat");
        assert_eq!(
            s1.contains("cat"),
            true,
            "contains() returns true for present value"
        );
        assert_eq!(
            s1.contains("dog"),
            false,
            "contains() returns false for absent value"
        );

        let mut s2: HashSet<String> = HashSet::new();
        s2.insert("cat".to_string());
        assert_eq!(
            s2.contains(&"cat".to_string()),
            true,
            "Can query with String"
        );
        assert_eq!(s2.contains("cat"), true, "Can query with &str");
    }

    #[test]
    fn remove() {
        let mut s1: HashSet<&str> = HashSet::new();
        s1.insert("cat");
        assert!(s1.contains("cat"), "'cat' exists before remove()");
        let ok = s1.remove("cat");
        assert_eq!(ok, true, "Successful removal returns true");
        assert!(!s1.contains("cat"), "'cat' is gone after remove()");

        let ok = s1.remove("elephant");
        assert_eq!(
            ok, false,
            "Trying to remove non-existing value returns false"
        );

        let mut s2: HashSet<String> = HashSet::new();
        s2.insert("cat".to_string());
        s2.insert("dog".to_string());
        assert!(s2.remove(&"cat".to_string()), "Can remove with String");
        assert!(
            !s2.contains("cat"),
            "Successfully removed value with String"
        );
        assert!(s2.remove("dog"), "Can remove with &str");
        assert!(!s2.contains("dog"), "Successfully removed value with &str");
    }

    #[test]
    fn from_iter() {
        let s1: HashSet<_> = ["cat", "dog", "rat"].iter().cloned().collect();
        assert!(s1.contains("cat"));
        assert!(s1.contains("dog"));
        assert!(s1.contains("rat"));
        assert_eq!(s1.len(), 3);
    }

    #[test]
    fn eq() {
        let set: HashSet<_> = ["cat", "dog", "rat"].iter().cloned().collect();

        let identical: HashSet<_> = ["cat", "dog", "rat"].iter().cloned().collect();
        assert!(set == identical, "sets of identical elements are equal");

        let reordered: HashSet<_> = ["rat", "cat", "dog"].iter().cloned().collect();
        assert!(set == reordered, "order of elements doesn't matter");

        let different: HashSet<_> = ["cat", "dog", "elephant"].iter().cloned().collect();
        assert!(set != different);

        let superset: HashSet<_> = ["cat", "dog", "rat", "elephant"].iter().cloned().collect();
        assert!(set != superset);

        let subset: HashSet<_> = ["cat"].iter().cloned().collect();
        assert!(set != subset);
    }

    #[test]
    fn union() {
        let s1: HashSet<_> = ["cat", "dog"].iter().cloned().collect();
        let s2: HashSet<_> = ["cat", "rat"].iter().cloned().collect();
        let union: HashSet<_> = s1.union(&s2).cloned().collect();

        let expect: HashSet<&str> = ["cat", "dog", "rat"].iter().cloned().collect();

        assert!(union == expect);
    }

    #[test]
    fn bitor() {
        let s1: HashSet<_> = ["cat", "dog"].iter().cloned().collect();
        let s2: HashSet<_> = ["cat", "rat"].iter().cloned().collect();
        let union = &s1 | &s2;

        let expect: HashSet<&str> = ["cat", "dog", "rat"].iter().cloned().collect();

        assert!(union == expect);

        assert_eq!(s1.len(), 2, "s1 is still available");
        assert_eq!(s2.len(), 2, "s2 is still available");
    }

    #[test]
    fn difference() {
        let s1: HashSet<_> = ["cat", "dog"].iter().cloned().collect();
        let s2: HashSet<_> = ["cat", "rat"].iter().cloned().collect();
        let difference: HashSet<_> = s1.difference(&s2).cloned().collect();

        let expect: HashSet<&str> = ["dog"].iter().cloned().collect();

        assert!(difference == expect);
    }

    #[test]
    fn sub() {
        let s1: HashSet<_> = ["cat", "dog"].iter().cloned().collect();
        let s2: HashSet<_> = ["cat", "rat"].iter().cloned().collect();
        let difference = &s1 - &s2;

        let expect: HashSet<&str> = ["dog"].iter().cloned().collect();

        assert!(difference == expect);
    }

    #[test]
    fn symmetric_difference() {
        let s1: HashSet<_> = ["cat", "dog"].iter().cloned().collect();
        let s2: HashSet<_> = ["cat", "rat"].iter().cloned().collect();
        let symmetric_difference: HashSet<_> = s1.symmetric_difference(&s2).cloned().collect();

        let expect: HashSet<&str> = ["dog", "rat"].iter().cloned().collect();

        assert!(symmetric_difference == expect);
    }

    #[test]
    fn bitxor() {
        let s1: HashSet<_> = ["cat", "dog"].iter().cloned().collect();
        let s2: HashSet<_> = ["cat", "rat"].iter().cloned().collect();
        let symmetric_difference: HashSet<_> = &s1 ^ &s2;

        let expect: HashSet<&str> = ["dog", "rat"].iter().cloned().collect();

        assert!(symmetric_difference == expect);
    }

    #[test]
    fn intersection() {
        let s1: HashSet<_> = ["cat", "dog"].iter().cloned().collect();
        let s2: HashSet<_> = ["cat", "rat"].iter().cloned().collect();
        let intersection: HashSet<_> = s1.intersection(&s2).cloned().collect();

        let expect: HashSet<&str> = ["cat"].iter().cloned().collect();

        assert!(intersection == expect);
    }

    #[test]
    fn bitand() {
        let s1: HashSet<_> = ["cat", "dog"].iter().cloned().collect();
        let s2: HashSet<_> = ["cat", "rat"].iter().cloned().collect();
        let intersection: HashSet<_> = &s1 & &s2;

        let expect: HashSet<&str> = ["cat"].iter().cloned().collect();

        assert!(intersection == expect);
    }
}
