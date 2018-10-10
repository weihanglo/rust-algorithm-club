use super::hash_map::HashMap;
use std::hash::Hash;
use std::borrow::Borrow;

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
}