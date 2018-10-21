use super::hash_map::HashMap;
use std::hash::Hash;
use std::borrow::Borrow;
use std::iter::FromIterator;

/// A hash set implementation with HashSet
pub struct HashSet<T> where T: Hash + Eq {
    hash_map: HashMap<T, ()>,
}

impl<T> HashSet<T> where T: Hash + Eq {
    ///
    pub fn new() -> HashSet<T> {
        HashSet { hash_map: HashMap::new() }
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
            None => false
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
            None => false
        }
    }
    /// Creates an iterator that yields immutable reference of each element
    /// in arbitrary order.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.hash_map.iter()
            .map(|(k, _)| k)
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
        Difference { iter: self.iter(), other }
    }
}

impl<T> FromIterator<T> for HashSet<T>
    where T: Hash + Eq
{
    fn from_iter<I>(iter: I) -> Self
        where I: IntoIterator<Item = T>
    {
        let mut s = Self::new();
        iter.into_iter().for_each(|i| { s.insert(i); });
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
        assert_eq!(ok, false, "Attempting to insert present value returns false");
        assert_eq!(s.len(), 2, "Certain value can only be inserted to set once");
    }

    #[test]
    fn contains() {
        let mut s1: HashSet<&str> = HashSet::new();
        s1.insert("cat");
        assert_eq!(s1.contains("cat"), true, "contains() returns true for present value");
        assert_eq!(s1.contains("dog"), false, "contains() returns false for absent value");

        let mut s2: HashSet<String> = HashSet::new();
        s2.insert("cat".to_string());
        assert_eq!(s2.contains(&"cat".to_string()), true, "Can query with String");
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
        assert_eq!(ok, false, "Trying to remove non-existing value returns false");

        let mut s2: HashSet<String> = HashSet::new();
        s2.insert("cat".to_string());
        s2.insert("dog".to_string());
        assert!(s2.remove(&"cat".to_string()), "Can remove with String");
        assert!(!s2.contains("cat"), "Successfully removed value with String");
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
        assert_eq!(union.contains(&"cat"), true, "union of s1 and s2 contains cat (from s1)");
        assert_eq!(union.contains(&"dog"), true, "union of s1 and s2 contains dog (from s1)");
        assert_eq!(union.contains(&"rat"), true, "union of s1 and s2 contains rat (from s2)");

        // // Also works with HashSet<String>
        // let mut s1: HashSet<String> = HashSet::new();
        // s1.insert("cat".to_string());
        // s1.insert("dog".to_string());

        // let mut s2: HashSet<String> = HashSet::new();
        // s2.insert("rat".to_string());

        // let union = s1.union(&s2).collect();
        // assert!(union.contains("cat"));
        // assert!(union.contains("dog"));
        // assert!(union.contains("rat"));

        // TODO: Overload the '&' operator!
        // let union = s1 & s2;
        // assert!(union.contains("cat"));
        // assert!(union.contains("dog"));
        // assert!(union.contains("rat"));
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
        assert_eq!(difference.contains(&"dog"), true, "dog is in s1 but not in s2, therefore included in difference");
        assert_eq!(difference.contains(&"cat"), false, "cat is in both s1 and s2, therefore not included in difference");
        assert_eq!(difference.contains(&"rat"), false, "rat is from s2, therefore not included in difference");
    }
}
