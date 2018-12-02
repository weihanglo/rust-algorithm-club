use super::hash_map::HashMap;
use std::borrow::Borrow;
use std::hash::Hash;
use std::iter::FromIterator;
use std::ops::BitAnd;
use std::ops::BitXor;
use std::ops::Sub;

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
    ///
    pub fn new() -> Self {
        Default::default()
    }

    ///
    pub fn len(&self) -> usize {
        self.hash_map.len()
    }

    ///
    pub fn is_empty(&self) -> bool {
        self.hash_map.is_empty()
    }

    ///
    pub fn insert(&mut self, value: T) -> bool {
        match self.hash_map.insert(value, ()) {
            None => true,
            Some(_) => false,
        }
    }

    ///
    pub fn contains<Q>(&self, value: &Q) -> bool
    where
        T: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        match self.hash_map.get(value) {
            Some(_) => true,
            None => false,
        }
    }

    ///
    pub fn remove<Q>(&mut self, value: &Q) -> bool
    where
        T: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        match self.hash_map.remove(value) {
            Some(_) => true,
            None => false,
        }
    }
    /// Creates an iterator that yields immutable reference of each element
    /// in arbitrary order.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.hash_map.iter().map(|(k, _)| k)
    }

    /// Returns an iterator visiting all items present in `self` and `other`
    ///
    /// The union of set `self` and set `other` is composed by chaining `self.iter()` with items
    /// that are only present in `other` (i.e. `other.difference(self)`)
    pub fn union<'a>(&'a self, other: &'a HashSet<T>) -> impl Iterator<Item = &T> + 'a {
        self.iter().chain(other.difference(self))
    }

    /// Returns an iterator visiting items present in `self` but not in `other`
    ///
    ///
    pub fn difference<'a>(&'a self, other: &'a HashSet<T>) -> impl Iterator<Item = &T> {
        Difference {
            iter: self.iter(),
            other,
        }
    }

    /// Returns an iterator visiting items which only preset in either self or other
    ///
    ///
    pub fn symmetric_difference<'a>(&'a self, other: &'a HashSet<T>) -> impl Iterator<Item = &T> {
        self.difference(other).chain(other.difference(self))
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

impl<'a, 'b, T> BitAnd<&'b HashSet<T>> for &'a HashSet<T>
where
    T: Hash + Eq + Clone,
{
    type Output = HashSet<T>;

    fn bitand(self, rhs: &'b HashSet<T>) -> HashSet<T> {
        self.union(&rhs).cloned().collect()
    }
}

impl<'a, 'b, T> Sub<&'b HashSet<T>> for &'a HashSet<T>
where
    T: Hash + Eq + Clone,
{
    type Output = HashSet<T>;

    fn sub(self, rhs: &'b HashSet<T>) -> HashSet<T> {
        self.difference(&rhs).cloned().collect()
    }
}

impl<'a, 'b, T> BitXor<&'b HashSet<T>> for &'a HashSet<T>
where
    T: Hash + Eq + Clone,
{
    type Output = HashSet<T>;

    fn bitxor(self, rhs: &'b HashSet<T>) -> HashSet<T> {
        self.symmetric_difference(&rhs).cloned().collect()
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
    fn union() {
        let mut s1: HashSet<&str> = HashSet::new();
        s1.insert("cat");
        s1.insert("dog");

        let mut s2: HashSet<&str> = HashSet::new();
        s2.insert("rat");

        let union: HashSet<_> = s1.union(&s2).collect();
        assert_eq!(
            union.contains(&"cat"),
            true,
            "union of s1 and s2 contains cat (from s1)"
        );
        assert_eq!(
            union.contains(&"dog"),
            true,
            "union of s1 and s2 contains dog (from s1)"
        );
        assert_eq!(
            union.contains(&"rat"),
            true,
            "union of s1 and s2 contains rat (from s2)"
        );
        assert_eq!(union.len(), 3, "length of union is 3");
    }

    #[test]
    fn bitand() {
        let mut s1: HashSet<&str> = HashSet::new();
        s1.insert("cat");
        s1.insert("dog");

        let mut s2: HashSet<&str> = HashSet::new();
        s2.insert("rat");

        let union = &s1 & &s2;
        assert!(
            union.contains("cat"),
            "union of s1 and s2 contains cat (from s1)"
        );
        assert!(
            union.contains("dog"),
            "union of s1 and s2 contains dog (from s1)"
        );
        assert!(
            union.contains("rat"),
            "union of s1 and s2 contains rat (from s2)"
        );
        assert_eq!(union.len(), 3, "length of union is 3");

        assert_eq!(s1.len(), 2, "s1 is still available");
        assert_eq!(s2.len(), 1, "s2 is still available");
    }

    #[test]
    fn difference() {
        let mut s1 = HashSet::new();
        s1.insert("cat");
        s1.insert("dog");

        let mut s2 = HashSet::new();
        s2.insert("cat");
        s2.insert("rat");

        let difference: HashSet<_> = s1.difference(&s2).collect();
        assert_eq!(
            difference.contains(&"dog"),
            true,
            "dog is in s1 but not in s2, therefore included in difference"
        );
        assert_eq!(
            difference.contains(&"cat"),
            false,
            "cat is in both s1 and s2, therefore not included in difference"
        );
        assert_eq!(
            difference.contains(&"rat"),
            false,
            "rat is from s2, therefore not included in difference"
        );
        assert_eq!(difference.len(), 1, "length of difference is 1");
    }

    #[test]
    fn sub() {
        let mut s1 = HashSet::new();
        s1.insert("cat");
        s1.insert("dog");

        let mut s2 = HashSet::new();
        s2.insert("cat");
        s2.insert("rat");

        let difference = &s1 - &s2;
        assert_eq!(
            difference.contains(&"dog"),
            true,
            "dog is in s1 but not in s2, therefore included in difference"
        );
        assert_eq!(
            difference.contains(&"cat"),
            false,
            "cat is in both s1 and s2, therefore not included in difference"
        );
        assert_eq!(
            difference.contains(&"rat"),
            false,
            "rat is from s2, therefore not included in difference"
        );
        assert_eq!(difference.len(), 1, "length of difference is 1");
    }

    #[test]
    fn symmetric_difference() {
        let mut s1 = HashSet::new();
        s1.insert("cat");
        s1.insert("dog");

        let mut s2 = HashSet::new();
        s2.insert("cat");
        s2.insert("rat");

        let symmetric_difference: HashSet<_> = s1.symmetric_difference(&s2).collect();
        assert_eq!(
            symmetric_difference.contains(&"cat"),
            false,
            "cat is in both s1 and s2, therefore not included in symmetric_difference"
        );
        assert_eq!(
            symmetric_difference.contains(&"dog"),
            true,
            "dog is in s1 but not in s2, therefore included in symmetric_difference"
        );
        assert_eq!(
            symmetric_difference.contains(&"rat"),
            true,
            "rat is s2 but not in s1, therefore included in symmetric_difference"
        );
        assert_eq!(
            symmetric_difference.len(),
            2,
            "length of symmetric_difference is 2"
        );
    }

    #[test]
    fn bitxor() {
        let mut s1 = HashSet::new();
        s1.insert("cat");
        s1.insert("dog");

        let mut s2 = HashSet::new();
        s2.insert("cat");
        s2.insert("rat");

        let symmetric_difference: HashSet<_> = &s1 ^ &s2;
        assert_eq!(
            symmetric_difference.contains(&"cat"),
            false,
            "cat is in both s1 and s2, therefore not included in symmetric_difference"
        );
        assert_eq!(
            symmetric_difference.contains(&"dog"),
            true,
            "dog is in s1 but not in s2, therefore included in symmetric_difference"
        );
        assert_eq!(
            symmetric_difference.contains(&"rat"),
            true,
            "rat is s2 but not in s1, therefore included in symmetric_difference"
        );
        assert_eq!(
            symmetric_difference.len(),
            2,
            "length of symmetric_difference is 2"
        );
    }
}
