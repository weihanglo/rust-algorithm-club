use crate::collections::HashMap;
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::hash::Hash;
use std::iter::FromIterator;
use std::ops::{BitAnd, BitOr, BitXor, Sub};

/// A hash set implementation based on `HashMap`.
///
/// References:
///
/// - [Rust Standard Library: std::collections::HashSet][1]
///
/// [1]: https://doc.rust-lang.org/stable/std/collections/struct.HashMap.html
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
    /// Creates an empty set.
    pub fn new() -> Self {
        Default::default()
    }

    /// Gets the number of non-repetitive elements, equivalently to the cardinality of a set.
    ///
    /// # Complexity
    ///
    /// Constant.
    pub fn len(&self) -> usize {
        self.hash_map.len()
    }

    /// Returns whether there is no any element in the set.
    ///
    /// # Complexity
    ///
    /// Constant.
    pub fn is_empty(&self) -> bool {
        self.hash_map.is_empty()
    }

    /// Inserts an element into the set.
    ///
    /// Returns `true` if there were no such element in the set; returns `false`
    /// if an identical element is already in the set.
    ///
    /// # Parameters
    ///
    /// * `value` - Element to be inserted.
    ///
    /// # Complexity
    ///
    /// Constant.
    pub fn insert(&mut self, value: T) -> bool {
        self.hash_map.insert(value, ()).is_none()
    }

    /// Returns whether an element is present in the set.
    ///
    /// This is equivalent to "belongs to ∈" relation in mathematics.
    ///
    /// # Parameters
    ///
    /// * `value` - Element to be checked whether is in the set.
    ///
    /// # Complexity
    ///
    /// Constant.
    pub fn contains<Q>(&self, value: &Q) -> bool
    where
        T: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.hash_map.get(value).is_some()
    }

    /// Removes an element from the set.
    ///
    /// Returns `true` if such item was present and removed; returns `false`
    /// if no such item was found in the set.
    ///
    /// # Parameters
    ///
    /// * `value` - Element to be removed.
    ///
    /// # Complexity
    ///
    /// Constant.
    pub fn remove<Q>(&mut self, value: &Q) -> bool
    where
        T: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.hash_map.remove(value).is_some()
    }

    /// Creates an iterator yielding immutable reference of each item in arbitrary order.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.hash_map.iter().map(|(k, _)| k)
    }

    /// Returns an iterator visiting items that exists in `self`, in `other`,
    /// or in both `self` and `other`
    ///
    /// This is equivalent to `self ∪ other` in mathematics.
    ///
    /// # Parameters
    ///
    /// * `other` - The other set.
    pub fn union<'a>(&'a self, other: &'a HashSet<T>) -> impl Iterator<Item = &T> {
        // self ∪ (other \ self)
        self.iter().chain(other.difference(self))
    }

    /// Returns an iterator visiting items that exists in `self` but not in `other`.
    ///
    /// This is equivalent to `self \ other` in mathematics.
    ///
    /// # Parameters
    ///
    /// * `other` - The other set.
    pub fn difference<'a>(&'a self, other: &'a HashSet<T>) -> impl Iterator<Item = &T> {
        self.iter().filter(move |item| !other.contains(item))
    }

    /// Returns an iterator visiting items that only exists in either `self` or
    /// `other` but not in their intersection.
    ///
    /// This is equivalent to `self △ other` in mathematics.
    ///
    /// # Parameters
    ///
    /// * `other` - The other set.
    pub fn symmetric_difference<'a>(&'a self, other: &'a HashSet<T>) -> impl Iterator<Item = &T> {
        // (self \ other) ∪ (other \ self)
        self.difference(other).chain(other.difference(self))
    }

    /// Returns an iterator visiting items that exists in both `self` and `other`.
    ///
    /// This is equivalent to `self ∩ other` in mathematics.
    ///
    /// # Parameters
    ///
    /// * `other` - The other set.
    pub fn intersection<'a>(&'a self, other: &'a HashSet<T>) -> impl Iterator<Item = &T> {
        self.iter().filter(move |item| other.contains(item))
    }

    /// Returns true if `self` has no elements in common with `other`.
    ///
    /// This is equivalent to checking for an empty intersection, which means
    /// their intersection is the empty set ∅.
    ///
    /// # Parameters
    ///
    /// * `other` - The other set.
    ///
    /// # Complexity
    ///
    /// Linear in the size of `self`.
    pub fn is_disjoint(&self, other: &HashSet<T>) -> bool {
        self.intersection(other).count() == 0
    }

    /// Returns true if `other` contains at least all elements in `self`.
    ///
    /// This is equivalent to `self ⊆ other` in mathematics.
    ///
    /// # Parameters
    ///
    /// * `other` - The other set.
    ///
    /// # Complexity
    ///
    /// Linear in the size of `self`.
    pub fn is_subset(&self, other: &HashSet<T>) -> bool {
        if self.len() > other.len() {
            return false;
        }
        self.iter().all(|item| other.contains(&item))
    }

    /// Returns true if `self` contains at least all elements in `other`.
    ///
    /// This is equivalent to `self ⊇ other` in mathematics.
    ///
    /// # Parameters
    ///
    /// * `other` - The other set.
    ///
    /// # Complexity
    ///
    /// Linear in the size of `other`.
    pub fn is_superset(&self, other: &HashSet<T>) -> bool {
        other.is_subset(self)
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

impl<T> PartialEq for HashSet<T>
where
    T: Hash + Eq,
{
    /// Checks the equality of sets.
    ///
    /// Two sets are defined to be equal if they contain the same elements and
    /// their cardinality are equal.
    ///
    /// Set theory definition: x = y ⇒ ∀z, (z ∈ x ⇔ z ∈ y)
    ///
    /// # Parameters
    ///
    /// * `other` - The other set.
    ///
    /// # Complexity
    ///
    /// Linear in the size of `self`.
    fn eq(&self, other: &HashSet<T>) -> bool {
        if self.len() != other.len() {
            return false;
        }
        self.iter().all(|item| other.contains(&item))
    }
}

/// A set is reflecxively equal to itself.
impl<T> Eq for HashSet<T> where T: Hash + Eq {}

impl<T> PartialOrd for HashSet<T>
where
    T: Hash + Eq,
{
    /// Compares sets to determine whether one is a subset of the other or not.
    ///
    /// # Parameters
    ///
    /// * `other` - The other set.
    ///
    /// # Complexity
    ///
    /// Linear in the size of `max(self, other)`.
    fn partial_cmp(&self, other: &HashSet<T>) -> Option<Ordering> {
        let is_subset = self.is_subset(other);
        let same_size = self.len() == other.len();
        match (is_subset, same_size) {
            (true, true) => Some(Ordering::Equal),
            (true, false) => Some(Ordering::Less),
            (false, true) => None,
            _ => Some(Ordering::Greater).filter(|_| self.is_superset(other)),
        }
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

/// The bitor operator `|`, as an alias of `union()`.
impl<'a, 'b, T> BitOr<&'b HashSet<T>> for &'a HashSet<T>
where
    T: Hash + Eq + Clone,
{
    type Output = HashSet<T>;

    fn bitor(self, rhs: &'b HashSet<T>) -> Self::Output {
        self.union(&rhs).cloned().collect()
    }
}

/// The sub operator `-`, as an alias of `difference()`.
impl<'a, 'b, T> Sub<&'b HashSet<T>> for &'a HashSet<T>
where
    T: Hash + Eq + Clone,
{
    type Output = HashSet<T>;

    fn sub(self, rhs: &'b HashSet<T>) -> Self::Output {
        self.difference(&rhs).cloned().collect()
    }
}

/// The bitxor operator `^`, as an alias of `symmetric_difference()`.
impl<'a, 'b, T> BitXor<&'b HashSet<T>> for &'a HashSet<T>
where
    T: Hash + Eq + Clone,
{
    type Output = HashSet<T>;

    fn bitxor(self, rhs: &'b HashSet<T>) -> Self::Output {
        self.symmetric_difference(&rhs).cloned().collect()
    }
}

/// The bit_and operator `&`, as an alias of intersection().
impl<'a, 'b, T> BitAnd<&'b HashSet<T>> for &'a HashSet<T>
where
    T: Hash + Eq + Clone,
{
    type Output = HashSet<T>;

    fn bitand(self, rhs: &'b HashSet<T>) -> Self::Output {
        self.intersection(&rhs).cloned().collect()
    }
}

#[cfg(test)]
mod basics {
    use super::*;

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
}

#[cfg(test)]
mod set_relations {
    use super::*;

    #[test]
    fn union() {
        // ∅ ∪ ∅ = ∅
        let s1: HashSet<&str> = [].iter().cloned().collect();
        let s2: HashSet<&str> = [].iter().cloned().collect();
        let union = s1.union(&s2);
        assert_eq!(union.count(), 0, "∅ ∪ ∅ = ∅");

        // ∅ ∪ {cat} = {cat}
        let s1: HashSet<&str> = [].iter().cloned().collect();
        let s2: HashSet<&str> = ["cat"].iter().cloned().collect();
        let union: HashSet<_> = s1.union(&s2).cloned().collect();
        let expect: HashSet<&str> = ["cat"].iter().cloned().collect();
        assert!(union == expect);

        // {cat} ∪ ∅ = {cat}
        let s1: HashSet<&str> = ["cat"].iter().cloned().collect();
        let s2: HashSet<&str> = [].iter().cloned().collect();
        let union: HashSet<_> = s1.union(&s2).cloned().collect();
        let expect: HashSet<&str> = ["cat"].iter().cloned().collect();
        assert!(union == expect);

        // {cat,dog} ∪ {cat,rat} = {cat,dot,rat}
        let s1: HashSet<_> = ["cat", "dog"].iter().cloned().collect();
        let s2: HashSet<_> = ["cat", "rat"].iter().cloned().collect();
        let union: HashSet<_> = s1.union(&s2).cloned().collect();
        let expect: HashSet<&str> = ["cat", "dog", "rat"].iter().cloned().collect();
        assert!(union == expect);
    }

    #[test]
    fn intersection() {
        // ∅ ∩ ∅ = ∅
        let s1: HashSet<&str> = [].iter().cloned().collect();
        let s2: HashSet<&str> = [].iter().cloned().collect();
        let intersection = s1.intersection(&s2);
        assert_eq!(intersection.count(), 0, "∅ ∩ ∅ = ∅");

        // ∅ ∩ {cat} = ∅
        let s1: HashSet<&str> = [].iter().cloned().collect();
        let s2: HashSet<&str> = ["cat"].iter().cloned().collect();
        let intersection = s1.intersection(&s2);
        assert_eq!(intersection.count(), 0);

        // {cat} ∩ ∅ = ∅
        let s1: HashSet<&str> = ["cat"].iter().cloned().collect();
        let s2: HashSet<&str> = [].iter().cloned().collect();
        let intersection = s1.intersection(&s2);
        assert_eq!(intersection.count(), 0);

        // {cat,dog} ∩ {cat,rat} = {cat}
        let s1: HashSet<_> = ["cat", "dog"].iter().cloned().collect();
        let s2: HashSet<_> = ["cat", "rat"].iter().cloned().collect();
        let intersection: HashSet<_> = s1.intersection(&s2).cloned().collect();
        let expect: HashSet<&str> = ["cat"].iter().cloned().collect();
        assert!(intersection == expect);
    }

    #[test]
    fn difference() {
        // ∅ \ ∅ = ∅
        let s1: HashSet<&str> = [].iter().cloned().collect();
        let s2: HashSet<&str> = [].iter().cloned().collect();
        let difference = s1.difference(&s2);
        assert_eq!(difference.count(), 0, r"∅ \ ∅ = ∅");

        // ∅ \ {cat} = ∅
        let s1: HashSet<&str> = [].iter().cloned().collect();
        let s2: HashSet<&str> = ["cat"].iter().cloned().collect();
        let difference = s1.difference(&s2);
        assert_eq!(difference.count(), 0);

        // {cat} \ ∅  = {cat}
        let s1: HashSet<&str> = ["cat"].iter().cloned().collect();
        let s2: HashSet<&str> = [].iter().cloned().collect();
        let difference: HashSet<_> = s1.difference(&s2).cloned().collect();
        let expect: HashSet<&str> = ["cat"].iter().cloned().collect();
        assert!(difference == expect);

        // {cat,dog} \ {cat,rat} = {dog}
        let s1: HashSet<_> = ["cat", "dog"].iter().cloned().collect();
        let s2: HashSet<_> = ["cat", "rat"].iter().cloned().collect();
        let difference: HashSet<_> = s1.difference(&s2).cloned().collect();
        let expect: HashSet<&str> = ["dog"].iter().cloned().collect();
        assert!(difference == expect);
    }

    #[test]
    fn symmetric_difference() {
        // ∅ △ ∅ = ∅
        let s1: HashSet<&str> = [].iter().cloned().collect();
        let s2: HashSet<&str> = [].iter().cloned().collect();
        let symmetric_difference = s1.symmetric_difference(&s2);
        assert_eq!(symmetric_difference.count(), 0, "∅ △ ∅ = ∅");

        // ∅ △ {cat} = {cat}
        let s1: HashSet<&str> = [].iter().cloned().collect();
        let s2: HashSet<&str> = ["cat"].iter().cloned().collect();
        let symmetric_difference: HashSet<_> = s1.symmetric_difference(&s2).cloned().collect();
        let expect: HashSet<&str> = ["cat"].iter().cloned().collect();
        assert!(symmetric_difference == expect);

        // {cat} △ ∅ = {cat}
        let s1: HashSet<&str> = ["cat"].iter().cloned().collect();
        let s2: HashSet<&str> = [].iter().cloned().collect();
        let symmetric_difference: HashSet<_> = s1.symmetric_difference(&s2).cloned().collect();
        let expect: HashSet<&str> = ["cat"].iter().cloned().collect();
        assert!(symmetric_difference == expect);

        // {cat,dog} △ {cat,rat} = {dog, rat}
        let s1: HashSet<_> = ["cat", "dog"].iter().cloned().collect();
        let s2: HashSet<_> = ["cat", "rat"].iter().cloned().collect();
        let symmetric_difference: HashSet<_> = s1.symmetric_difference(&s2).cloned().collect();
        let expect: HashSet<&str> = ["dog", "rat"].iter().cloned().collect();
        assert!(symmetric_difference == expect);
    }

    #[test]
    fn is_disjoint() {
        // ∅, ∅ are disjoint.
        let s1: HashSet<&str> = [].iter().cloned().collect();
        let s2: HashSet<&str> = [].iter().cloned().collect();
        assert!(s1.is_disjoint(&s2), "∅, ∅ are disjoint");

        // ∅, {cat} are disjoint.
        let s1: HashSet<&str> = [].iter().cloned().collect();
        let s2: HashSet<&str> = ["cat"].iter().cloned().collect();
        assert!(s1.is_disjoint(&s2), "∅, {cat} are disjoint");
        assert!(s2.is_disjoint(&s1), "∅, {cat} are disjoint");

        // {rat}, {cat} are disjoint.
        let s1: HashSet<&str> = ["rat"].iter().cloned().collect();
        let s2: HashSet<&str> = ["cat"].iter().cloned().collect();
        assert!(s1.is_disjoint(&s2));

        // {cat}, {cat} are not disjoint.
        let s1: HashSet<&str> = ["cat"].iter().cloned().collect();
        let s2: HashSet<&str> = ["cat"].iter().cloned().collect();
        assert_eq!(s1.is_disjoint(&s2), false);
        assert_eq!(s2.is_disjoint(&s1), false);
    }

    #[test]
    fn is_subset() {
        // ∅ ⊆ ∅
        let s1: HashSet<&str> = [].iter().cloned().collect();
        let s2: HashSet<&str> = [].iter().cloned().collect();
        assert!(s1.is_subset(&s2), "∅ ⊆ ∅");
        assert!(s2.is_subset(&s1), "∅ ⊆ ∅");

        // ∀𝑨: ∅ ⊆ 𝑨
        let s1: HashSet<&str> = [].iter().cloned().collect();
        let s2: HashSet<&str> = ["cat"].iter().cloned().collect();
        assert!(s1.is_subset(&s2), "∀𝑨: ∅ ⊆ 𝑨");

        // ∀𝑨, 𝑨 ≠ ∅: 𝑨 ⊈ ∅
        let s1: HashSet<&str> = ["cat"].iter().cloned().collect();
        let s2: HashSet<&str> = [].iter().cloned().collect();
        assert_eq!(s1.is_subset(&s2), false, "∀𝑨, 𝑨 ≠ ∅: 𝑨 ⊈ ∅");

        // {cat} ⊆ {cat}
        let s1: HashSet<&str> = ["cat"].iter().cloned().collect();
        let s2: HashSet<&str> = ["cat"].iter().cloned().collect();
        assert!(s1.is_subset(&s2));

        // {cat} ⊆ {cat,rat}
        let s1: HashSet<&str> = ["cat"].iter().cloned().collect();
        let s2: HashSet<&str> = ["cat", "rat"].iter().cloned().collect();
        assert!(s1.is_subset(&s2));

        // {cat,rat} ⊈ {cat}
        let s1: HashSet<&str> = ["cat", "rat"].iter().cloned().collect();
        let s2: HashSet<&str> = ["cat"].iter().cloned().collect();
        assert_eq!(s1.is_subset(&s2), false);
    }

    #[test]
    fn is_superset() {
        // ∅ ⊇ ∅
        let s1: HashSet<&str> = [].iter().cloned().collect();
        let s2: HashSet<&str> = [].iter().cloned().collect();
        assert!(s1.is_superset(&s2), "∅ ⊇ ∅");
        assert!(s2.is_superset(&s1), "∅ ⊇ ∅");

        // ∀𝑨, 𝑨 ≠ ∅: ∅ ⊉ 𝑨
        let s1: HashSet<&str> = [].iter().cloned().collect();
        let s2: HashSet<&str> = ["cat"].iter().cloned().collect();
        assert_eq!(s1.is_superset(&s2), false, "∀𝑨, 𝑨 ≠ ∅: ∅ ⊉ 𝑨");

        // ∀𝑨: 𝑨 ⊇ ∅
        let s1: HashSet<&str> = ["cat"].iter().cloned().collect();
        let s2: HashSet<&str> = [].iter().cloned().collect();
        assert_eq!(s1.is_superset(&s2), true, "∀𝑨: 𝑨 ⊇ ∅");

        // {cat} ⊇ {cat}
        let s1: HashSet<&str> = ["cat"].iter().cloned().collect();
        let s2: HashSet<&str> = ["cat"].iter().cloned().collect();
        assert!(s1.is_superset(&s2));

        // {cat} ⊉ {cat,rat}
        let s1: HashSet<&str> = ["cat"].iter().cloned().collect();
        let s2: HashSet<&str> = ["cat", "rat"].iter().cloned().collect();
        assert_eq!(s1.is_superset(&s2), false);

        // {cat,rat} ⊇ {cat}
        let s1: HashSet<&str> = ["cat", "rat"].iter().cloned().collect();
        let s2: HashSet<&str> = ["cat"].iter().cloned().collect();
        assert!(s1.is_superset(&s2));
    }
}

#[cfg(test)]
mod logical_ops {
    use super::*;

    #[test]
    fn bitor() {
        // Same as union
        let s1: HashSet<_> = ["cat", "dog"].iter().cloned().collect();
        let s2: HashSet<_> = ["cat", "rat"].iter().cloned().collect();
        let union = &s1 | &s2;
        let expect: HashSet<&str> = ["cat", "dog", "rat"].iter().cloned().collect();
        assert!(union == expect);
        assert_eq!(s1.len(), 2, "s1 is still available");
        assert_eq!(s2.len(), 2, "s2 is still available");
    }

    #[test]
    fn bitand() {
        // Same as intersection
        let s1: HashSet<_> = ["cat", "dog"].iter().cloned().collect();
        let s2: HashSet<_> = ["cat", "rat"].iter().cloned().collect();
        let intersection: HashSet<_> = &s1 & &s2;
        let expect: HashSet<&str> = ["cat"].iter().cloned().collect();
        assert!(intersection == expect);
    }

    #[test]
    fn sub() {
        // Same as difference
        let s1: HashSet<_> = ["cat", "dog"].iter().cloned().collect();
        let s2: HashSet<_> = ["cat", "rat"].iter().cloned().collect();
        let difference = &s1 - &s2;
        let expect: HashSet<&str> = ["dog"].iter().cloned().collect();
        assert!(difference == expect);
    }

    #[test]
    fn bitxor() {
        // Same as difference
        let s1: HashSet<_> = ["cat", "dog"].iter().cloned().collect();
        let s2: HashSet<_> = ["cat", "rat"].iter().cloned().collect();
        let symmetric_difference: HashSet<_> = &s1 ^ &s2;
        let expect: HashSet<&str> = ["dog", "rat"].iter().cloned().collect();
        assert!(symmetric_difference == expect);
    }
}

#[cfg(test)]
mod cmp_ops {
    use super::*;

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

        // ∅ = ∅
        let s1: HashSet<&str> = [].iter().cloned().collect();
        let s2: HashSet<&str> = [].iter().cloned().collect();
        assert!(s1 == s2, "∅ = ∅");

        // ∅ ≠ {cat}
        let s1: HashSet<&str> = [].iter().cloned().collect();
        let s2: HashSet<&str> = ["cat"].iter().cloned().collect();
        assert_eq!(s1 != s2, true);

        // {cat} ≠ ∅
        let s1: HashSet<&str> = ["cat"].iter().cloned().collect();
        let s2: HashSet<&str> = [].iter().cloned().collect();
        assert_eq!(s1 != s2, true)
    }

    #[test]
    fn partial_cmp() {
        let set: HashSet<_> = ["cat", "dog", "rat"].iter().cloned().collect();

        let identical: HashSet<_> = ["cat", "dog", "rat"].iter().cloned().collect();
        assert_eq!(set.partial_cmp(&identical), Some(Ordering::Equal));
        assert_eq!(&set > &identical, false);
        assert_eq!(&set >= &identical, true);
        assert_eq!(&set < &identical, false);
        assert_eq!(&set <= &identical, true);
        assert_eq!(&set == &identical, true);

        let different: HashSet<_> = ["cat", "dog", "elephant"].iter().cloned().collect();
        assert_eq!(set.partial_cmp(&different), None);
        assert_eq!(&set > &different, false);
        assert_eq!(&set >= &different, false);
        assert_eq!(&set < &different, false);
        assert_eq!(&set <= &different, false);
        assert_eq!(&set == &different, false);

        let superset: HashSet<_> = ["cat", "dog", "rat", "elephant"].iter().cloned().collect();
        assert_eq!(set.partial_cmp(&superset), Some(Ordering::Less));
        assert_eq!(&set > &superset, false);
        assert_eq!(&set >= &superset, false);
        assert_eq!(&set < &superset, true);
        assert_eq!(&set <= &superset, true);
        assert_eq!(&set == &superset, false);

        let subset: HashSet<_> = ["cat"].iter().cloned().collect();
        assert_eq!(set.partial_cmp(&subset), Some(Ordering::Greater));
        assert_eq!(&set > &subset, true);
        assert_eq!(&set < &subset, false);
        assert_eq!(&set == &subset, false);

        // ∅ = ∅
        let s1: HashSet<&str> = [].iter().cloned().collect();
        let s2: HashSet<&str> = [].iter().cloned().collect();
        assert_eq!(s1.partial_cmp(&s2), Some(Ordering::Equal));
        assert_eq!(&s1 > &s2, false);
        assert_eq!(&s1 >= &s2, true);
        assert_eq!(&s1 < &s2, false);
        assert_eq!(&s1 <= &s2, true);
        assert_eq!(&s1 == &s2, true);

        // ∅ ≠ {cat}
        let s1: HashSet<&str> = [].iter().cloned().collect();
        let s2: HashSet<&str> = ["cat"].iter().cloned().collect();
        assert_eq!(s1.partial_cmp(&s2), Some(Ordering::Less));
        assert_eq!(&s1 > &s2, false);
        assert_eq!(&s1 >= &s2, false);
        assert_eq!(&s1 < &s2, true);
        assert_eq!(&s1 <= &s2, true);
        assert_eq!(&s1 == &s2, false);
    }
}
