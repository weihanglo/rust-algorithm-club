use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::borrow::Borrow;
use std::mem;

/// A hash map implemented with separate chaining collision resolution strategy.
///
/// This implementation is focused on hash map functionalities, so we choose to
/// adopt Rust `DefaultHasher` to avoid avalanche of details, and vectors as
/// the underlying data structure for separate chaining method.
///
/// The interface is a simplified version of Rust `HashMap`.
///
/// References:
///
/// - [Rust Standard Library: std::collections::HashMap][1]
/// - [C++ Container Library: std::unordered_map][2]
///
/// [1]: https://doc.rust-lang.org/stable/std/collections/struct.HashMap.html
/// [2]: https://en.cppreference.com/w/cpp/container/unordered_map
pub struct HashMap<K, V> where K: Hash + Eq {
    buckets: Vec<Bucket<K, V>>,
    len: usize,
}

/// Internal container to store collided elements.
type Bucket<K, V> = Vec<(K, V)>;

/// Default load factor.
const LOAD_FACTOR: f64 = 0.75;

/// Computes hash for a given key and modulus.
fn make_hash<X>(x: &X, len: usize) -> usize
    where X: Hash + ?Sized,
{
    let mut hasher = DefaultHasher::new();
    x.hash(&mut hasher);
    hasher.finish() as usize % len
}

impl<K, V> HashMap<K, V> where K: Hash + Eq {
    /// Creates an empty map with capacity 0.
    ///
    /// The allocation is triggered at the first insertion occurs.
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a map with a given capacity as the number of underlying buckets.
    ///
    /// # Parameters
    ///
    /// * `cap`: The number of bucket in the map.
    pub fn with_capacity(cap: usize) -> Self {
        let mut buckets: Vec<Bucket<K, V>> =  Vec::with_capacity(cap);
        for _ in 0..cap {
            buckets.push(Bucket::new());
        }
        Self { buckets, len: 0 }
    }

    /// Gets a reference to the value under the specified key.
    ///
    /// We use Q here to accept any type that K can be borrowed as. For example,
    /// given a HashMap m using String as key, both m.get(&String) and m.get(&str)
    /// would work because String can be borrow as &str. The same technique is
    /// applied for `get_mut` and `remove`.
    ///
    /// Learn more about Borrow trait:
    ///
    /// - [Trait std::borrow:Borrow][1]
    /// - [TRPL 1st edition: Borrow and AsRef][2]
    ///
    /// # Complexity
    ///
    /// Constant (amortized).
    ///
    /// [1]: https://doc.rust-lang.org/stable/std/borrow/trait.Borrow.html
    /// [2]: https://doc.rust-lang.org/stable/book/first-edition/borrow-and-asref.html
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
        where
            K: Borrow<Q>,
            Q: Hash + Eq + ?Sized
    {
        let index = self.make_hash(key);
        self.buckets.get(index).and_then(|bucket|
            bucket.iter()
                .find(|(k, _)| key == k.borrow())
                .map(|(_, v)| v)
        )
    }

    /// Gets a mutable reference to the value under the specified key.
    ///
    /// # Complexity
    ///
    /// Constant (amortized).
    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
        where
            K: Borrow<Q>,
            Q: Hash + Eq + ?Sized
    {
        let index = self.make_hash(key);
        self.buckets.get_mut(index).and_then(|bucket|
            bucket.iter_mut()
                .find(|(k, _)| key == k.borrow())
                .map(|(_, v)| v)
        )
    }

    /// Inserts key-value pair into the map. Replaces previous value if
    /// the same key exists at the same index.
    ///
    /// Returns the old value if the key presents. Otherwise returns `None`.
    ///
    /// Steps are described as following:
    ///
    /// 1. Try to resize hashmap to ensure an appropriate load factor.
    /// 2. Compute hash of the key to get the inner bucket under certain index.
    /// 3. Find if there is already a pair with identical key.
    ///     1. If yes, substitute for it.
    ///     2. Else, push new value into the bucket.
    ///
    /// # Parameters
    ///
    /// * `key` - Key of the pair to insert.
    /// * `value` - Value of the pair to insert.
    ///
    /// # Complexity
    ///
    /// Constant (amortized).
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.try_resize();
        let index = self.make_hash(&key);
        self.buckets.get_mut(index).and_then(|bucket|
            match bucket.iter_mut().find(|(k, _)| *k == key) {
                Some((_ , v)) =>  Some(mem::replace(v, value)),
                None => {
                    bucket.push((key , value));
                    None
                }
            }
        ).or_else(|| { //  Length increase by one.
            self.len += 1;
            None
        })
    }

    /// Removes a pair with specified key.
    ///
    /// The caveat is that ordering in the bucket cannot be preserved due to
    /// the removal using `swap_remove` to ensure O(1) deletion.
    ///
    /// # Parameters
    ///
    /// * `key` - Key of the pair to remove.
    ///
    /// # Complexity
    ///
    /// Constant. This operation won't shrink to fit automatically.
    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
        where
            K: Borrow<Q>,
            Q: Hash + Eq + ?Sized
    {
        let index = self.make_hash(key);
        self.buckets.get_mut(index).and_then(|bucket| {
            bucket.iter_mut()
                .position(|(k, _)| key == (*k).borrow())
                .map(|index| bucket.swap_remove(index).1) // Extract the pair.
        }).map(|v| {
            self.len -= 1; // Length decreases by one.
            v
        })
    }

    /// Removes all key-value pairs but keeps the allocated memory for reuse.
    ///
    /// # Complexity
    ///
    /// Linear in the size of the container.
    pub fn clear(&mut self) {
        for bucket in &mut self.buckets {
            *bucket = Bucket::new();
        }
        self.len = 0;
    }

    ///	Checks whether the container is empty.
    ///
    /// # Complexity
    ///
    /// Constant.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Gets the number of key-value pairs in the container.
    ///
    /// # Complexity
    ///
    /// Constant.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Gets the number of underlying buckets.
    ///
    /// # Complexity
    ///
    /// Constant.
    pub fn bucket_count(&self) -> usize {
        self.buckets.len()
    }

    /// Computes hash for a given key.
    ///
    /// This is an internal function which calls a private module function.
    fn make_hash<X: Hash + ?Sized>(&self, x: &X) -> usize {
        make_hash(x, self.bucket_count())
    }

    /// Tries to resize the capacity if the usage is over the threshold. The
    /// threshold (load factor) of current hash policy is 75%.
    ///
    /// The are two situation may occur in this function. 1) The capacity is
    /// zero, and 2) the capacity reaches the limit. The reason to handle the
    /// first situation here is to delay the actual allocation timing for
    /// conforming to the lazy allocation pattern of Rust philosophy.
    fn try_resize(&mut self) {
        let entry_count = self.len();
        let capacity = self.bucket_count();

        // Initialization.
        if capacity == 0 {
            self.buckets.push(Bucket::new());
            return
        }

        if entry_count as f64 / capacity as f64 > LOAD_FACTOR {
            // Resize. Rehash. Reallocate!
            let mut new_map = Self::with_capacity(capacity << 1);
            self.buckets.iter_mut()
                .flat_map(|bucket| mem::replace(bucket, vec![]))
                .for_each(|(k, v)| { new_map.insert(k, v); });
            *self = new_map;
        }
    }
}


impl<K, V> Default for HashMap<K, V>
    where K: Hash + Eq
{
    fn default() -> Self {
        Self { buckets: Vec::<Bucket<K, V>>::new(), len: 0 }
    }
}

#[cfg(test)]
mod separate_chaining {
    use super::HashMap;

    type Map<'a> = HashMap<&'a str, &'a str>;

    #[test]
    fn basics() {
        let m = Map::new();
        assert_eq!(m.len(), 0);
        assert!(m.is_empty());
    }

    #[test]
    fn insert() {
        let mut m = Map::new();

        let ret = m.insert("cat", "cute");
        assert_eq!(ret, None);
        assert_eq!(m.len(), 1);

        m.insert("dog", "loyal");
        assert_eq!(m.len(), 2);

        // Inserting the same key must replace and return previous entry.
        let ret = m.insert("cat", "fat");
        assert_eq!(m.len(), 2);
        assert_eq!(ret, Some("cute"));

        m.insert("rat", "lovely");
        assert_eq!(m.len(), 3);
    }

    #[test]
    fn remove() {
        let mut m = Map::new();

        m.insert("cat", "cute");
        m.insert("dog", "loyal");
        m.insert("rat", "lovely");

        // Test remove
        m.remove(&"cat");
        assert_eq!(m.len(), 2);

        // No effect
        m.remove(&"cat");
        assert_eq!(m.len(), 2);

        m.remove(&"dog");
        assert_eq!(m.len(), 1);

        // No effect
        m.remove(&"mice");
        assert_eq!(m.len(), 1);

        m.remove(&"rat");
        assert_eq!(m.len(), 0);


        // Use String as key
        let mut m = HashMap::new();
        m.insert("cat".to_string(), "cute");
        m.insert("dog".to_string(), "loyal");
        m.insert("rat".to_string(), "lovely");

        // Query with &String
        m.remove(&"cat".to_string());
        assert_eq!(m.len(), 2);

        // Query with &str also work
        m.remove("dog");
        assert_eq!(m.len(), 1);
    }

    #[test]
    fn get() {
        let mut m = Map::new();

        m.insert("cat", "cute");
        m.insert("dog", "loyal");

        assert_eq!(m.get(&"cat"), Some(&"cute"));
        assert_eq!(m.get(&"dog"), Some(&"loyal"));
        assert_eq!(m.get(&"rat"), None);


        // Use String as key (HashMap<String, &str>)
        let mut m = HashMap::new();
        m.insert("cat".to_string(), "cute");
        m.insert("dog".to_string(), "loyal");

        // Query with &String
        assert_eq!(m.get(&"cat".to_string()), Some(&"cute"));
        // Query with &str also work
        assert_eq!(m.get("dog"), Some(&"loyal"));
    }


    #[test]
    fn get_mut() {
        let mut m = Map::new();
        m.insert("cat", "cute");
        m.insert("dog", "loyal");

        assert_eq!(m.get_mut(&"cat"), Some(&mut "cute"));
        assert_eq!(m.get_mut(&"dog"), Some(&mut "loyal"));
        assert_eq!(m.get_mut(&"rat"), None);


        // Use String as key
        let mut m = HashMap::new();
        m.insert("cat".to_string(), "cute");
        m.insert("dog".to_string(), "loyal");

        // Query with &String
        assert_eq!(m.get_mut(&"cat".to_string()), Some(&mut "cute"));
        // Query with &str also work
        assert_eq!(m.get_mut("dog"), Some(&mut "loyal"));
    }

    #[test]
    fn resize() {
        let mut m = Map::new();
        assert_eq!(m.bucket_count(), 0);

        m.insert("cat", "cute");
        assert_eq!(m.len(), 1);
        assert_eq!(m.bucket_count(), 1);

        m.insert("dog", "loyal");
        assert_eq!(m.len(), 2);
        assert_eq!(m.bucket_count(), 2);

        m.insert("rat", "lovely");
        assert_eq!(m.len(), 3);
        assert_eq!(m.bucket_count(), 4);

        m.insert("dragon", "omnipotent");
        assert_eq!(m.len(), 4);
        assert_eq!(m.bucket_count(), 4);

        m.insert("human", "lazy");
        assert_eq!(m.len(), 5);
        assert_eq!(m.bucket_count(), 8);
    }

    #[test]
    fn clear() {
        let mut m = Map::new();
        m.insert("cat", "cute");
        m.insert("dog", "loyal");

        m.clear();
        assert!(m.is_empty());
        assert_eq!(m.len(), 0);
        assert_eq!(m.bucket_count(), 2); // Preserve previous allocation.
    }
}

#[cfg(test)]
// TODO: linear probing method
mod linear_probing {
    #[ignore]
    #[test]
    fn basics() {
    }

    #[ignore]
    #[test]
    fn insert() {
    }

    #[ignore]
    #[test]
    fn remove() {
    }

    #[ignore]
    #[test]
    fn get() {
    }

    #[ignore]
    #[test]
    fn get_mut() {
    }

    #[ignore]
    #[test]
    fn resize() {
    }

    #[ignore]
    #[test]
    fn clear() {
    }
}
