use std::fmt::{Debug, Display, Formatter};
use std::marker;

use rand::Rng;
use std::ptr::null_mut;

/// The maximum height for skip node.
const MAX_HEIGHT: usize = 12;

type SkipTrack<K, V> = [Option<*mut SkipNode<K, V>>; MAX_HEIGHT];

/// A skip list implementation by **unsafe Rust**. **mulit-thread unsafe**, Copy of Leveldb skip list.
///
/// Reference:
/// [LevelDB](https://github.com/google/leveldb)
///
// ANCHOR: struct
pub struct SkipList<K, V>
where
    K: Default,
    V: Default,
{
    head: *mut SkipNode<K, V>,
    rng: rand::rngs::ThreadRng,
    len: usize,
}
// ANCHOR_END: struct

impl<K: Ord + Default, V: Default> SkipList<K, V> {
    fn random_height(&mut self) -> usize {
        // 超几何分布打的表，参考leveldb
        // f(n) = p^n(1-p), p=0.25
        const RAND_TABLE: [u32; 12] = [
            0u32,
            0xBFFFFFFFu32,
            0xEFFFFFFEu32,
            0xFBFFFFFDu32,
            0xFEFFFFFCu32,
            0xFFBFFFFBu32,
            0xFFEFFFFAu32,
            0xFFFBFFF9u32,
            0xFFFEFFF8u32,
            0xFFFFBFF7u32,
            0xFFFFEFF6u32,
            0xFFFFFBF5u32,
        ];
        let mut level: usize = 0;
        let rand_num = self.rng.gen::<u32>();
        while level < RAND_TABLE.len() && rand_num > RAND_TABLE[level] {
            level += 1;
        }
        level
    }
    /// Creates an empty skip list.
    pub fn new() -> Self {
        SkipList {
            head: Box::into_raw(Box::new(SkipNode {
                entry: Default::default(),
                next_by_height: [None; MAX_HEIGHT],
            })),
            rng: rand::thread_rng(),
            len: 0,
        }
    }

    fn find(
        &self,
        mut ptr: *mut SkipNode<K, V>,
        level: usize,
        key: &K,
    ) -> (*mut SkipNode<K, V>, bool) {
        unsafe {
            while let Some(next_node) = (*ptr).next_by_height[level] {
                if (*next_node).entry.key > *key {
                    break;
                }
                ptr = next_node;
            }
            (ptr, (*ptr).entry.key == *key)
        }
    }
    /// Gets a reference to the value under the given key.
    pub fn get(&self, key: &K) -> Option<&V> {
        let mut point = self.head;
        for level in (0..MAX_HEIGHT).rev() {
            let (ptr, is_found) = self.find(point, level, key);
            if is_found {
                return Some(unsafe { &(*ptr).entry.value });
            }
            point = ptr;
        }
        None
    }
    /// Returns the number of elements in the skip list.
    pub fn len(&self) -> usize {
        self.len
    }
    /// Gets a mutable reference to the value under the given key.
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        let mut point = self.head;
        for level in (0..MAX_HEIGHT).rev() {
            let (ptr, is_found) = self.find(point, level, key);
            if is_found {
                return Some(unsafe { &mut (*ptr).entry.value });
            }
            point = ptr;
        }
        None
    }
    /// Puts a key-value pair into the list. Replaces previous value if the same key exists.
    /// Steps are described as following:
    /// 1. search for the closest skip node by key.
    /// 2. if searched same key. replace and return.
    /// 3. else, create a new skip node and insert to proper location.
    pub fn insert(&mut self, key: K, value: V) {
        let jump_height: usize = self.random_height();
        let mut cached = [null_mut(); MAX_HEIGHT];
        let mut start_point = self.head;
        for level in (0..MAX_HEIGHT).rev() {
            let (ptr, is_found) = self.find(start_point, level, &key);
            if is_found {
                unsafe { (*ptr).entry = Entry { key, value } };
                return;
            }
            start_point = ptr;
            cached[level] = ptr;
        }
        let new_node_ptr = Box::into_raw(Box::new(SkipNode {
            entry: Entry { key, value },
            next_by_height: [None; MAX_HEIGHT],
        }));
        cached
            .iter()
            .enumerate()
            .take(jump_height)
            .for_each(|(i, &skip_node)| unsafe {
                (*new_node_ptr).next_by_height[i] = (*skip_node).next_by_height[i];
                (*skip_node).next_by_height[i].replace(new_node_ptr);
            });
        self.len += 1;
    }
    /// Removes the entry from the list.
    // pub fn remove(&mut self, key: &K) -> Option<Entry<K, V>> {
    //     let mut cached: Vec<*mut SkipNode<K, V>> = Vec::new();
    //     let mut start_point = self.head;
    //     for level in (0..MAX_HEIGHT).rev() {
    //         unsafe {
    //             while let Some(next_node) = (*start_point).next_by_height[level] {
    //                 if (*next_node).entry.key == *key {

    //                 }
    //             }
    //         }
    //     }
    //     None;
    // }
    /// Remove the first entry under key order.
    pub fn pop_first(&mut self) -> Option<Entry<K, V>> {
        let head_node = unsafe { &mut *self.head };
        head_node.next_by_height[0].take().map(|first_node_ptr| {
            let mut first_node = unsafe { Box::from_raw(first_node_ptr) };
            for (level, transmit_ptr) in first_node.next_by_height.iter_mut().enumerate() {
                head_node.next_by_height[level] = transmit_ptr.take();
            }
            self.len -= 1;
            first_node.entry
        })
    }
    /// Creates a consuming iterator yielding key-value tuple in specifi order.
    pub fn into_iter(self) -> IntoIterator<K, V> {
        IntoIterator { list: self }
    }
    /// Creates an anonymous iterator yields the key in specifi order.
    pub fn keys(&self) -> Box<dyn Iterator<Item = &K> + '_> {
        Box::new(self.iter().map(|(key, _)| key)) as Box<dyn Iterator<Item = &K>>
    }
    /// Creates an anonymous iterator yields the values in specifi order.
    pub fn values(&self) -> Box<dyn Iterator<Item = &V> + '_> {
        Box::new(self.iter().map(|(_, value)| value)) as Box<dyn Iterator<Item = &V>>
    }
    /// Creates an anonymous iterator yields the values in range [key1, key2).
    pub fn range(&self, key1: K, key2: K) -> Box<dyn Iterator<Item = (&K, &V)> + '_> {
        Box::new(
            self.iter()
                .filter(move |(key, _)| (*key >= &key1) && (*key < &key2)),
        )
    }
    /// Indiates whether the skiplist is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    /// Gets an iterator over the list, sorted by key.
    pub fn iter(&self) -> Iter<K, V> {
        Iter {
            ptr: unsafe { (*self.head).next_by_height[0] },
            _marker: Default::default(),
        }
    }
    /// Gets an mutable iterator over the list, sorted by key.
    pub fn iter_mut(&mut self) -> MutIterator<K, V> {
        MutIterator {
            ptr: unsafe { (*self.head).next_by_height[0] },
            _marker: Default::default(),
        }
    }
}

impl<K, V> Display for SkipList<K, V>
where
    K: Default,
    V: Default,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "SkipList:\n {:?}", self.head)
    }
}

impl<K, V> Drop for SkipList<K, V>
where
    K: Default,
    V: Default,
{
    fn drop(&mut self) {
        let mut ptr = self.head;
        unsafe {
            while let Some(next_ptr) = (*ptr).next_by_height[0].take() {
                Box::from_raw(ptr);
                ptr = next_ptr;
            }
            Box::from_raw(ptr)
        };
    }
}

pub struct MutIterator<'a, K, V>
where
    K: Default,
    V: Default,
{
    ptr: Option<*mut SkipNode<K, V>>,
    _marker: marker::PhantomData<&'a K>,
}

pub struct Iter<'a, K, V>
where
    K: Default,
    V: Default,
{
    ptr: Option<*mut SkipNode<K, V>>,
    _marker: marker::PhantomData<&'a K>,
}

pub struct IntoIterator<K, V>
where
    K: Default,
    V: Default,
{
    list: SkipList<K, V>,
}

impl<'a, K, V: 'a> Iterator for MutIterator<'a, K, V>
where
    K: Default,
    V: Default,
{
    type Item = (&'a mut K, &'a mut V);
    fn next(&mut self) -> Option<Self::Item> {
        self.ptr.map(|node| unsafe {
            self.ptr = (*node).next_by_height[0];
            (&mut (*node).entry.key, &mut (*node).entry.value)
        })
    }
}

impl<'a, K, V: 'a> Iterator for Iter<'a, K, V>
where
    K: Default,
    V: Default,
{
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.ptr.map(|node| unsafe {
            self.ptr = (*node).next_by_height[0];
            (&(*node).entry.key, &(*node).entry.value)
        })
    }
}

impl<K, V> Iterator for IntoIterator<K, V>
where
    K: Ord + Default,
    V: Default,
{
    type Item = (K, V);
    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop_first().map(Entry::into_tuple)
    }
}

#[derive(Default)]
pub struct Entry<K, V> {
    key: K,
    value: V,
}

impl<K, V> Entry<K, V> {
    pub fn into_tuple(self) -> (K, V) {
        (self.key, self.value)
    }
}

impl<K, V> Display for Entry<K, V>
where
    K: Ord + Default + Debug,
    V: Default + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {:?}", self.key, self.value)
    }
}

struct SkipNode<K, V> {
    entry: Entry<K, V>,
    next_by_height: SkipTrack<K, V>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::RngCore;
    use std::cmp::Ordering;
    use std::collections::BTreeMap;
    use std::time::Instant;

    fn create_testee() -> SkipList<String, i32> {
        let mut list = SkipList::new();
        let test_case = [("c", 4i32), ("e", 5i32), ("a", 3), ("a", 8)];
        test_case
            .iter()
            .for_each(|&(k, v)| list.insert(k.to_owned(), v));
        list
    }

    #[test]
    fn insert() {
        let list = create_testee();
        let res: Vec<(String, i32)> = list.into_iter().collect();
        assert_eq!(
            vec![
                ("a".to_owned(), 8),
                ("c".to_owned(), 4),
                ("e".to_owned(), 5)
            ],
            res
        );
    }

    #[test]
    fn insert_get() {
        let mut list = create_testee();
        assert_eq!(list.get(&"c".to_owned()), Some(&4));
        assert_eq!(list.get(&"e".to_owned()), Some(&5));
        assert_eq!(list.get(&"x".to_owned()), None);
        list.insert("x".to_owned(), 100);
        assert_eq!(list.get(&"x".to_owned()), Some(&100));
    }

    #[test]
    fn pop_and_into_iter() {
        let mut list = SkipList::new();
        let test_case = [("c", 4), ("e", 5), ("a", 3), ("a", 8)];
        test_case.iter().for_each(|&(k, v)| list.insert(k, v));
        assert_eq!(list.len(), 3);
        assert_eq!(
            list.pop_first().map(|x| Entry::into_tuple(x)),
            Some(("a", 8))
        );
        assert_eq!(
            list.pop_first().map(|x| Entry::into_tuple(x)),
            Some(("c", 4))
        );
        assert_eq!(
            list.pop_first().map(|x| Entry::into_tuple(x)),
            Some(("e", 5))
        );
        assert_eq!(list.pop_first().map(|x| Entry::into_tuple(x)), None);
        assert_eq!(list.pop_first().map(|x| Entry::into_tuple(x)), None);
        assert_eq!(list.len(), 0);
        test_case.iter().for_each(|&(k, v)| list.insert(k, v));
        assert_eq!(list.len(), 3);
        assert_eq!(
            list.into_iter().collect::<Vec<(&str, i32)>>(),
            vec![("a", 8), ("c", 4), ("e", 5)]
        );
    }

    #[test]
    fn test_fn_keys_values() {
        let list = create_testee();
        assert_eq!(
            vec![&"a".to_owned(), &"c".to_owned(), &"e".to_owned()],
            list.keys().collect::<Vec<&String>>()
        );
        assert_eq!(vec![&8, &4, &5], list.values().collect::<Vec<&i32>>());
    }

    #[test]
    fn test_get_mut() {
        let mut list = create_testee();
        let mut ele = list.get_mut(&"a".to_owned()).unwrap();
        *ele = 114514;
        assert_eq!(*list.get_mut(&"a".to_owned()).unwrap(), 114514);
    }

    #[test]
    fn great_many_inserts_gets() {
        let mut list = SkipList::new();
        let mut btree_map = BTreeMap::new();
        let mut rng = rand::thread_rng();
        let mut b_map_cost = Vec::new();
        let mut skip_list_cost = Vec::new();
        (0..70000)
            .map(|_| (rng.next_u64(), rng.next_u64()))
            .for_each(|(k, v)| {
                let t = Instant::now();
                list.insert(k, v);
                skip_list_cost.push(t.elapsed().as_nanos());
                let t = Instant::now();
                btree_map.insert(k, v);
                b_map_cost.push(t.elapsed().as_nanos());
            });
        assert_eq!(btree_map.iter().cmp(list.iter()), Ordering::Equal);
        assert_eq!(btree_map.len(), list.len());
        let skip_list_mean_cost =
            skip_list_cost.iter().sum::<u128>() / skip_list_cost.len() as u128;
        let btree_map_mean_cost = b_map_cost.iter().sum::<u128>() / b_map_cost.len() as u128;
        println!(
            "SkipList time cost: {} ns, BTreeMap cost {} ns",
            skip_list_mean_cost, btree_map_mean_cost
        );
        assert!(
            skip_list_mean_cost * 2 > btree_map_mean_cost,
            "The performance of SkipList is too bad. SkipList:{} ns, BTreeMap{} ns",
            skip_list_mean_cost,
            btree_map_mean_cost
        );
    }
}
