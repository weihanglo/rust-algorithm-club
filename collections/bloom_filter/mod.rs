use std::collections::hash_map::{DefaultHasher, RandomState};
use std::hash::{BuildHasher, Hash, Hasher};
use std::marker::PhantomData;

/// A space efficient probablistic data structures offering an approximate
/// containment test with only false positive error.
///
/// The false positive error probability _ε_ of a Bloom filter is defined as
/// the probability that a Bloom filter claims an element is contained in it 
/// but actually not.
///
/// The count of hash functions denoted by _k_ indicates thant _k_ different
/// hash functions that map _k_ position on the bit array. Typically _k_ is a 
/// small constant depends on error probability _ε_.
///
/// The underlying container is a bit array of _m_ bits, where the optimal _m_
/// is proportional to count of hash functions _k_.
///
/// Cheat sheet:
///
/// - _m_: total bits (memory usage)
/// - _n_: expected number of input elements (cardinality)
/// - _k_: number of hash functions counted for each input
/// - _ε_: expected false positive error probability
///
/// References:
///
/// - [Google Guava: BloomFilter][1]
/// - [Onat: Let's implement a Bloom Filter][2]
///
/// [1]: https://github.com/google/guava/blob/v29.0/guava/src/com/google/common/hash/BloomFilter.java
/// [2]: https://onatm.dev/2020/08/10/let-s-implement-a-bloom-filter/
pub struct BloomFilter<T: ?Sized> {
    /// The bit array of _m_ bits that stores existence information of elements.
    bits: Vec<bool>,
    /// Count of hash functions. Denoted by _k_.
    hash_fn_count: usize,
    /// The hashers that do real works. See [Less Hashing, Same Performance:Building a Better Bloom Filter][1]
    /// to figure out why two-hashers strategy would not significantly deteriorate
    /// the performance of a Bloom filter.
    ///
    /// [1]: https://www.eecs.harvard.edu/~michaelm/postscripts/rsa2008.pdf
    hashers: [DefaultHasher; 2],
    _phantom: PhantomData<T>,
}

impl<T: ?Sized> BloomFilter<T> {
    /// Creates an empty Bloom filter with desired capacity and error rate.
    ///
    /// This constructor would give an optimal size for bit array based on
    /// provided `capacity` and `err_rate`.
    ///
    /// # Parameters
    ///
    /// * `capacity` - Expected size of elements will put in.
    /// * `err_rate` - False positive error probability.
    pub fn new(capacity: usize, err_rate: f64) -> Self {
        let bits_count = Self::optimal_bits_count(capacity, err_rate);
        let hash_fn_count = Self::optimal_hashers_count(err_rate);
        let hashers = [
            RandomState::new().build_hasher(),
            RandomState::new().build_hasher(),
        ];

        Self {
            bits: vec![false; bits_count],
            hash_fn_count,
            hashers,
            _phantom: PhantomData,
        }
    }

    /// Inserts an element into the container.
    ///
    /// This function simulates multiple hashers with only two hashers using
    /// the following formula:
    ///
    /// > g_i(x) = h1(x) + i * h2(x)
    ///
    /// # Parameters
    ///
    /// * `elem` - Element to be inserted.
    ///
    /// # Complexity
    ///
    /// Linear in the size of `hash_fn_count` _k_.
    pub fn insert(&mut self, elem: &T)
    where
        T: Hash,
    {
        // g_i(x) = h1(x) + i * h2(x)
        let hashes = self.make_hash(elem);
        for fn_i in 0..self.hash_fn_count {
            let index = self.get_index(hashes, fn_i as u64);
            self.bits[index] = true;
        }
    }

    /// Returns whether an element is present in the container.
    ///
    /// # Parameters
    ///
    /// * `elem` - Element to be checked whether is in the container.
    ///
    /// # Complexity
    ///
    /// Linear in the size of `hash_fn_count` _k_.
    pub fn contains(&self, elem: &T) -> bool
    where
        T: Hash,
    {
        let hashes = self.make_hash(elem);
        (0..self.hash_fn_count).all(|fn_i| {
            let index = self.get_index(hashes, fn_i as u64);
            self.bits[index]
        })
    }

    /// Gets index of the bit array for a single hash iteration.
    ///
    /// As a part of multiple hashers simulation for this formula:
    ///
    /// > g_i(x) = h1(x) + i * h2(x)
    ///
    /// This function calculate the right hand side of the formula.
    ///
    /// Note that the usage fo `wrapping_` is acceptable here for a hash
    /// algorithm to get a valid slot.
    fn get_index(&self, (h1, h2): (u64, u64), fn_i: u64) -> usize {
        (h1.wrapping_add(fn_i.wrapping_mul(h2)) % self.bits.len() as u64) as usize
    }

    /// Hashes the element.
    ///
    /// As a part of multiple hashers simulation for this formula:
    ///
    /// > g_i(x) = h1(x) + i * h2(x)
    ///
    /// This function do the actual `hash` work with two independant hashers, 
    /// returing both h1(x) and h2(x) within a tuple.
    fn make_hash(&self, elem: &T) -> (u64, u64)
    where
        T: Hash,
    {
        let hasher1 = &mut self.hashers[0].clone();
        let hasher2 = &mut self.hashers[1].clone();

        elem.hash(hasher1);
        elem.hash(hasher2);

        (hasher1.finish(), hasher2.finish())
    }

    /// m = -1 * (n * ln ε) / (ln 2)^2
    ///
    /// See [Wikipedia: Bloom filter][1].
    ///
    /// [1]: https://en.wikipedia.org/wiki/Bloom_filter#Optimal_number_of_hash_functions
    fn optimal_bits_count(capacity: usize, err_rate: f64) -> usize {
        let ln_2_2 = std::f64::consts::LN_2.powf(2f64);
        (-1f64 * capacity as f64 * err_rate.ln() / ln_2_2).ceil() as usize
    }

    /// k = -log_2 ε
    ///
    /// See [Wikipedia: Bloom filter][1].
    ///
    /// [1]: https://en.wikipedia.org/wiki/Bloom_filter#Optimal_number_of_hash_functions
    fn optimal_hashers_count(err_rate: f64) -> usize {
        (-1f64 * err_rate.log2()).ceil() as usize
    }
}

#[cfg(test)]
mod classic {
    use super::BloomFilter;

    #[test]
    fn insert() {
        let mut bf = BloomFilter::new(100, 0.01);
        (0..20).for_each(|i| bf.insert(&i));
        (0..20).for_each(|i| assert!(bf.contains(&i)));
    }

    #[test]
    fn contains() {
        let mut bf = BloomFilter::new(100, 0.1);
        assert!(!bf.contains("1"));
        bf.insert("1");
        assert!(bf.contains("1"));
    }

    #[test]
    fn err_rate_100() {
        let bf = BloomFilter::new(100, 1.0);
        // Test correctness of optimal formula.
        assert_eq!(bf.bits.len(), 0);
        // Always positive
        assert!(bf.contains("1"));
        assert!(bf.contains("2"));
        assert!(bf.contains("3"));
    }

    #[test]
    fn get_one_slots_bf() {
        let mut bf = BloomFilter::new(1, 0.8);
        // Test correctness of optimal formula.
        assert_eq!(bf.bits.len(), 1);
        assert!(!bf.contains("1"));
        bf.insert("1");
        // Now all slots are occupied.
        assert!(bf.contains("2"));
        assert!(bf.contains("3"));
    }

    #[test]
    fn is_a_generics_container() {
        let mut bf = BloomFilter::new(100, 0.1);
        bf.insert("1");
        assert!(bf.contains("1"));
        let mut bf = BloomFilter::new(100, 0.1);
        bf.insert(&1);
        assert!(bf.contains(&1));
        let mut bf = BloomFilter::new(100, 0.1);
        bf.insert(&'1');
        assert!(bf.contains(&'1'));

        #[derive(Hash)]
        struct S;
        let mut bf = BloomFilter::new(100, 0.1);
        let s = S;
        assert!(!bf.contains(&s));
        bf.insert(&s);
        assert!(bf.contains(&s));
    }
}
